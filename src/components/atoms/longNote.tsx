import { createSignal } from "solid-js";
import togglingButton from "./problemTogglingButton";

export default function LongNote({ content }: { content: () => string }) {
  const limit = 15;
  const [tooLong, setTooLong] = createSignal(content().length > limit);

  return (
    <section>
      <p>{tooLong() ? content().slice(0, limit) : content()}</p>
      {togglingButton({
        showButton: () => content().length > limit,
        showMore: () => tooLong(),
        doOnClick: () => setTooLong(!tooLong()),
      })}
    </section>
  );
}
