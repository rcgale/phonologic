import {Component} from "react";
import {FeatureDelta} from "@phonologic/wasm";

type FeaturesProps = {
    delta: FeatureDelta
}

export class Features extends Component<FeaturesProps> {
    value(value: number) {
        switch (value) {
            case -1:
                return `–`;
            case +1:
                return `+`;
            case -0.5:
                return `–+`;
            case +0.5:
                return `+–`;
            default:
                return `${value}`;
        }
    }

    cost(delta: FeatureDelta) {
        let name = (delta.left || delta.right).replace(/[+\-0]/, '')
        return `Cost for [${name}]: ${delta.cost}`
    }

    render() {
        let delta = this.props.delta;
        let left = `${delta.left.replace("-", "–")}${delta.name}`;
        let right = `${delta.right.replace("-", "–")}${delta.name}`;
        return (
            <span title={delta.cost.toString()}>
                {(delta.left && delta.right &&
                    <span>{left} &rarr; {right}</span>

                ) || null}
                {(delta.left && !delta.right &&
                    <span>
                        {left}
                    </span>
                ) || null}
                {(!delta.left && delta.right &&
                    <span>
                        {right}
                    </span>
                ) || null}
            </span>
        );
    }
}