import {Component} from "react";
import {FeatureDelta} from "@phonologic/wasm";

type FeaturesProps = {
    delta: FeatureDelta
}

export function Features({delta}: FeaturesProps) {
    function value(value: number) {
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

    function cost(delta: FeatureDelta) {
        let name = (delta.left || delta.right).replace(/[+\-0]/, '')
        return `Cost for [${name}]: ${delta.cost}`
    }

    const left = `${delta.left.replace("-", "–")}${delta.name}`;
    const right = `${delta.right.replace("-", "–")}${delta.name}`;
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