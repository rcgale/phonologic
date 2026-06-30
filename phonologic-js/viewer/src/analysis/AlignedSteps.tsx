import {TranscriptFormatted} from "./TranscriptFormatted";
import {AnalysisStep} from "../services/AnalysisService";
import {useContext} from "react";
import {HoverContext} from "../HoverContext";

type AlignedStepsProps = {
    steps: AnalysisStep[],
    alphabet: string,
    labelLeft: string,
    labelRight: string,
};

export function AlignedSteps({steps, alphabet, labelLeft, labelRight}: AlignedStepsProps) {
    const [hoverIndex] = useContext(HoverContext)
    return (
        <div className="transcript-steps-wrapper">
            <div className="transcript-steps" style={{gridTemplateColumns: `repeat(${steps.length}, auto)`}}>
                <div className="expected">
                    <label>{labelLeft}</label>
                    {steps.map((step, n) =>
                        <span key={n}
                              className={[
                            "step-expected",
                            (step.cost > 0 && "step-error"),
                            (hoverIndex === n && "highlight")
                        ].join(" ")}>
                                <TranscriptFormatted transcript={step.left} alphabet={alphabet} />
                        </span>
                    )}
                </div>
                <div className="actual">
                    <label>{labelRight}</label>
                    {steps.map((step, n) =>
                        <span className={[
                            "step-actual",
                            (step.cost > 0 && "step-error"),
                            (hoverIndex === n && "highlight")
                        ].join(" ")}>
                                <TranscriptFormatted transcript={step.right} alphabet={alphabet} />
                        </span>
                    )}
                </div>
            </div>
        </div>
    );
}
