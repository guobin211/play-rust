import * as React from "react"
import { CSSProperties } from "react"

const btnStyle: CSSProperties = {
  background: "#f2f3f5",
  color: "#000000",
  fontSize: "28px",
  width: "200px",
  height: "48px",
  borderRadius: "8px"
}
export default class MButton extends React.Component<any, any> {

  constructor(props: any) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
  }

  handleClick() {
    window.alert("Hello Click Button");
  }

  render() {
    return (
        <button style={btnStyle} onClick={this.handleClick}>
          {this.props.children}
        </button>
    )
  }
}