import type {TranscriptPair} from "./TranscriptService";
import init, {FeatureDelta, PhlAnalyzer} from "@phonologic/wasm";

const cleanse = (obj: any) => JSON.parse(JSON.stringify(obj));

export interface AnalysisStep {
    left: string
    right: string
    cost: number
    action: string
}

export interface AnalysisDetails {
    distance: number
    expectedLength: number
    steps: Array<AnalysisStep>
    errorRate: number
}

export interface Analysis {
    id: string
    transcriptPair: TranscriptPair
    features: AnalysisDetails
    phonemes: AnalysisDetails
    left: string
    right: string
    fer: number
    per: number
}

export interface AnalysisException extends Analysis {
    id: string
    message: string
}

export class AnalysisCollection extends Array<Analysis> {
    featureDistance: number = NaN;
    featureLength: number = NaN;
    fer: number = NaN;
    phonemeDistance: number = NaN;
    phonemeLength: number = NaN;
    per: number = NaN;
    exceptions: AnalysisException[];

    constructor(items: Analysis[], exceptions: AnalysisException[] = []) {
        super(...Array.from(items));
        this.exceptions = exceptions;
        [
            this.featureDistance,
            this.featureLength,
            this.phonemeDistance,
            this.phonemeLength
        ] = AnalysisCollection.summarize(items);
        if (!exceptions.length) {
            this.fer = this.featureDistance / this.featureLength;
            this.per = this.phonemeDistance / this.phonemeLength
        }
    }

    private static summarize(items: Analysis[]) {
        if (!items.length) {
            return [0, 0, 0, 0];
        }
        return Object.values(
            Array.from(items).map(a => ({
                    featureDistance: a.features.distance,
                    featureLength: a.features.expectedLength,
                    phonemeDistance: a.phonemes.distance,
                    phonemeLength: a.phonemes.expectedLength,
            })).reduce((partial, a) => {
                return {
                    featureDistance: partial.featureDistance + a.featureDistance,
                    featureLength: partial.featureLength + a.featureLength,
                    phonemeDistance: partial.phonemeDistance + a.phonemeDistance,
                    phonemeLength: partial.phonemeLength + a.phonemeLength,
                };
            })
        );
    }

    static EMPTY = new AnalysisCollection([]);
}


export class AnalysisService {
    private static _analyzer: PhlAnalyzer|null = null;
    private static getAnalyzer = async (system: string = "hayes-ipa-arpabet") => {
        if (!this._analyzer) {
            await init();
            this._analyzer = new PhlAnalyzer(system);
        }
        return this._analyzer;
    }

    static async getDeltas(left: string, right: string) {
        if (!left || !right) {
            return [];
        }
        let analyzer = await this.getAnalyzer();
        let result = cleanse(analyzer.featureDeltas(left, right));
        return result.deltas as FeatureDelta[];
    }

    static async getAll(transcriptPairs: TranscriptPair[]): Promise<Analysis[]> {
        await this.getAnalyzer();
        let analysisResults = await Promise.all(
            transcriptPairs.map(
                async tp => {
                    try {
                        return await AnalysisService.getAnalysis(tp);
                    }
                    catch (e: any) {
                        return {id: tp.id, message: e.message} as AnalysisException;
                    }
                }
            )
        );
        // split analyses from exceptions
        const [analyses, exceptions] = analysisResults.reduce(([a, e], item) => {
            return "message" in item
                ?  [a, [...e, item as AnalysisException]]
                : [[...a, item as Analysis], e];
        }, [[], []] as [Analysis[], AnalysisException[]]);
        return new AnalysisCollection(analyses, exceptions);
    }

    private static async getAnalysis(transcriptPair: TranscriptPair): Promise<Analysis> {
        // const wasm-bak = import('./wasm');
        // const phl: any = await wasm-bak;
        const analyzer = await this.getAnalyzer("hayes-ipa-arpabet");
        let [leftTranscript, rightTranscript] = transcriptPair.transcripts;
        let phonemes = cleanse(analyzer.phonemeDiff(leftTranscript, rightTranscript));
        let features = cleanse(analyzer.featureDiff(leftTranscript, rightTranscript));
        return this.adaptPhlAnalysis(transcriptPair.id, phonemes, features, transcriptPair);
    }

    private static adaptPhlAnalysis(id: string, phonemes: any, features: any, transcripts: TranscriptPair) {
        return {
            id: id,
            transcriptPair: transcripts,
            features: this.adaptDiff(features),
            phonemes: this.adaptDiff(phonemes),
            fer: features.errorRate,
            per: phonemes.errorRate,
        } as Analysis;
    }

    private static adaptDiff(diff: any) {
        return {
            distance: diff.cost,
            expectedLength: diff.length,
            steps: diff.steps.map(this.adaptStep),
            errorRate: diff.errorRate,
        } as AnalysisDetails
    }

    private static adaptStep(step: any) {
        return {
            left: step.left,
            right: step.right,
            cost: step.cost,
            action: step.action,
            deltas: [] // todo
        } as AnalysisStep
    }
}
