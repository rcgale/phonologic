import {AlignedSteps} from "../analysis/AlignedSteps";
import {Analysis} from "../services/AnalysisService";
import {Component} from "react";
import {ErrorRate} from "../analysis/ErrorRate";
import {Distance} from "../analysis/Distance";

type ResultTableRowProps = {
    onSelect: () => void,
    analysis: Analysis,
    alphabet: string,
    labelLeft: string,
    labelRight: string,
};

export class ResultTableRow extends Component<ResultTableRowProps, {}> {
    render() {
        let analysis = this.props.analysis;
        return (
            <tr>
                <td className="column-utterance-id">
                    <button onClickCapture={() => this.props.onSelect()} className="select-utterance-button">{analysis.id}</button>
                </td>
                <td className="column-transcript">
                    <AlignedSteps
                        steps={analysis.features.steps}
                        alphabet={this.props.alphabet}
                        labelLeft={this.props.labelLeft}
                        labelRight={this.props.labelRight}/>
                </td>
                <td className="column-error-metric">
                    <ErrorRate value={analysis.features.errorRate} />
                </td>
                <td className="column-error-counts">
                    <Distance analysisDetails={analysis.features} />
                </td>
                <td className="column-error-metric">
                    <ErrorRate value={analysis.phonemes.errorRate} />
                </td>
                <td className="column-error-counts">
                    <Distance analysisDetails={analysis.phonemes} />
                </td>
            </tr>
        );
    }
}