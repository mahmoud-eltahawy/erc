import { createSignal } from "solid-js";
import { Name } from "../..";
import Namer from "./Namer";
import togglingButton from "./problemTogglingButton";

export default function ToggelableList(
  { elements }: { elements: () => Name[] },
) {
  const limit = 3;
  const [tooLong, setTooLong] = createSignal(elements().length > limit);

  return (
    <ul>
      {tooLong()
        ? elements().slice(0, limit).map((element) => <li>{element.name}</li>)
        : elements().map((element) => <li>{element.name}</li>)}
      {togglingButton({
        showButton: () => elements().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </ul>
  );
}

export function ReliableToggelableList({
  ids,
  command,
}: {
  ids: () => string[];
  command: string;
}) {
  const limit = 3;
  const [tooLong, setTooLong] = createSignal(ids().length > limit);

  return (
    <ul>
      {tooLong()
        ? ids().slice(0, limit).map((id) => (
          <li>
            <Namer id={() => id} command={command} />
          </li>
        ))
        : ids().map((id) => (
          <li>
            <Namer id={() => id} command={command} />
          </li>
        ))}
      {togglingButton({
        showButton: () => ids().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </ul>
  );
}
