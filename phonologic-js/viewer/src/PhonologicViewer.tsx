import './index.css';
import './PhonologicViewer.css';

import {useState} from "react";
import type {Analysis} from "./services/AnalysisService";
import {AnalysisCollection, AnalysisService} from "./services/AnalysisService";
import TranscriptService from "./services/TranscriptService";
import {SelectedModal} from "./details/SelectedModal";
import {Welcome} from "./Welcome";
import {Menu} from "./menu/Menu";
import {ResultTable} from "./results/ResultTable";
import {HoverContext} from "./HoverContext";

export function PhonologicViewer() {
    const [file, setFile] = useState<File|null>(null);
    const [analyses, setAnalyses] = useState(AnalysisCollection.EMPTY);
    const [selectedId, setSelectedId] = useState<string|null>(null);
    const [overallFer, setOverallFer] = useState<number|null>(null);
    const [overallPer, setOverallPer] = useState<number|null>(null);
    const [alphabet, setAlphabet] = useState<string>("ipa");
    const [loading, setLoading] = useState<boolean>(false);
    const [errorMessage, setErrorMessage] = useState<string>("");
    const [labelLeft, setLabelLeft] = useState<string>("");
    const [labelRight, setLabelRight] = useState<string>("");

    function analysis(): Analysis|null {
        if (!selectedId) {
            return null;
        }
        let found = analyses.filter(a => a.id === selectedId);
        return found.length
            ? found[0]
            : null;
    }
    async function receivedFile(file: File) {
        try {
            setAnalyses(AnalysisCollection.EMPTY);
            setErrorMessage("");
            setLoading(true);
            const parsedFile = await TranscriptService.getTranscripts(file);
            const analyses = await AnalysisService.getAll(parsedFile.rows);
            const collection = new AnalysisCollection(analyses);
            setAnalyses(collection);
            setLoading(false)
            setLabelLeft(parsedFile.labels[0])
            setLabelRight(parsedFile.labels[1])
        }
        catch (e: any) {
            setErrorMessage(e.message || "Unknown error")
        }
    }

    return (
        <div id="app">
            <HoverContext value={useState<number>()}>
                <div id="results">
                    <Menu
                        alphabet={alphabet}
                        setAlphabet={setAlphabet}
                        analyses={analyses}
                        loading={loading}
                        onUpload={(f) => receivedFile(f)}
                        receivedFile={receivedFile}
                    />
                    <div id="main-pane-wrapper" className="main-pane">
                        <div className="error-message">{errorMessage}</div>
                        {(!loading && !errorMessage && !analyses?.length &&
                            <Welcome useDemoFile={(f) => receivedFile(f) } />
                        ) || null}
                        {(!errorMessage &&
                                <ResultTable
                                    loading={loading}
                                    show={(id: string) => setSelectedId(id)}
                                    analyses={analyses}
                                    alphabet={alphabet}
                                    labelLeft={labelLeft}
                                    labelRight={labelRight} />
                        ) || null}
                    </div>
                </div>
                <SelectedModal
                    deselect={() => setSelectedId(null)}
                    selectedId={selectedId}
                    analysis={analysis()}
                    alphabet={alphabet}
                    labelLeft={labelLeft}
                    labelRight={labelRight}
                    />
            </HoverContext>
        </div>
    );
}
