import { createSignal } from "solid-js";
import { css } from "solid-styled-components";

export default function SubmitButton(
  props: { length: (() => number) | undefined },
) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      display: "block",
      width: "40%",
      borderRadius: hover() ? "5px" : "20px",
      fontSize: hover() ? "24px" : "18px",
      border: "solid 3px",
      margin: "2px auto",
      padding: "2px",
    });

  return (
    <button
      class={style()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      type="submit"
    >
      تاكيد {props.length ? props.length() : ""}
    </button>
  );
}
