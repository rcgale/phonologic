import React, {Component, useEffect, useState} from 'react';
import {Analysis, AnalysisStep} from "@phonologic/wasm";
import {PhonologicViewer} from "./PhonologicViewer";

type WordProps = {
    initValue: string,
    onChange: any,
};

type WordState = {
    value: string
};

class Word extends Component<WordProps, WordState> {
    render() {
        return (
            <input type="text" value={this.props.initValue} onChange={(el) => this.props.onChange(el.target.value)}/>
        );
    }
}

type StepProps = {
    step: AnalysisStep
}

class StepComponent extends Component<StepProps, {}> {
    display() {
        let step = this.props.step;
        switch (step.action) {
            case "SUB":
                return <span>{step.action}: /{step.left || " "}/ for /{step.right || " "}/</span>;
            case "EQ":
                return <span>{step.action}: /{this.props.step.left || " "}/</span>;
            case "DEL":
                return <span>{step.action}: /{this.props.step.left || " "}/</span>;
            case "INS":
                return <span>{step.action}: /{this.props.step.right || " "}/</span>;
        }
    }

    render() {
        return (
            <li>
                cost: {this.props.step.cost} features, {this.display()}
            </li>
        );
    }
}

type AppState = {
    left: string,
    right: string,
    diff: Analysis | undefined,
    error: string,
}

export function App() {
    const [left, setLeft] = useState<string>("ˈkæktʌs");
    const [right, setRight] = useState<string>("ˈtæktʌs");
    const [diff, setDiff] = useState<Analysis | undefined>(undefined);
    const [error, setError] = useState<string>("");

    useEffect(() => {
        onWordChange()

    }, []);

    const onWordChange = async (stateUpdate: any = {}) => {
        // await this.setState(stateUpdate);
        // try {
        //   let diff = await getDiff(this.state.left, this.state.right);
        //   await this.setState({diff: diff});
        // }
        // catch (e: any) {
        //   await this.setState({error: (e || "").toString()});
        // }
    };

    const steps = () => {
        let steps: AnalysisStep[] = diff
            ? diff.steps
            : [];
        return steps
            // .filter(s => s.action != "EQ")
            .map(s => <StepComponent key={(s as any)["ptr"]} step={s}/>);
    }

    return (
        <div className="App">
            <PhonologicViewer/>
            <header className="App-header">
                <p>
                    <Word initValue={left} onChange={(w: string) => onWordChange({left: w})}/>
                    <Word initValue={right} onChange={(w: string) => onWordChange({right: w})}/>
                </p>
                <ul>
                    {steps() || <p>/{left}/ and /{right}/ match</p>}
                </ul>
                <p>
                    {error || ""}
                </p>
            </header>
        </div>
    );
}

export default App;
