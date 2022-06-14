import React from "react";
import "./css/style.scss";
import init, { add } from "portfolio";

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
    this.tree = {
      Documents: {
        "about.kons": "hello konsh",
      },
      hello_konsh: {},
    };
    init().then(() => {
      //   console.log(add());
    });
  }

  setTerminal(ind) {
    const terminal = document.getElementById("main_terminal");
    // const body = document.body;
    let innerHeight = window.innerHeight;
    terminal.style.height = innerHeight;
    // body.style.height = innerHeight;
  }

  componentDidMount() {
    window.addEventListener("resize", () => {
      this.setTerminal();
    });
    const cursor = document.getElementById("cursor").classList;
    document.getElementById("command_line").addEventListener("focus", () => {
      cursor.add("active");
    });
    document.getElementById("command_line").addEventListener("blur", () => {
      cursor.remove("active");
    });
  }

  componentDidUpdate() {
    const main_terminal = document.getElementById("main_terminal");
    main_terminal.scrollTop =
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
    const com = command.split(/\xa0+/).filter(Boolean);
    // const com = command.split(" ");
    if (com.length === 0) {
      return null;
    }

    if (com[0] === "hello_konsh") {
      return [
        "Welcome to kons's portfolio:)",
        "Application made by kons are displayed in this site.",
        "ゆっくりしていってね!!!!",
        "",
        "(The list of command can be found by typing 'help command')",
      ];
    } else if (com[0] === "github") {
      return (
        <>
          Please open{" "}
          <a
            href="https://github.com/kons-9"
            target={"_blank"}
            rel="noreferrer noopener"
            style={{ color: "white" }}
          >
            https://github.com/kons-9
          </a>
        </>
      );
    } else if (com[0] === "clear") {
      return "clear";
    } else if (com[0] === "help") {
      if (com[1] === "command") {
        const command_list = [
          "hello_konsh",
          "github",
          "osero",
          "push4",
          "cd, ls, cat, clear",
        ];
        const explanation = [
          "explanation of this site.",
          "diplay url of my github page.",
          "traditional game. you can play this in terminal with option '-t'.(todo)",
          "game like sannmoku. it is easy to understand the rule, but very interesting.(todo)",
          "it behaves like normal terminal.(todo)",
        ];

        let max_len = command_list.reduce(
          (acc, elm) => Math.max(acc, elm.length),
          0
        );
        return command_list.map((content, ind) => (
          <>
            <span
              className="yellow"
              style={{ display: "inline-block", width: max_len * 6 + 1 + "pt" }}
            >
              {content}
            </span>
            : {explanation[ind]}
          </>
        ));
      } else if (com.length === 1) {
        return [
          //   "What help do you want?",
          "usage: help <something>",
          "(You are available 'command' in <something> now)",
        ];
      } else {
        return (
          "help: invalid options: " +
          com.slice(1).reduce((acc, elm) => acc + " " + elm, "")
        );
      }
    } else {
      return "konsh: command not found: " + com[0];
    }
  }
  addContents() {
    const pre_prompt = this.pre_prompt(this.dir);

    const command = document
      .getElementById("input")
      .innerHTML.replace(/&nbsp;/gi, "\xa0");
    document.getElementById("input").innerHTML = "";
    let output = this.processCommand(command);
    if (output === null) {
      this.setState({
        contents: this.state.contents.concat([
          pre_prompt,
          this.prompt + "\xa0" + command,
        ]),
      });
    } else if (output === "clear") {
      this.setState({
        contents: [],
      });
    } else {
      this.setState({
        contents: this.state.contents.concat(
          [pre_prompt, this.prompt + "\xa0" + command].concat(output)
        ),
      });
    }
  }

  updateContent(e) {
    console.log(e.key);
    if (e.key === "Enter") {
      // confirm input
      this.addContents();
    } else if (e.key === "Backspace" || (e.ctrlKey && e.key === "h")) {
      // delete char
      let input = document.getElementById("input").innerHTML;
      // spaceだけ特殊文字を用いているため分岐
      if (input.slice(-6) === "&nbsp;") {
        document.getElementById("input").innerHTML = input.slice(0, -6);
      } else {
        document.getElementById("input").innerHTML = input.slice(0, -1);
      }
      // cursor blink reset
      let cursor = document.getElementById("cursor");
      cursor.classList.remove("active");
      void cursor.offsetWidth;
      cursor.classList.add("active");
    } else if (e.key.length === 1) {
      // input char
      let char = e.key;
      if (e.shiftKey) {
        // change capital letter
        char = char.toUpperCase();
      } else if (char === " ") {
        // change valid space
        char = "\xa0";
      }

      let input = document.getElementById("input").innerHTML;
      input += char;
      document.getElementById("input").innerHTML = input;

      // cursor blink reset
      let cursor = document.getElementById("cursor");
      cursor.classList.remove("active");
      void cursor.offsetWidth;
      cursor.classList.add("active");
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
          <span id="input"></span>
          <span id="cursor">&nbsp;</span>
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
        onClick={() => this.focusEvent()}
        style={{ height: height }}
      >
        {this.state.contents.map((content, ind) => (
          <div className="displayed_area" key={ind}>
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
