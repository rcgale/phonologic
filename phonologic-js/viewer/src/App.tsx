import React, {Component} from 'react';
import './App.css';
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
        <input type="text" value={this.props.initValue} onChange={(el) => this.props.onChange(el.target.value)} />
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
  diff: Analysis|undefined,
  error: string,
}

export class App extends Component<{}, AppState> {
  constructor(props: any) {
    super(props)
    this.state = {
      left: "ˈkæktʌs",
      right: "ˈtæktʌs",
      diff: undefined,
      error: ""
    };
  }

  componentDidMount() {
    this.onWordChange()
  }

  onWordChange = async (stateUpdate: any = {}) => {
    // await this.setState(stateUpdate);
    // try {
    //   let diff = await getDiff(this.state.left, this.state.right);
    //   await this.setState({diff: diff});
    // }
    // catch (e: any) {
    //   await this.setState({error: (e || "").toString()});
    // }
  };

  steps = () => {
    let steps: AnalysisStep[] = this.state.diff
        ? this.state.diff.steps
        : [];
    return steps
        // .filter(s => s.action != "EQ")
        .map(s => <StepComponent key={(s as any)["ptr"]} step={s} />);
  }

  render = () => {
    return (
        <div className="App">
          <PhonologicViewer />
          <header className="App-header">
            <p>
              <Word initValue={this.state.left} onChange={(w: string) => this.onWordChange({left: w})}/>
              <Word initValue={this.state.right} onChange={(w: string) => this.onWordChange({right: w})}/>
            </p>
            <ul>
              {this.steps() || <p>/{this.state.left}/ and /{this.state.right}/ match</p>}
            </ul>
            <p>
              {this.state.error || ""}
            </p>
          </header>
        </div>
    );
  }
}

export default App;
