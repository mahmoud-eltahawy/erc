import { createSignal } from "solid-js";
import { css } from "solid-styled-components";

export default function togglingButton(props: {
  showButton: () => boolean;
  showMore: () => boolean;
  doOnClick: () => void;
}) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      background: "inherit",
      display: "block",
      width: "90%",
      margin: "10px",
      padding: "5px",
      fontSize: "10px",
      borderTop: hover() ? "none" : "dotted 2px",
      borderBottom: hover() ? "none" : "dotted 2px",
      borderRight: hover() ? "dotted 2px" : "none",
      borderLeft: hover() ? "dotted 2px" : "none",
      borderRadius: hover() ? "20px" : "none",
      cursor: "pointer",
      borderTopRightRadius: "3px",
    });
  return (
    props.showButton()
      ? (
        <button
          class={style()}
          onClick={() => props.doOnClick()}
          onMouseOver={() => setHover(true)}
          onMouseLeave={() => setHover(false)}
        >
          {props.showMore() ? "شاهد اكثر" : "شاهد اقل"}
        </button>
      )
      : <></>
  );
}
