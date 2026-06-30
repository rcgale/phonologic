import {ChangeEvent, useState} from "react";
import {Button, Container, Form} from "react-bootstrap";
import {EXAMPLE_FILE} from "../Example";

interface FilePickerProps {
    upload: (file: File) => void
    onReset: () => void
    useDemoFile: (file: File) => void
}

export function FilePicker({upload, onReset, useDemoFile}: FilePickerProps) {
    const [loading, setLoading] = useState(false);
    const uploadFile = async (element: ChangeEvent<HTMLInputElement>) => {
        setLoading(true);
        if (element.target.files?.length) {
            upload(element.target.files[0]);
        }
        setLoading(false)
    }

    return (
        <Container className="file-picker">
            <Form.Label htmlFor="select-file">Choose a File:
                <Form.Control type="file" onChange={uploadFile} />
            </Form.Label>
            <Button variant="secondary" onClickCapture={() => onReset()}>Reset</Button>
            <Button variant="light" onClickCapture={ () => useDemoFile(EXAMPLE_FILE) }>
                    Example
            </Button>
            {loading &&
                <div className="loading-container">
                    <div className="loader">&nbsp;</div>
                    Processing...
                </div>
            }
        </Container>
    );
}
