import React from "react";
import "./css/style.scss";

function Header() {
  return <div className="konsh_header">Welcome to kons's portfolio!</div>;
}
class Main extends React.Component {
  constructor(props) {
    super(props);
    let contents = [
      <>########################################################</>,
      <>
        #################
        <span className="yellow"> toshiki's portfolio! </span>
        #################
      </>,
      <>########################################################</>,
      <></>,
      <></>,
      <>
        If it is first time to access this site, you should type "
        <span className="green">hello_konsh</span>"
      </>,
      <></>,
    ];
    this.state = {
      contents: contents,
    };
    this.prompt = "%";
    this.dir = "~";
    this.pre_prompt = (dir) => (
      <>
        <span className="green">kons@portfolio:</span>
        &nbsp;
        <span className="yellow">{dir}</span>
      </>
    );
  }

  setTerminal(ind) {
    const terminal = document.getElementById("main_terminal");
    let innerHeight = window.innerHeight;
    terminal.style.height = innerHeight - 150 + "px";
  }

  componentDidMount() {
    const func = this.setTerminal;
    window.addEventListener("resize", func);
  }

  componentDidUpdate() {
    let main_terminal = document.getElementById("main_terminal");
    document.getElementById("main_terminal").scrollTop =
      main_terminal.scrollHeight - main_terminal.offsetHeight;
  }

  line_number(ind) {
    // todo: countが3桁になったら全体をそれに合わせて変える
    const leng = Math.max(
      Math.floor(Math.log10(this.state.contents.length + 1)) + 1,
      3
    );
    const num = ind + 1;
    const num_string = num.toString().padStart(leng, "\xa0");
    return <span className="line_number">{num_string} </span>;
  }
  processCommand(command) {
    console.log(command);
    return <>hello world</>;
  }
  addContents() {
    const pre_prompt = this.pre_prompt(this.dir);

    const command = document.getElementById("input").innerHTML;
    document.getElementById("input").innerHTML = "";
    let output = this.processCommand(command);
    if (output === null) {
      this.setState({
        contents: this.state.contents.concat([
          pre_prompt,
          this.prompt + "\xa0" + command,
        ]),
      });
    } else {
      this.setState({
        contents: this.state.contents.concat([
          pre_prompt,
          this.prompt + "\xa0" + command,
          output,
        ]),
      });
    }
  }

  updateContent(e) {
    if (e.key === "Enter") {
      this.addContents();
    } else if (e.key === "Backspace" || (e.ctrlKey && e.key === "h")) {
      let input = document.getElementById("input").innerHTML;
      document.getElementById("input").innerHTML = input.slice(0, -1);
    } else if (e.key.length === 1) {
      let char = e.key;
      if (e.shiftKey) {
        char = char.toUpperCase();
      }
      let input = document.getElementById("input").innerHTML;
      input += char;
      document.getElementById("input").innerHTML = input;
    }
  }
  focusEvent() {
    let main_terminal = document.getElementById("main_terminal");
    main_terminal.scrollTop =
      main_terminal.scrollHeight - main_terminal.offsetHeight;
    document.getElementById("command_line").focus();
  }
  inputarea() {
    const count = this.state.contents.length;
    return (
      <div className="input_area" spellCheck="false">
        <p className="command">
          <span className="prompt">
            {this.line_number(count)}
            {this.pre_prompt(this.dir)}
            <br />
            {this.line_number(count + 1)}
            {this.prompt}&nbsp;
          </span>
          <span className="not_prompt">
            {/* <span className="green">hello_konsh </span> */}
            <span id="input"></span>
            <span className="cursor">&nbsp;</span>
          </span>
        </p>
        <textarea
          className="command_line"
          id="command_line"
          onKeyDown={(e) => this.updateContent(e)}
        ></textarea>
      </div>
    );
  }

  render() {
    const innerHeight = window.innerHeight;
    const height = innerHeight - 150 + "px";

    //   this.setTerminal(terminal);
    return (
      <div
        className="main_terminal"
        id="main_terminal"
        onClick={this.focusEvent}
        style={{ height: height }}
      >
        {this.state.contents.map((content, ind) => (
          <div className="displayed_area">
            <p>
              {this.line_number(ind)}
              {content}
            </p>
          </div>
        ))}
        {this.inputarea()}
      </div>
    );
  }
}
function Footer() {
  return (
    <div id="konsh_footer">
      <div className="status_bar">
        <div className="sta_1">TERMINAL</div>
        <div className="sta_2">konsh | -</div>
        <div className="sta_3">unix | utf-8 | no ft | 0x0</div>
        <div className="sta_4">100%</div>
        <div className="sta_5">35:14</div>
      </div>
      <div className="command_bar">:terminal</div>
    </div>
  );
}

class Terminal extends React.Component {
  //   constructor(props) {
  //     super(props);
  //   }
  renderHeader() {
    return <Header />;
  }
  renderMain() {
    return <Main />;
  }
  renderFooter() {
    return <Footer />;
  }

  render() {
    return (
      <div className="terminal">
        {this.renderHeader()}
        {this.renderMain()}
        {this.renderFooter()}
      </div>
    );
  }
}

export default Terminal;
