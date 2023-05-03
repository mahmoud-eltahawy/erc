import { createEffect, createSignal, For, JSXElement, Show } from "solid-js";
import { NavToggleButton } from "../../navBar";

export function ButtonsOrElement({
  rank,
  buttonElementPairs,
  num,
  fun,
}: {
  rank: number;
  buttonElementPairs: () => [string, JSXElement][];
  num: number[];
  fun: Function;
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
    if (num[0] !== -1) {
      toggle(num[0]);
      fun();
    }
  });

  const isChosen = (index: number) => buttonIndex() === index;

  return (
    <For each={buttonElementPairs()}>
      {(item, index) => (
        <>
          <Show when={buttonIndex() === -1 || isChosen(index())}>
            <NavToggleButton
              rank={rank}
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

export function ButtonsOrElementLite({
  rank,
  buttonElementPairs,
}: {
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
    <For each={buttonElementPairs()}>
      {(item, index) => (
        <>
          <Show when={buttonIndex() === -1 || isChosen(index())}>
            <NavToggleButton
              rank={rank}
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
