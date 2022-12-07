import {Component} from "react";
import {AnalysisCollection} from "../services/AnalysisService";
import {FilePicker} from "./FilePicker";
import {ErrorRate} from "../analysis/ErrorRate";

type MenuProps = {
    loading: boolean,
    analyses: AnalysisCollection,
    onUpload: (f: File) => void,
    setAlphabet: (a: string) => void,
};

type MenuState = {
    alphabet: string
}

export class Menu extends Component<MenuProps, MenuState> {
    constructor(props: MenuProps) {
        super(props);
        this.state = {
            alphabet: "ipa"
        }
    }

    private reload() {
        window.location.reload();
    }

    private setAlphabet(alphabet: string) {
        this.props.setAlphabet(alphabet);
        this.setState({alphabet: alphabet});
    }

    render() {
        let analyses = this.props.analyses;
        return (
            <div id="top-pane" className="main-pane">
                {(analyses?.length &&
                    <div id="error-summary">
                        {(!this.props.loading &&
                            <table>
                                <thead>
                                <tr>
                                    <th colSpan={2}>Overall</th>
                                </tr>
                                <tr>
                                    <th>FER</th>
                                    <th>PER</th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr>
                                    <td><ErrorRate value={analyses.fer} /></td>
                                    <td><ErrorRate value={analyses.per} /></td>
                                </tr>
                                </tbody>
                            </table>
                        ) || null }
                    </div>
                ) || null}
                <div id="menu">
                    <div className="menu-item" id="file-picker">
                        <FilePicker upload={file => this.props.onUpload(file)}></FilePicker>
                    </div>
                    <div className="menu-item" id="reload-button">
                        <button onClickCapture={() => this.reload()}>Reset</button>
                    </div>
                    {(analyses?.length &&
                        <div className="menu-item" id="alphabet-picker">
                            <div>Alphabet</div>
                            <input type="radio"
                                   name="alphabet"
                                   id="radio-ipa"
                                   checked={this.state.alphabet === "ipa"}
                                   onChange={(e) => e.target.value && this.setAlphabet("ipa")} />
                            <label htmlFor="radio-ipa">IPA</label>
                            <input
                                type="radio"
                                name="alphabet"
                                id="radio-arpabet"
                                checked={this.state.alphabet === "arpabet"}
                                onChange={(e) => e.target.value && this.setAlphabet("arpabet")} />
                            <label htmlFor="radio-arpabet">ARPAbet</label>
                        </div>
                    ) || null}

                </div>
            </div>
        );
    }
}