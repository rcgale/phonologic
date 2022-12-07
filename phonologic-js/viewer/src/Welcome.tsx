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
        return (
            <div id="welcome" className="main-pane">
                <h1>
                    Welcome to PhonoLogic Viewer!
                </h1>
                <p>
                To get started, please use the Browse button above to upload a spreadsheet in .csv or .tsv format. In
                each row, the first column should contain a unique utterance ID, the second column should contain the
                "left" transcript (e.g. an Human Transcript), and the third column should contain the "right" transcript
                (e.g. an ASR transcript). You can choose your own (unique) labels in the header, which will be displayed
                in the resulting analyses. Here's what an example of a data file would look like:
                </p>

                <table id="example-spreadsheet">
                    <thead>
                        <tr>
                            {
                                // <th v-for="columnName in csvHeader.split(',')">
                                // {columnName}
                                // </th>
                            }
                        </tr>
                    </thead>
                    <tbody>
                        {
                            //<tr v-for="row in csvBody.split('\n').filter(r => r.trim())">
                            //   <td v-for="cellText in row.split(',')">{cellText}</td>
                            //</tr>
                        }
                    </tbody>
                </table>
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
            </div>
        );
    }
}
