// import AlignedSteps from "../../components/analysis/AlignedSteps";
import {ResultTableRow} from "./ResultTableRow";
// import ResultTableRowError from "./ResultTableRowError";
import {Analysis, AnalysisCollection, AnalysisException} from "../services/AnalysisService";
import {Component} from "react";

type ResultTableRowErrorProps = {
    analysisException: AnalysisException
}
export class ResultTableRowError extends Component<ResultTableRowErrorProps, {}> {
    render() {
        return (
            <div></div>
        );
    }
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

export class ResultTable extends Component<ResultTableProps, ResultTableState> {
    render() {
        return (
            <div>
                {(this.props.analyses?.length &&
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
                        {this.props.analyses.exceptions.map(analysisException =>
                            <ResultTableRowError
                                analysisException={analysisException} />)
                        }

                        {this.props.analyses.map(analysis => {
                            return <ResultTableRow
                                // class={{highlight: this.state.selectedId === analysis.id}}
                                onSelect={() => this.props.show(analysis.id)}
                                key={analysis.id}
                                analysis={analysis}
                                alphabet={this.props.alphabet}
                                labelLeft={this.props.labelLeft}
                                labelRight={this.props.labelRight} />

                        })}

                        </tbody>
                    </table>
                ) || null}

                {(this.props.loading &&
                    <div className="loading-container">
                        <div className="loader">&nbsp;</div>
                        Analyzing...
                    </div>
                ) || null}
            </div>
        );
    }
}