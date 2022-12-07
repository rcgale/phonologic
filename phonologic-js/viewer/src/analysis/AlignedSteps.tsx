import {TranscriptFormatted} from "./TranscriptFormatted";
import {AnalysisStep} from "../services/AnalysisService";
import {Component} from "react";

type AlignedStepsProps = {
    steps: AnalysisStep[],
    alphabet: string,
    labelLeft: string,
    labelRight: string,
};

type AlignedStepsState = {
    detailHoverIndex: number|null
}

export class AlignedSteps extends Component<AlignedStepsProps, AlignedStepsState> {
    constructor(props: AlignedStepsProps) {
        super(props);
        this.state = {
            detailHoverIndex: null
        }
    }
    render() {
        return (
            <div className="transcript-steps-wrapper">
                <div className="transcript-steps" style={{gridTemplateColumns: `repeat(${this.props.steps.length}, auto)`}}>
                    <div className="expected">
                        <label>{this.props.labelLeft}</label>
                        {this.props.steps.map((step, n) =>
                            <span className={[
                                "step-expected",
                                (step.cost > 0 && "step-error"),
                                (this.state.detailHoverIndex === n && "highlight")
                            ].join(" ")}>
                                    <TranscriptFormatted transcript={step.left} alphabet={this.props.alphabet} />
                            </span>
                        )}
                    </div>
                    <div className="actual">
                        <label>{this.props.labelRight}</label>
                        {this.props.steps.map((step, n) =>
                            <span className={[
                                "step-actual",
                                (step.cost > 0 && "step-error"),
                                (this.state.detailHoverIndex === n && "highlight")
                            ].join(" ")}>
                                    <TranscriptFormatted transcript={step.right} alphabet={this.props.alphabet} />
                            </span>
                        )}
                    </div>
                </div>
            </div>
        );
    }
}
