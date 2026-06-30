import * as React from "react";
import {AnalysisCollection} from "../services/AnalysisService";
import {FilePicker} from "./FilePicker";
import {ErrorRate} from "../analysis/ErrorRate";
import {Button, Col, Container, Form, Row} from "react-bootstrap";

type MenuProps = {
    loading: boolean,
    analyses: AnalysisCollection,
    onUpload: (f: File) => void,
    alphabet: string,
    setAlphabet: (a: string) => void,
    receivedFile: (file: File) => void
};

export function Menu({loading, analyses, onUpload, alphabet, setAlphabet, receivedFile}: MenuProps) {
    function reload() {
        window.location.reload();
    }

    return (
        <div id="top-pane" className="main-pane">
            {(analyses?.length &&
                <div id="error-summary">
                    {(!loading &&
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
            <Container id="menu">
                <Col id="file-picker">
                    <Row>
                        <Col>
                            <FilePicker upload={file => onUpload(file)}
                                        onReset={() => reload()}
                                        useDemoFile={(f) => receivedFile(f)}
                                        />
                        </Col>
                    </Row>
                </Col>
                {(analyses?.length &&
                    <Col xs={2}>
                        <Row className="justify-content-center">
                            Alphabet
                        </Row>
                        <Row>
                            <Col>
                                <Form.Label>
                                    IPA
                                    <Form.Check
                                           type="radio"
                                           name="alphabet"
                                           id="radio-ipa"
                                           checked={alphabet === "ipa"}
                                           onChange={(e) => e.target.value && setAlphabet("ipa")} />
                                </Form.Label>
                            </Col>
                            <Col>
                                <Form.Label>
                                    ARPAbet
                                    <Form.Check
                                        type="radio"
                                        name="alphabet"
                                        id="radio-arpabet"
                                        checked={alphabet === "arpabet"}
                                        onChange={(e) => e.target.value && setAlphabet("arpabet")} />
                                </Form.Label>
                            </Col>
                        </Row>
                    </Col>
                ) || null}

            </Container>
        </div>
    );
}