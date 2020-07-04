import * as React from "react"
import * as ReactDOM from 'react-dom'
import App from "./src/app"

console.log("main.js: start")

const app = document.getElementById("app")
ReactDOM.render(
    <App/>,
    app
)