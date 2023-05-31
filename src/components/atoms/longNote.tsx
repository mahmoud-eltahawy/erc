import { createSignal } from "solid-js";
import togglingButton from "./problemTogglingButton";

export default function LongNote(props: { content: () => string }) {
  const limit = 15;
  const [tooLong, setTooLong] = createSignal(props.content().length > limit);

  return (
    <section>
      <p>{tooLong() ? props.content().slice(0, limit) : props.content()}</p>
      {togglingButton({
        showButton: () => props.content().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </section>
  );
}
