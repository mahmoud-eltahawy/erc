import {
  Accessor,
  createEffect,
  createSignal,
  For,
  JSXElement,
  Show,
} from "solid-js";
import { NavToggleButton } from "../../navBar";

export function ButtonsOrElement(props: {
  rank: number;
  buttonElementPairs: () => [string, JSXElement][];
  num: Accessor<number>;
  fun: () => void;
}) {
  const [buttonIndex, setButtonIndex] = createSignal(-1);

  const toggle = (id: number) => {
    if (buttonIndex() === id) {
      setButtonIndex(-1);
    } else {
      setButtonIndex(id);
    }
  };

  createEffect(() => {
    if (props.num() !== -1) {
      toggle(props.num());
      props.fun();
    }
  });

  const isChosen = (index: number) => buttonIndex() === index;

  return (
    <For each={props.buttonElementPairs()}>
      {(item, index) => (
        <>
          <Show when={buttonIndex() === -1 || isChosen(index())}>
            <NavToggleButton
              rank={props.rank}
              transition={() => buttonIndex() !== -1}
              cont={item[0] as string}
              toggle={() => toggle(index())}
            />
          </Show>
          <Show when={isChosen(index())}>{item[1]}</Show>
        </>
      )}
    </For>
  );
}

export function ButtonsOrElementLite(props: {
  rank: number;
  buttonElementPairs: () => [string, JSXElement][];
}) {
  const [buttonIndex, setButtonIndex] = createSignal(-1);

  const toggle = (id: number) => {
    if (buttonIndex() === id) {
      setButtonIndex(-1);
    } else {
      setButtonIndex(id);
    }
  };

  const isChosen = (index: number) => buttonIndex() === index;

  return (
    <For each={props.buttonElementPairs()}>
      {(item, index) => (
        <>
          <Show when={buttonIndex() === -1 || isChosen(index())}>
            <NavToggleButton
              rank={props.rank}
              transition={() => buttonIndex() !== -1}
              cont={item[0] as string}
              toggle={() => toggle(index())}
            />
          </Show>
          <Show when={isChosen(index())}>{item[1]}</Show>
        </>
      )}
    </For>
  );
}
