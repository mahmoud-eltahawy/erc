import { createSignal } from "solid-js"
import { css } from "solid-styled-components"

export function ToggleButton({
    toggle,
    cont,
    defaultCont,
    tButton
    } : {
    toggle          : Function,
    cont            : string,
    defaultCont     : string,
    tButton         : Function
}){
  const [hover,setHover] = createSignal(false)

  const style = () => css({
      background: "inherit",
      display: "inline-block",
      width: "25%",
      margin: "10px",
      padding: "10px 30px",
      color:        hover() ? "#0f0f0f" : "inherit",
      fontSize:     hover() ? "20px" : "18px",
      borderTop:    hover() ? "none" : "double 5px",
      borderBottom: hover() ? "none" : "solid 1px",
      borderRight:  hover() ? "solid 5px" : "none",
      borderLeft:   hover() ? "solid 5px" : "none",
      cursor: "pointer",
      borderTopRightRadius: "20px",
  })
  return (
    <button
        class={style()}
        onClick={() => toggle()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}>
        {tButton() ? defaultCont : cont}
    </button>
  )
}
