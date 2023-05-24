import { invoke } from "@tauri-apps/api";
import { createSignal, Setter, Show } from "solid-js";
import { css } from "solid-styled-components";
import { employee, permissions } from "../../App";
import SubmitButton from "../atoms/submitButton";

export default function DefineProblem({
  toggle,
}: {
  toggle: () => void;
}) {
  const [title, setTitle] = createSignal("");
  const [desc, setDesc] = createSignal("");

  async function handleSubmit(e: Event) {
    e.preventDefault();
    toggle();
    try {
      await invoke("define_problem", {
        departmentId: employee()?.department_id,
        title: title(),
        description: desc(),
      });
      setTitle("");
      setDesc("");
    } catch (err) {
      alert(err);
    }
  }

  const style = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={style}>
      <Show
        when={permissions()?.includes("DefineProblem")}
        fallback={<h1>ليس لديك صلاحية تعريف مشكلة</h1>}
      >
        <form onSubmit={handleSubmit}>
          <TitleInput title={() => title()} setTitle={setTitle} />
          <DescriptionInput desc={() => desc()} setDesc={setDesc} />
          <SubmitButton length={() => desc().length} />
        </form>
      </Show>
    </section>
  );
}

export function DescriptionInput(
  { desc, setDesc }: { desc: () => string; setDesc: Setter<string> },
) {
  const style = css({
    display: "block",
    width: "50%",
    fontSize: "20px",
    padding: ".5em",
    margin: ".3em auto",
    backgroundColor: "beige",
    border: "solid 3px",
  });
  return (
    <textarea
      value={desc()}
      onInput={(e) => setDesc(e.currentTarget.value)}
      maxLength={349}
      cols={30}
      rows={5}
      class={style}
      placeholder="اقل من 350 حرف"
      required
    >
    </textarea>
  );
}

function TitleInput(
  { title, setTitle }: { title: () => string; setTitle: Setter<string> },
) {
  const style = css({
    display: "block",
    width: "50%",
    fontSize: "20px",
    padding: ".5em",
    margin: ".3em auto",
    backgroundColor: "beige",
    border: "solid 3px",
  });

  return (
    <input
      value={title()}
      class={style}
      onInput={(e) => setTitle(e.currentTarget.value)}
      type="text"
      placeholder="اسم المشكلة"
      required
    />
  );
}
