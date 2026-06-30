
export interface TranscriptPair {
    id: string
    transcripts: [string, string]
}

export interface TranscriptCollection {
    filename: string
    labels: [string, string]
    rows: TranscriptPair[]
}

export class TranscriptService {
    static async getTranscripts(file: any): Promise<TranscriptCollection> {
        let text = await file.text();
        return this.parseCsv(file.name, text);
    }

    private static parseCsv(filename: string, fileText: string) {
        let sep = filename.endsWith(".csv")
            ? ","
            : "\t";
        let [header, ...lines] = fileText.split("\n").map(s => s.trim());
        let [, ...labels] = header.split(sep);
        let rows = lines.map(line => {
            let [id, left, right] = line.split(sep);
            return {id: id, transcripts: [left, right]} as TranscriptPair
        });
        return {filename: filename, labels: labels, rows: rows} as TranscriptCollection;
    }
}

export default TranscriptService;