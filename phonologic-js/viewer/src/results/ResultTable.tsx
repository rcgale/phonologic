// import AlignedSteps from "../../components/analysis/AlignedSteps";
import {ResultTableRow} from "./ResultTableRow";
// import ResultTableRowError from "./ResultTableRowError";
import {Analysis, AnalysisCollection, AnalysisException} from "../services/AnalysisService";
import {Component, useState} from "react";

type ResultTableRowErrorProps = {
    analysisException: AnalysisException
}
export function ResultTableRowError({}: ResultTableRowErrorProps) {
    return (
        <div></div>
    );
}

type ResultTableProps = {
    loading: boolean,
    show: (id: string) => void,
    analyses: AnalysisCollection,
    alphabet: string,
    labelLeft: string,
    labelRight: string
};

type ResultTableState = {
    selectedId: string,
    analysis: Analysis
};

export function ResultTable({loading, show, analyses, alphabet, labelLeft, labelRight}: ResultTableProps) {
    const [selectedId, setSelectedId] = useState<string>();
    const [analysis, setAnalysis] = useState<Analysis>();

    return (
        <div>
            {(analyses?.length &&
                <table id="result-table">
                    <thead>
                    <tr className="header-extra">
                        <th className="column-utterance-id">&nbsp;</th>
                        <th className="column-transcript">&nbsp;</th>
                        <th colSpan={2} className="header-features" >Features</th>
                        <th colSpan={2} className="header-phonemes" >Phonemes</th>
                    </tr>

                    <tr className="header-main">
                        <th className="column-utterance-id">Utterance ID</th>
                        <th className="column-transcript">Transcripts</th>
                        <th className="column-error-metric">FER</th>
                        <th className="column-error-counts">Err/Len</th>
                        <th className="column-error-metric">PER</th>
                        <th className="column-error-counts">Err/Len</th>
                    </tr>
                    </thead>
                    <tbody>
                    {analyses.exceptions.map((analysisException, idx) =>
                        <ResultTableRowError
                            key={idx}
                            analysisException={analysisException} />)
                    }

                    {analyses.map(analysis => {
                        return <ResultTableRow
                            // class={{highlight: state.selectedId === analysis.id}}
                            onSelect={() => show(analysis.id)}
                            key={analysis.id}
                            analysis={analysis}
                            alphabet={alphabet}
                            labelLeft={labelLeft}
                            labelRight={labelRight} />

                    })}

                    </tbody>
                </table>
            ) || null}

            {(loading &&
                <div className="loading-container">
                    <div className="loader">&nbsp;</div>
                    Analyzing...
                </div>
            ) || null}
        </div>
    );
}