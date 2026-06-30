import {TranscriptDiff} from "./TranscriptDiff";
import {Details} from "./Details";
import {Analysis} from "../services/AnalysisService";
import {Component, useState} from "react";
import {HoverContext} from "../HoverContext";

type SelectedModalProps = {
    deselect: () => void,
    selectedId: string|null,
    analysis: Analysis|null,
    alphabet: string,
    labelLeft: string,
    labelRight: string
}

export function SelectedModal({deselect, selectedId, analysis, alphabet, labelLeft, labelRight}: SelectedModalProps) {
    return (
        <div id="selected-modal" style={{visibility: selectedId ? "visible" : "hidden"}}>
            <HoverContext value={useState()}>
                {(selectedId && analysis &&
                    <div id="selected-analysis">
                        <button className="close-selected" onClickCapture={() => deselect()}>✕</button>
                        <h2>{selectedId}</h2>
                        <TranscriptDiff
                            key={selectedId}
                            analysis={analysis}
                            alphabet={alphabet}
                            // detailHoverIndex={detailHoverIndex}
                            labelLeft={labelLeft}
                            labelRight={labelRight}
                        />
                        {/*<AudioPlayer utteranceId="selectedId" />*/}
                    </div>
                ) || null}
                {(selectedId && analysis &&
                    <Details
                        key={selectedId}
                        selectedId={selectedId}
                        analysis={analysis}
                        alphabet={alphabet}
                    />
                ) || null}
            </HoverContext>
        </div>
    );
}
