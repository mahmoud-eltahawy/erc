import { createEffect, createSignal, For, JSXElement, Show } from "solid-js";
import { css } from "solid-styled-components";

type NavButton = {
  rank: number;
  button: JSXElement;
  toggle: () => void;
};

export const [buttons, setButtons] = createSignal<NavButton[]>([]);
const [hoverNavBar, setHoverNavBar] = createSignal(false);

export default function NavBar() {
  const isEmpty = () => buttons().length === 0;
  const heights = () => {
    if (isEmpty()) {
      return `${(height + margin)}px`;
    }
    return `${(height + margin) * buttons().length}px`;
  };
  const style = () =>
    css({
      position: "absolute",
      left: "0px",
      width: hoverNavBar() ? "25%" : "3%",
      height: heights(),
      display: "block",
      backgroundColor: "lightyellow",
      borderLeft: "2px solid",
      borderRight: "2px solid",
      borderBottom: "2px solid",
      borderBottomLeftRadius: "70px",
      borderBottomRightRadius: "70px",
      margin: "1%",
    });
  return (
    <section
      onMouseOver={() => setHoverNavBar(true)}
      onMouseLeave={() => setHoverNavBar(false)}
    >
      <Show
        when={!isEmpty()}
        fallback={<h1>الصفحة الرئيسية</h1>}
      >
        <ul class={style()}>
          <For each={buttons()}>
            {(item) => <li>{item.button}</li>}
          </For>
        </ul>
      </Show>
    </section>
  );
}

const height = 50;
const margin = 10;

export function NavToggleButton({
  transition,
  rank,
  toggle,
  cont,
}: {
  transition: () => boolean;
  rank: number;
  toggle: () => void;
  cont: string;
}) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      background: "inherit",
      display: transition() ? hoverNavBar() ? "block" : "none" : "inline-block",
      width: "70%",
      height: `${height}px`,
      margin: `${margin}px auto`,
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
      setButtons((
        buttons,
      ) => [{ rank, button: core, toggle: toggle }, ...buttons]);
    } else {
      setButtons((buttons) => {
        for (const x of buttons) {
          if (x.rank > rank) {
            x.toggle();
          }
        }
        return buttons.filter((x) => x.rank < rank);
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
