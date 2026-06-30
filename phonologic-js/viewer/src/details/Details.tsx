import {Features} from "./Features";
import {TranscriptFormatted} from "../analysis/TranscriptFormatted";
import type {AnalysisStep} from "../services/AnalysisService";
import {useContext, useEffect, useState} from "react";
import {Analysis, AnalysisService} from "../services/AnalysisService";
import {FeatureDelta} from "@phonologic/wasm";
import {HoverContext} from "../HoverContext";

type DeltasProps = {
    step: AnalysisStep
}

function Deltas({step}: DeltasProps) {
    const [deltas, setDeltas] = useState<FeatureDelta[]>([]);

    useEffect(() => {
        AnalysisService.getDeltas(step.left, step.right).then(setDeltas);
    }, [step]);

    if (!deltas.length) {
        return <></>;
    }

    return (
        <ul className="feature-collection with-brackets">
            {deltas.map((delta, idx) =>
                <li key={idx}><Features delta={delta} /></li>
            )}
        </ul>
    );
}

type DetailsProps = {
    selectedId: string,
    alphabet: string,
    analysis: Analysis
}

export function Details({selectedId, alphabet, analysis}: DetailsProps) {
    const [_, setHoverIndex] = useContext(HoverContext);
    function stepFeatureCost(step: AnalysisStep) {
        return `${costFormatted(step.cost)} / 24`
    }

    function costFormatted(cost: number) {
        let rounded = Math.round(100 * (cost + Number.EPSILON)) / 100
        return `${rounded}`
    }

    let steps = analysis.features.steps;
    return (
        <div id="detail" style={{visibility: selectedId ? "visible" : "hidden"}}>
            <table className="feature-steps">
                <thead>
                    <tr>
                        <th>Action</th>
                        <th>Cost</th>
                        <th>From</th>
                        <th>To</th>
                        <th>Features</th>
                    </tr>
                </thead>
                <tbody>
                {
                    steps.map((step, n) =>
                        <tr key={n}
                            onMouseOver={() => setHoverIndex(n)}
                            onMouseLeave={() => setHoverIndex(undefined)}
                            className={'action-' + step.action.toLowerCase()}
                        >
                            <td>{step.action}</td>
                            <td>{stepFeatureCost(step)}</td>
                            <td>
                                {(step.left &&
                                    <span>
                                        <TranscriptFormatted
                                            transcript={step.left}
                                            alphabet={alphabet}
                                        />
                                    </span>
                                ) || null}

                            </td>
                            <td>
                                {(step.right &&
                                    <span>
                                        <TranscriptFormatted
                                            transcript={step.right}
                                            alphabet={alphabet}
                                        />
                                    </span>
                                ) || null}

                            </td>
                            <td>
                                <Deltas step={step} />
                            </td>
                        </tr>
                    )
                }
                </tbody>
            </table>
        </div>
    );
}
