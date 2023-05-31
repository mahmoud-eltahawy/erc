import { createSignal } from "solid-js";
import { Name } from "../..";
import Namer from "./Namer";
import togglingButton from "./problemTogglingButton";

export default function ToggelableList(
  props: { elements: () => Name[] },
) {
  const limit = 3;
  const [tooLong, setTooLong] = createSignal(props.elements().length > limit);

  return (
    <ul>
      {tooLong()
        ? props.elements().slice(0, limit).map((element) => <li>{element.name}</li>)
        : props.elements().map((element) => <li>{element.name}</li>)}
      {togglingButton({
        showButton: () => props.elements().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </ul>
  );
}

export function ReliableToggelableList(props: {
  ids: () => string[];
  command: string;
}) {
  const limit = 3;
  const [tooLong, setTooLong] = createSignal(props.ids().length > limit);

  return (
    <ul>
      {tooLong()
        ? props.ids().slice(0, limit).map((id) => (
          <li>
            <Namer id={() => id} command={props.command} />
          </li>
        ))
        : props.ids().map((id) => (
          <li>
            <Namer id={() => id} command={props.command} />
          </li>
        ))}
      {togglingButton({
        showButton: () => props.ids().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </ul>
  );
}
