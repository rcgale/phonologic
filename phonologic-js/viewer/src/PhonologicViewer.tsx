import './PhonologicViewer.css';

import {Component} from "react";
import type {Analysis} from "./services/AnalysisService";
import {AnalysisCollection, AnalysisService} from "./services/AnalysisService";
import TranscriptService from "./services/TranscriptService";
import {SelectedModal} from "./details/SelectedModal";
import {Welcome} from "./Welcome";
import {Menu} from "./menu/Menu";
import {ResultTable} from "./results/ResultTable";

type PhonologicViewerState = {
    file: File|null,
    analyses: AnalysisCollection,
    selectedId: string|null,
    overallFer: number|null,
    overallPer: number|null,
    alphabet: string,
    loading: boolean
    errorMessage: string,
    labelLeft: string,
    labelRight: string,
}

export class PhonologicViewer extends Component<{}, PhonologicViewerState> {
    constructor(props: any) {
        super(props);
        this.state =  {
            file: null,
            analyses: AnalysisCollection.EMPTY,
            selectedId: null,
            overallFer: null,
            overallPer: null,
            alphabet: "ipa",
            loading: false,
            errorMessage: "",
            labelLeft: "",
            labelRight: "",
        } as PhonologicViewerState;
    }
    analysis(): Analysis|null {
        if (!this.state.selectedId) {
            return null;
        }
        let found = this.state.analyses.filter(a => a.id === this.state.selectedId);
        return found.length
            ? found[0]
            : null;
    }
    async receivedFile(file: File) {
        try {
            this.setState({analyses: AnalysisCollection.EMPTY, errorMessage: "", loading: true});
            let parsedFile = await TranscriptService.getTranscripts(file);
            let analyses = await AnalysisService.getAll(parsedFile.rows);
            let collection = new AnalysisCollection(analyses);
            this.setState({
                analyses: collection,
                loading: false,
                labelLeft: parsedFile.labels[0],
                labelRight: parsedFile.labels[1]
            });
        }
        catch (e: any) {
            this.setState({errorMessage: e.message || "Unknown error"});
        }
    }
    render() {
        return (
            <div id="app">
                <div id="results">
                    <Menu
                          analyses={this.state.analyses}
                          loading={this.state.loading}
                          onUpload={(f) => this.receivedFile(f)}
                          setAlphabet={(a: string) => this.setState({alphabet: a})} />
                    <div id="main-pane-wrapper" className="main-pane">
                        <div className="error-message">{this.state.errorMessage}</div>
                        {(!this.state.loading && !this.state.errorMessage && !this.state.analyses?.length &&
                            <Welcome useDemoFile={(f) => this.receivedFile(f) } />
                        ) || null}
                        {(!this.state.errorMessage &&
                                <ResultTable
                                    loading={this.state.loading}
                                    show={(id: string) => this.setState({selectedId: id})}
                                    analyses={this.state.analyses}
                                    alphabet={this.state.alphabet}
                                    labelLeft={this.state.labelLeft}
                                    labelRight={this.state.labelRight} />
                        ) || null}
                    </div>
                </div>
                <SelectedModal
                    deselect={() => this.setState({selectedId: null})}
                    selectedId={this.state.selectedId}
                    analysis={this.analysis()}
                    alphabet={this.state.alphabet}
                    labelLeft={this.state.labelLeft}
                    labelRight={this.state.labelRight}
                    />
            </div>
        );
    }
}