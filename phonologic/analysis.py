import csv
import dataclasses
import itertools
import multiprocessing
import os
import statistics
from argparse import ArgumentParser
from typing import Iterable, Tuple, Union, IO, Dict

import regex

import phonologic
from phonologic import logger
from phonologic._error_analysis import ErrorAnalysis
from phonologic._eval import ParallelAnalyzer


def get_args():
    parser = ArgumentParser()
    parser.add_argument("in_file")
    parser.add_argument("--out-dir", help="Where to write the analysis. If unspecfied, writes in same dir as tsv_files")
    parser.add_argument("--n-jobs", type=int, default=multiprocessing.cpu_count() - 1)
    parser.add_argument("--log-level", default="INFO", choices=("DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"))
    parser.add_argument("--system", default="hayes")
    parser.add_argument("--column-index", default=None, help="Column name for the index/name of the two transcripts")
    parser.add_argument("--column-expected", default=None, help="Column name for reference transcript")
    parser.add_argument("--column-actual", default=None, help="Column name for transcript to compare to reference")
    return parser.parse_args()


def do_asr_evaluation(
        in_files: str,
        out_dir: str,
        system: str,
        column_index: str,
        column_expected: str,
        column_actual: str,
        n_jobs=None,
        log_level="INFO"
):
    if out_dir:
        out_dir = os.path.expanduser(out_dir)
    n_jobs = n_jobs or multiprocessing.cpu_count() - 1
    logger.setLevel(log_level)

    for in_file in in_files:
        current_out_dir = out_dir or os.path.dirname(in_file)

        analysis = analyze_file(in_file, column_index, column_expected, column_actual, system, n_jobs)

        out_base_filename = regex.sub("\.\w+$", "", os.path.basename(in_file))
        out_filename = os.path.join(current_out_dir, f"{out_base_filename}-analysis.json")
        assert os.path.abspath(out_filename) != os.path.abspath(in_file), f"Unexpected filename: {in_file}, add .tsv/.csv extension?"
        analysis.save(out_filename)

        reports = {
            "utterance": aggregate(analysis, "Utterance", key=lambda utterance: utterance),
            "session": aggregate(analysis, "Session", key=lambda utterance: utterance.split("-", 1)[0]),
            "prompt": aggregate(analysis, "Prompt", key=lambda utterance: utterance.split("-", 1)[1]),
        }

        for report_name, report in reports.items():
            report_filename = os.path.join(current_out_dir, f"{out_base_filename}-report-{report_name}.csv")
            write_tsv(report_filename, report)
            for metric in ("PER", "FER"):
                mean = statistics.mean(u[metric] for u in report)
                median = statistics.median(u[metric] for u in report)
                std = statistics.stdev(u[metric] for u in report)
                logger.info(f"{report_name} {metric} mean: {mean:.3f}, median {median:.3f}, std= {std:.3f}")
        logger.info(f"Wrote analysis to {out_filename}")


def analyze_file(filename, column_index, column_expected, column_actual, system_name, n_jobs):
    logger.info(f"Analyzing file {filename}")
    system = phonologic.load(system_name)
    comparison = ComparisonFile.load(
        filename,
        column_index=column_index,
        column_left=column_expected,
        column_right=column_actual
    )
    analyzer = ParallelAnalyzer(system)
    index, expecteds, actuals = zip(*((i, e, a) for i, (e, a) in comparison.rows.items()))
    return analyzer.analyze_parallel(expecteds, actuals, n_jobs, index=index)


def write_tsv(filename, rows):
    if not len(rows):
        logger.warning(f"No rows for report at {filename}, skipping")
        return
    header = tuple(rows[0].keys())
    with open(filename, "w") as f:
        writer = csv.writer(f)
        writer.writerow(header)
        for row in rows:
            assert header == tuple(row.keys())
            writer.writerow(row.values())
    logger.info(f"Wrote report to {filename}")


def aggregate(analysis, key_name, key):
    def aggregate_row(g, items):
        fer = error_rate(item["features"] for item in items)
        per = error_rate(item["phonemes"] for item in items)
        n_phonemes = sum(item["phonemes"].expected_length for item in items)
        per_over_fer = per / fer if fer else ""
        norm_fer_over_per = per / fer / n_phonemes if fer and n_phonemes else ""
        return {
            key_name: g,
            "FER": fer,
            "PER": per,
            "PER/FER": per_over_fer,
            "||PER/FER||": norm_fer_over_per
        }

    grouped = list(
        (g, [analysis.items[u] for u in utts])
        for g, utts in itertools.groupby(sorted(analysis.items, key=key), key=key)
    )
    return [
        aggregate_row(g, items)
        for g, items in grouped
    ]


def error_rate(items: Iterable[ErrorAnalysis]):
    distances, lengths = zip(*((item.distance, item.expected_length) for item in items))
    total_length = sum(lengths) if any(lengths) else 0
    if total_length == 0:
        return float("inf")
    return sum(distances) / total_length


@dataclasses.dataclass
class ComparisonFile:
    filename: str
    labels: Tuple[str, str]
    rows: Dict[str, Tuple[str, str]]

    @classmethod
    def load(
            cls,
            file: Union[str, IO],
            *,
            column_index: str = None,
            column_left: str = None,
            column_right: str = None
    ) -> "ComparisonFile":
        if isinstance(file, str):
            with open(file) as file:
                return cls.load(file, column_index=column_index, column_left=column_left, column_right=column_right)
        dialect_map = {"tsv": csv.excel_tab, "csv": csv.excel}
        dialect = dialect_map.get(file.name.rsplit(".", 1)[-1], csv.excel)

        reader = csv.reader(file, dialect=dialect)
        header = next(reader)
        if len(header) < 2:
            raise ValueError(f"Problem parsing input file {file.name}")

        if len(header) == 2:
            column_left = column_left or header[0]
            column_right = column_right or header[1]
            idx_left = header.index(column_left)
            idx_right = header.index(column_right)
            rows = {
                i: (row[idx_left], row[idx_right])
                for i, row in enumerate(reader)
            }
        else:
            column_index = column_index or header[0]
            column_left = column_left or header[1]
            column_right = column_right or header[2]
            idx_index = header.index(column_index)
            idx_left = header.index(column_left)
            idx_right = header.index(column_right)
            # row_values = zip(*((row[idx_index], row[idx_expected], row[idx_actual]) for row in reader))
            rows = {
                row[idx_index]: (row[idx_left], row[idx_right])
                for row in reader
            }

        return cls(filename=file.name, labels=(column_left, column_right), rows=rows)


def main():
    args = get_args()
    do_asr_evaluation(**vars(args))


if __name__ == '__main__':
    main()