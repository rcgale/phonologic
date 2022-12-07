import {TranscriptDiff} from "./TranscriptDiff";
import {Details} from "./Details";
import {Analysis} from "../services/AnalysisService";
import {Component} from "react";

type SelectedModalProps = {
    deselect: () => void,
    selectedId: string|null,
    analysis: Analysis|null,
    alphabet: string,
    labelLeft: string,
    labelRight: string
}

export class SelectedModal extends Component<SelectedModalProps, {detailHoverIndex: number}> {
    render() {
        return (
            <div id="selected-modal" style={{visibility: this.props.selectedId ? "visible" : "hidden"}}>
                {(this.props.selectedId && this.props.analysis &&
                    <div id="selected-analysis">
                        <button className="close-selected" onClickCapture={() => this.props.deselect()}>✕</button>
                        <h2>{this.props.selectedId}</h2>
                        <TranscriptDiff
                            key={this.props.selectedId}
                            analysis={this.props.analysis}
                            alphabet={this.props.alphabet}
                            // detailHoverIndex={this.props.detailHoverIndex}
                            labelLeft={this.props.labelLeft}
                            labelRight={this.props.labelRight}
                        />
                        {/*<AudioPlayer utteranceId="selectedId" />*/}
                    </div>
                ) || null}
                {(this.props.selectedId && this.props.analysis &&
                    <Details
                        key={this.props.selectedId}
                        selectedId={this.props.selectedId}
                        analysis={this.props.analysis}
                        alphabet={this.props.alphabet}
                        // detailHoverIndex={this.props.detailHoverIndex}
                        // updateHoverIndex={(idx: number) => this.setState({detailHoverIndex: idx})}
                    />
                ) || null}
            </div>
        );
    }
}
