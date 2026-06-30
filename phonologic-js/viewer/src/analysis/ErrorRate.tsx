import * as React from "react";

interface ErrorRateProps {
    value: number
}

export function ErrorRate({value}: ErrorRateProps) {
    return (
        <span>
            {(value * 100).toFixed(1)}%
        </span>
    );
}