import multiprocessing
import sys

from tqdm import tqdm

from phonologic import PhonologicalFeatureSystem, ErrorAnalysisDict


class ParallelAnalyzer:
    def __init__(self, system: PhonologicalFeatureSystem):
        self.system = system

    def __call__(self, args):
        id, expected, actual = args
        analysis_phon = self.system.analyze_phoneme_errors(expected, actual)
        analysis_feat = self.system.analyze_feature_errors(expected, actual)
        return id, {
            "features": analysis_feat,
            "phonemes": analysis_phon,
        }

    def analyze_parallel(self, expecteds, actuals, n_jobs, index=None) -> ErrorAnalysisDict:
        expecteds = list(expecteds)  # ensure we have a len()
        if index is None:
            index = range(len(expecteds))
        job_args = zip(index, expecteds, actuals)

        if n_jobs == 1:
            jobs = (self(job) for job in job_args)
        else:
            jobs = multiprocessing.Pool(n_jobs).imap_unordered(self, job_args)

        results = dict(sorted(tqdm(jobs, total=len(expecteds), file=sys.stderr)))
        total_feature_length = sum(a["features"].expected_length for a in results.values())
        fer = sum(a["features"].distance for a in results.values()) / total_feature_length
        total_phoneme_length = sum(a["phonemes"].expected_length for a in results.values())
        per = sum(a["phonemes"].distance for a in results.values()) / total_phoneme_length
        return ErrorAnalysisDict(fer=fer, per=per, items=results)
