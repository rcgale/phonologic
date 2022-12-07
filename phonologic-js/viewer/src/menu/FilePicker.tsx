import {ChangeEvent, Component} from "react";

type FilePickerProps = {
    upload: (file: File) => void
};

type FilePickerState = {
    loading: boolean
};

export class FilePicker extends Component<FilePickerProps, FilePickerState> {
    constructor(props: FilePickerProps) {
        super(props);
        this.state = {
            loading: false
        }
    }
    uploadFile = async (element: ChangeEvent<HTMLInputElement>) => {
        this.setState({loading: true});
        if (element.target.files?.length) {
            this.props.upload(element.target.files[0]);
        }
        this.setState({loading: false});
    }
    render() {
        return (
            <div className="file-picker">
                <label htmlFor="select-file">Choose a File:</label>
                <input type="file" onChange={this.uploadFile} />
                {this.state.loading &&
                    <div className="loading-container">
                        <div className="loader">&nbsp;</div>
                        Processing...
                    </div>
                }

            </div>        );
    }
}
