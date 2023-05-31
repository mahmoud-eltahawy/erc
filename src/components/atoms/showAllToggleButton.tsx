import { createSignal } from "solid-js";
import { css } from "solid-styled-components";

export default function ShowAllToggleButton(
  props: { toggle: () => void; target: [string | null] },
) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      display: "block",
      width: "25%",
      borderRadius: hover() ? "5px" : "20px",
      fontSize: hover() ? "24px" : "18px",
      border: "solid 3px",
      margin: "2px auto",
      padding: "2px",
    });

  return (
    <button
      onClick={() => props.toggle()}
      class={style()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      type="submit"
    >
      {props.target[0] === "*" ? "شاهد اقل" : "شاهد الكل"}
    </button>
  );
}
