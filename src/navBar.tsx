import { createEffect, createSignal, For, JSXElement, Show } from "solid-js";
import { css } from "solid-styled-components";

type NavButton = {
  rank: number;
  button: JSXElement;
  toggle: Function;
};

export const [buttons, setButtons] = createSignal<NavButton[]>([]);
const [hoverNavBar, setHoverNavBar] = createSignal(false);

export default function NavBar() {
  const style = () =>
    css({
      position: "absolute",
      left: "0px",
      width: hoverNavBar() ? "25%" : "5%",
      height: "100%",
    });
  const mugStyle = () =>
    css({
      display: hoverNavBar() ? "block" : "none",
      backgroundColor: "lightyellow",
      borderLeft: "2px solid",
      borderRight: "2px solid",
      borderBottom: "2px solid",
      borderBottomLeftRadius: "70px",
      borderBottomRightRadius: "70px",
      margin: "5%",
    });
  return (
    <section
      onMouseOver={() => setHoverNavBar(true)}
      onMouseLeave={() => setHoverNavBar(false)}
      class={style()}
    >
      <ul class={mugStyle()}>
        <For each={buttons()}>
          {(item) => <li>{item.button}</li>}
        </For>
      </ul>
    </section>
  );
}

export function NavToggleButton({
  transition,
  rank,
  toggle,
  cont,
}: {
  transition: () => boolean;
  rank: number;
  toggle: Function;
  cont: string;
}) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      background: "inherit",
      display: transition() ? hoverNavBar() ? "block" : "none" : "inline-block",
      width: "70%",
      margin: "10px auto",
      padding: "10px 30px",
      color: hover() ? "#0f0f0f" : "inherit",
      fontSize: hover() ? "20px" : "18px",
      borderTop: hover() ? "none" : "double 5px",
      borderBottom: hover() ? "none" : "solid 1px",
      borderRight: hover() ? "solid 5px" : "none",
      borderLeft: hover() ? "solid 5px" : "none",
      cursor: "pointer",
      borderTopRightRadius: "20px",
    });

  createEffect(() => {
    if (transition()) {
      setButtons((buttons) => {
        const newList = [{ rank, button: core, toggle: toggle }, ...buttons];
        newList.sort((x, y) => y.rank - x.rank);
        return newList;
      });
    } else {
      setButtons((buttons) => {
        const newList = [];
        for (const x of buttons) {
          if (x.rank > rank) {
            x.toggle();
          } else {
            newList.push(x);
          }
        }
        return newList;
      });
    }
  });

  const core = (
    <button
      class={style()}
      onClick={() => toggle()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      {cont}
    </button>
  );

  return <Show when={!transition()}>{core}</Show>;
}
