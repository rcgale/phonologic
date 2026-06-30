import * as React from "react";

interface DistanceProps {
    distance: number
    expectedLength: number
}

export function Distance({distance, expectedLength}: DistanceProps) {
    return <span>{distance} / {expectedLength}</span>;
}