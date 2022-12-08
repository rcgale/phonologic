import {Component} from "react";

type WelcomeProps = {
    useDemoFile: (file: File) => void
};

export class Welcome extends Component<WelcomeProps, {}> {
    render() {
        const csvHeader = "Utterance ID,Human transcript,ASR transcript\n"
        const csvBody =
            "Participant01-laughing,L AE F IH N,B R AA P R IH NG\n" +
            "Participant01-house,EY HH AW S,HH AW S\n" +
            "Participant01-comb,K OW M,K OW M\n" +
            "Participant01-toothbrush,T UW TH B R AH SH,T UW B R AH SH\n";

        let exampleFile = new File([csvHeader, csvBody], "example.csv")
        let exampleDownloadLink = window.URL.createObjectURL(exampleFile);
        const PSSTLink = (
            <a href="https://psst.study/">
                The Post-Stroke Speech Transcription (PSST) Challenge
            </a>
        );

        const SummaryPaperLink = (
            <a href="https://aclanthology.org/2022.rapid-1.6/">the summary we published</a>
        );

        const RapidAtLrecLink = (<span>
            <a href="https://spraakbanken.gu.se/en/rapid-2022">RaPID</a>@<a href="https://lrec2022.lrec-conf.org/en/">LREC 2022</a>
        </span>);

        return (
            <div id="welcome" className="main-pane">
                <div style={{maxWidth: "50rem", margin: "0 auto"}}>
                    <h1>
                        Welcome to PhonoLogic Viewer!
                    </h1>
                    <img
                        style={{height: "1.5em", float: "left", marginTop: "-.1em"}}
                        src={`${process.env.PUBLIC_URL}/images/psst.jpg`}
                        alt="Logo for the PSST challenge. A pair of orange hands in a 'whisper' gesture are embedded in the nodes and edges of a deep neural network."/>
                    <h2>
                        The PSST Challenge (Background)
                    </h2>
                    <p>
                        PhonoLogic Viewer was created as an analysis tool for the automatic speech recognizers (ASRs)
                        submitted to {PSSTLink}. In short, we evaluated the models on a phoneme error rate (PER)
                        and a phonological feature error rate (FER). The FER metric is computed using phonological details
                        that could provide meaningful insight into the behavior of an ASR, but these details are usually
                        shrouded behind an algorithm. PhonoLogic Viewer presents those details in a browsable, visual
                        interface. A more detailed explanation can be found
                        in {SummaryPaperLink} in
                        the proceedings of {RapidAtLrecLink}.
                    </p>
                    <p>
                        The source code for this tool is <a href="https://github.com/rcgale/phonologic/tree/rusty">available on GitHub</a>.
                    </p>
                    <h2>How to use PhonoLogic</h2>
                    <p>
                        For a quick example, we have a simple you can download & modify, or immediately load up in the tool.

                        <ul style={{margin: "1.5rem auto", width: "16rem"}}>
                            <li>
                                <a href={exampleDownloadLink} download>
                                    Download example.csv
                                </a>
                            </li>
                            <li>
                                <button className="button-as-link" onClickCapture={ () => this.props.useDemoFile(exampleFile) }>
                                    Use example.csv
                                </button>
                            </li>
                        </ul>
                    </p>

                    <p>
                    Use the Browse button above to upload a spreadsheet in .csv or .tsv format. In
                    each row, the first column should contain a unique utterance ID, the second column should contain the
                    "left" transcript (e.g. an Human Transcript), and the third column should contain the "right" transcript
                    (e.g. an ASR transcript). You can choose your own (unique) labels in the header, which will be displayed
                    in the resulting analyses.
                    </p>
                    <p>The contents of the example file are structured like so:</p>
                    <table id="example-spreadsheet">
                        <thead>
                        <tr>
                            {csvHeader.split(",").map(columnName => (
                                <th v-for="columnName in csvHeader.split(',')">
                                    {columnName}
                                </th>
                            ))}
                        </tr>
                        </thead>
                        <tbody>
                        {csvBody.split('\n').filter(r => r.trim()).map(row => (
                            <tr>
                                {row.split(',').map(cellText => (
                                      <td v-for="cellText in row.split(',')">{cellText}</td>
                                ))}
                            </tr>
                        ))}
                        </tbody>
                    </table>
                </div>
            </div>
        );
    }
}
