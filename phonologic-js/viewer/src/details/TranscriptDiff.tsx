import {AlignedSteps} from "../analysis/AlignedSteps";
import {Analysis} from "../services/AnalysisService";
import {Component} from "react";
import {ErrorRate} from "../analysis/ErrorRate";

type TranscriptDiffProps = {
    // detailHoverIndex: number|null,
    alphabet: string,
    analysis: Analysis,
    labelLeft: string,
    labelRight: string
}

export function TranscriptDiff({alphabet, analysis, labelLeft, labelRight}: TranscriptDiffProps) {
    return (
        <div id="item" v-if="analysis">
                    <AlignedSteps
                        steps={analysis.features.steps}
                        alphabet={alphabet}
                        labelLeft={labelLeft}
                        labelRight={labelRight}
                    />
                    <table id="details-error-rates">
                        <thead>
                            <tr>
                                <th colSpan={2}>Utterance</th>
                            </tr>
                            <tr>
                                <th>FER</th>
                                <th>PER</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><ErrorRate value={analysis.fer} /></td>
                                <td><ErrorRate value={analysis.per} /></td>
                            </tr>
                        </tbody>
                    </table>
            </div>
    );
}