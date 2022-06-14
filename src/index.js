import React from "react";
import ReactDOM from "react-dom/client";
// import "./css/style.css";
import Terminal from "./App";
import reportWebVitals from "./reportWebVitals";

const root = ReactDOM.createRoot(document.getElementById("root"));
// console.log(welcome("toshiki"));
root.render(
  <React.StrictMode>
    <Terminal />
  </React.StrictMode>
);

// var terminal = null;

// function setTerminal() {
//   let innerHeight = window.innerHeight;
//   terminal.style.height = innerHeight - 150 + "px";
// }

// window.addEventListener("load", function () {
//   terminal = document.getElementById("main_terminal");

//   setTerminal();
// });

// window.addEventListener("resize", function () {
//   setTerminal();
// });

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
