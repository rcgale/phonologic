import {Component} from "react";
import {AnalysisDetails} from "../services/AnalysisService";

export class Distance extends Component<{ analysisDetails: AnalysisDetails }> {
    render() {
        let distance = this.props.analysisDetails.distance;
        let length = this.props.analysisDetails.expectedLength;
        return <span>{distance} / {length}</span>;
    }
}