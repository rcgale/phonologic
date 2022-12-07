import {Features} from "./Features";
import {TranscriptFormatted} from "../analysis/TranscriptFormatted";
import type {AnalysisStep} from "../services/AnalysisService";
import {Component} from "react";
import {Analysis, AnalysisService} from "../services/AnalysisService";
import {FeatureDelta} from "@phonologic/wasm";

type DetailsProps = {
    selectedId: string,
    alphabet: string,
    analysis: Analysis
}

type DeltasProps = {
    step: AnalysisStep
}

type DeltasState = {
    deltas: FeatureDelta[]
}

class Deltas extends Component<DeltasProps, DeltasState> {
    constructor(props: DeltasProps) {
        super(props);
        this.state = {deltas: []}
    }

    componentDidMount() {
        let step = this.props.step;
        AnalysisService.getDeltas(step.left, step.right).then(deltas => {
            this.setState({deltas: deltas});
        });
    }

    render() {
        if (!this.state.deltas.length) {
            return null;
        }
        return (
            <ul className="feature-collection with-brackets">
                {this.state.deltas.map(delta =>
                    <li><Features delta={delta} /></li>
                )}
            </ul>
        );
    }
}

export class Details extends Component<DetailsProps> {
    stepFeatureCost(step: AnalysisStep) {
        return `${this.costFormatted(step.cost)} / 24`
    }

    costFormatted(cost: number) {
        let rounded = Math.round(100 * (cost + Number.EPSILON)) / 100
        return `${rounded}`
    }

    render() {
        let steps = this.props.analysis.features.steps;
        return (
            <div id="detail" style={{visibility: this.props.selectedId ? "visible" : "hidden"}}>
                <table className="feature-steps">
                    <thead>
                        <th>Action</th>
                        <th>Cost</th>
                        <th>From</th>
                        <th>To</th>
                        <th>Features</th>
                    </thead>
                    <tbody>
                    {
                        steps.map((step, n) =>
                            <tr
                                // onMouseOver="$emit('updateHoverIndex', n)"
                                // onMouseleave="$emit('updateHoverIndex', null)"
                                className={'action-' + step.action.toLowerCase()}
                            >
                                <td>{step.action}</td>
                                <td>{this.stepFeatureCost(step)}</td>
                                <td>
                                    {(step.left &&
                                        <span>
                                            <TranscriptFormatted
                                                transcript={step.left}
                                                alphabet={this.props.alphabet}
                                            />
                                        </span>
                                    ) || null}

                                </td>
                                <td>
                                    {(step.right &&
                                        <span>
                                            <TranscriptFormatted
                                                transcript={step.right}
                                                alphabet={this.props.alphabet}
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
}
