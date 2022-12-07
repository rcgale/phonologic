import {Component} from "react";


export class ErrorRate extends Component<{ value: number }> {
    render() {
        return (
            <span>
                {(this.props.value * 100).toFixed(1)}%
            </span>
        );
    }
}