import AlignedSteps from "./AlignedSteps.js";
import Details from "./Details.js";
import TranscriptFormatted from "./TranscriptFormatted.js";
import AudioPlayer from "./AudioPlayer.js";

export default {
        components: { AlignedSteps, Details, TranscriptFormatted, AudioPlayer },
        data() {
            return {
                splits: [],
                split: null,
                analyses: [],
                analysis: null,
                filteredAnalyses: null,
                utterances: null,
                details: {utteranceId: null, utterance: null},
                detailHoverIndex: null,
                overallFer: null,
                overallPer: null,
                alphabet: "ipa",
                loading: true,
            }
        },
        methods: {
            updateUtterances(analysis) {
                let sorted = this.sortUtterances(analysis.items);
                this.utterances = sorted;
                this.fer = analysis.fer;
                this.per = analysis.per;
                if (sorted) {
                    let detail_id = Array.from(sorted.keys())[0];
                    this.setDetails(detail_id, sorted[detail_id]);
                }
                else {
                    this.setDetails(null, null);
                }
            },
            setDetails(utteranceId, utterance) {
                this.details = { utterance_id: utteranceId, utterance: utterance };
            },
            sortUtterances(utterances) {
                return new Map([...Object.entries(utterances)
                    .map(([id, value]) => [id, value])
                    .sort((l, r) => r[1].features.error_rate - l[1].features.error_rate)
                ]);
            },
            costFormatted(cost) {
                let rounded = Math.round(100 * cost, 2) / 100
                return `${rounded}`
            },
            utteranceFeatureCost(utterance) {
                return `${this.costFormatted(utterance.features.distance)}/${utterance.features.expected_length}`
            },
            utterancePhonemeCost(utterance) {
                return `${utterance.phonemes.distance}/${utterance.phonemes.expected_length}`
            },
            transcriptExpected(utterance) {
                return utterance.features.steps.map(s => s.expected).filter(t => t);
            },
            transcriptActual(utterance) {
                return utterance.features.steps.map(s => s.actual).filter(t => t);
            },
            loadFile(analysisPath) {
                this.loading = true;
                this.updateUtterances({per: 0.0, fer: 0.0, items: []});
                fetch(analysisPath).then(r => r.json()).then((r) => {
                    this.updateUtterances(r);
                    this.loading = false;
                });
            },
            selectFile(event) {
                this.loadFile(event.target.value);
            },
            selectSplit(split) {
                this.split = split;
                this.filteredAnalyses = this.analyses.filter(a => a.split === split);
                if (!this.analysis || this.analysis.split !== split) {
                    this.loadFile(this.filteredAnalyses[0].path);
                }
                else {
                    console.log(this.analysis)
                }
            },
        },
        async created () {
            this.analyses = await fetch("./analysis-files.json").then(r => r.json());
            this.splits = [...new Set(this.analyses.map(a => a.split))];
            this.selectSplit(this.splits[0]);
            await this.loadFile(this.filteredAnalyses[0].path);
        },
    template: `
        <div id="results">
        <div id="top-pane" class="main-pane">
            <h1>
                PSST Error Analysis
            </h1>
            <div id="error-summary">
                <table v-if="!loading">
                    <thead>
                        <tr>
                            <th colspan="2">Overall</th>
                        </tr>
                        <tr>
                            <th>FER</th>
                            <th>PER</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{{(100 * fer).toFixed(1)}}%</td>
                            <td>{{(100 * per).toFixed(1)}}%</td>
                        </tr>
                    </tbody>
                </table>
    
            </div>
            <div id="menu">
                <div class="menu-item" id="split-picker" v-show="splits.length > 1">
                    <div><label for="select-file">Split:</label></div>
                    <select :model="split" @change="selectSplit($event.target.value)" id="select-split">
                        <option v-for="s in splits" :value="s">{{s}}</option>
                    </select>
                </div>
                <div class="menu-item" id="file-picker">
                    <div><label for="select-file">Analysis:</label></div>
                    <select :model="analysis" @change="selectFile($event)" id="select-file">
                        <option v-for="f in filteredAnalyses" :value="f.path">{{f.path.replace("/analysis-files/", "")}}</option>
                    </select>
                </div>
                <div class="menu-item" id="alphabet-picker">
                    <div>Alphabet</div>
                    <input type="radio" name="alphabet" id="radio-ipa" value="ipa" v-model="alphabet" />
                    <label for="radio-ipa">IPA</label>
                    <input type="radio" name="alphabet" id="radio-arpabet" value="arpabet" v-model="alphabet" />
                    <label for="radio-arpabet">ARPAbet</label>
                </div>
            </div>
        </div>
        <div id="result-table-wrapper" class="main-pane">
            <table v-if="!loading" id="result-table">
                <thead>
                    <tr class="header-extra">
                        <th class="column-utterance-id">&nbsp;</th>
                        <th class="column-transcript">&nbsp;</th>
                        <th colspan="2" class="header-features" >Features</th>
                        <th colspan="2" class="header-phonemes" >Phonemes</th>
                    </tr>

                    <tr class="header-main">
                        <th class="column-utterance-id">Utterance ID</th>
                        <th class="column-transcript">Transcript (True/Predicted)</th>
                        <th class="column-error-metric">FER</th>
                        <th class="column-error-counts">Err/Len</th>
                        <th class="column-error-metric">PER</th>
                        <th class="column-error-counts">Err/Len</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="[utteranceId, utterance] in utterances" :class="{highlight: utterance == details.utterance}">
                        <td class="column-utterance-id">
                            <button @click="setDetails(utteranceId, utterance)">{{utteranceId}}</button>
                        </td>
                        <td class="column-transcript">
                            <AlignedSteps :steps="utterance.features.steps" :alphabet="alphabet" />
                        </td>
                        <td class="column-error-metric">
                            {{(utterance.features.error_rate * 100).toFixed(1)}}%
                        </td>
                        <td class="column-error-counts">
                            {{ utteranceFeatureCost(utterance) }}
                        </td>
                        <td class="column-error-metric">
                            {{(utterance.phonemes.error_rate * 100).toFixed(1)}}%
                        </td>
                        <td class="column-error-counts">
                            {{ utterancePhonemeCost(utterance) }}
                        </td>
                    </tr>
                </tbody>
            </table>
            <div v-if="loading" class="loading-container">
                <div class="loader">&nbsp;</div>
                Analyzing...
            </div>
        </div>
        <div id="item" v-if="!!details.utterance" class="main-pane">
            <h2>{{details.utterance_id}}</h2>
            <AlignedSteps :steps="details.utterance.features.steps" :alphabet="alphabet" :detailHoverIndex="detailHoverIndex "/>
            <table id="details-error-rates">
                <thead>
                    <tr>
                        <th colspan="2">Utterance</th>
                    </tr>
                    <tr>
                        <th>FER</th>
                        <th>PER</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{{(100 * details.utterance.features.error_rate).toFixed(1)}}%</td>
                        <td>{{(100 * details.utterance.phonemes.error_rate).toFixed(1)}}%</td>
                    </tr>
                </tbody>
            </table>
            <AudioPlayer :utteranceId="details.utterance_id" />
        </div>

        <Details 
            v-if="!!details.utterance" :steps="details.utterance.features.steps"
            :alphabet="alphabet"
            :detailHoverIndex="detailHoverIndex"
            />
    </div>`
};