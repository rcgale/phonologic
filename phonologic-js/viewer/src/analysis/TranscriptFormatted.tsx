import {Component} from "react";

type TranscriptFormattedProps = {
    transcript: string,
    alphabet: string
};

export class TranscriptFormatted extends Component<TranscriptFormattedProps, {}> {
    formatTranscript(tokens: string[] | string) {
        if (!tokens) {
            return "";
        }
        // @ts-ignore
        if (!tokens.join) {
            tokens = [tokens as string];
        }
        tokens = (tokens as string[]);

        const arpaToIpa: any = {
            "P": "p", "B": "b", "M": "m", "W": "w", "F": "f", "V": "v", "DH": "ð", "TH": "θ", "T": "t",
            "D": "d", "S": "s", "Z": "z", "N": "n", "L": "l", "DX": "ɾ", "CH": "t͡ʃ", "JH": "d͡ʒ", "SH": "ʃ",
            "ZH": "ʒ", "R": "ɹ", "Y": "j", "K": "k", "G": "ɡ", "NG": "ŋ", "HH": "h", "IY": "i", "UW": "u",
            "IH": "ɪ", "UH": "ʊ", "EH": "ɛ", "EY": "e͡ɪ", "AH": "ʌ", "AO": "ɔ", "OY": "ɔ͡ɪ", "OW": "o͡ʊ",
            "AE": "æ", "AW": "a͡ʊ", "AY": "a͡ɪ", "AA": "ɑ", "ER": "ɝ"
        }
        if (this.props.alphabet === "ipa") {
            return tokens.map(t => arpaToIpa[t] || t).join("");
        }
        else {
            return tokens.join(" ");
        }
    }
    render() {
        let transcriptFormatted = this.formatTranscript(this.props.transcript);
        return (
            <span className="transcript">{transcriptFormatted}</span>
        )
    }
}
