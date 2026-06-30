import * as React from "react";
import {AlignedSteps} from "../analysis/AlignedSteps";
import {Analysis} from "../services/AnalysisService";
import {ErrorRate} from "../analysis/ErrorRate";
import {Distance} from "../analysis/Distance";
import {Button} from "react-bootstrap";

type ResultTableRowProps = {
    onSelect: () => void,
    analysis: Analysis,
    alphabet: string,
    labelLeft: string,
    labelRight: string,
};

export function ResultTableRow({onSelect, analysis, alphabet, labelLeft, labelRight}: ResultTableRowProps) {
    let {distance, expectedLength} = analysis.features;
    return (
        <tr>
            <td className="column-utterance-id">
                <Button variant="primary" onClickCapture={() => onSelect()} className="select-utterance-button">{analysis.id}</Button>
            </td>
            <td className="column-transcript">
                <AlignedSteps
                    steps={analysis.features.steps}
                    alphabet={alphabet}
                    labelLeft={labelLeft}
                    labelRight={labelRight}/>
            </td>
            <td className="column-error-metric">
                <ErrorRate value={analysis.features.errorRate} />
            </td>
            <td className="column-error-counts">
                <Distance distance={analysis.features.distance} expectedLength={analysis.features.expectedLength} />
            </td>
            <td className="column-error-metric">
                <ErrorRate value={analysis.phonemes.errorRate} />
            </td>
            <td className="column-error-counts">
                <Distance distance={analysis.phonemes.distance} expectedLength={analysis.phonemes.expectedLength} />
            </td>
        </tr>
    );
}