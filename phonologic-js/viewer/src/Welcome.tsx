import * as React from "react";
import psstJpg from './assets/images/psst.jpg';
import {Button} from "react-bootstrap";
import 'bootstrap/dist/css/bootstrap.min.css';
import {EXAMPLE_CSV_BODY, EXAMPLE_CSV_HEADER, EXAMPLE_FILE} from "./Example";

type WelcomeProps = {
    useDemoFile: (file: File) => void
};

export function Welcome({useDemoFile}: WelcomeProps) {
    let exampleDownloadLink = window.URL.createObjectURL(EXAMPLE_FILE);
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
                    src={psstJpg}
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
                </p>

                <ul style={{margin: "1.5rem auto", width: "16rem"}}>
                    <li>
                        <a href={exampleDownloadLink} download>
                            Download example.csv
                        </a>
                    </li>
                    <li>
                        <Button variant="link" onClickCapture={ () => useDemoFile(EXAMPLE_FILE) }>
                            Use example.csv
                        </Button>
                    </li>
                </ul>

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
                        {EXAMPLE_CSV_HEADER.split(",").map((columnName, columnIdx) => (
                            <th key={columnIdx}>
                                {columnName}
                            </th>
                        ))}
                    </tr>
                    </thead>
                    <tbody>
                    {EXAMPLE_CSV_BODY.split('\n').filter(r => r.trim()).map((row, rowIdx) => (
                        <tr key={rowIdx}>
                            {row.split(',').map((cellText, cellIdx) => (
                                  <td key={cellIdx}>{cellText}</td>
                            ))}
                        </tr>
                    ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
