import * as React from "react"
import { CSSProperties } from "react"
import MButton from "./components/MButton"

const style: CSSProperties = {
  background: "#fff",
  border: "1px solid",
  borderColor: "#cbcbcb"
}
export default class App extends React.Component<any, any> {
  constructor(props: any) {
    super(props)
  }

  render() {
    return (
        <div style={style}>
          <MButton>Hello Button</MButton>
        </div>
    )
  }
}