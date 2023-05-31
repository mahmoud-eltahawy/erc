import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";
import SubmitButton from "../atoms/submitButton";
import { ToggleButton } from "../atoms/toggleButton";
import { DescriptionInput } from "./defineProblem";
import { ShiftNote } from "./shiftWrittenNote";

export default function AddShiftNote(props: {
  toggle: () => void;
}) {
  const [desc, setDesc] = createSignal("");

  async function handleSubmit(e: Event) {
    e.preventDefault();
    props.toggle();
    try {
      await invoke("save_shift_note", {
        content: desc(),
      });
      setDesc("");
    } catch (err) {
      alert(err);
    }
  }
  return (
    <form onSubmit={handleSubmit}>
      <DescriptionInput desc={() => desc()} setDesc={setDesc} />
      <SubmitButton length={() => desc().length} />
    </form>
  );
}

export function UpdateShiftNote(props: {
  note: () => ShiftNote;
  toggle: () => void;
}) {
  const { id, shift_id, content } = props.note();
  const [desc, setDesc] = createSignal(content.trim());

  async function handleSubmit(e: Event) {
    e.preventDefault();
    props.toggle();
    try {
      await invoke("upgrade_shift_note", {
        note: { id, shift_id, content: desc().trim() },
        oldContent: content.trim(),
      });
      setDesc("");
    } catch (err) {
      alert(err);
    }
  }
  return (
    <form onSubmit={handleSubmit}>
      <DescriptionInput desc={() => desc()} setDesc={setDesc} />
      <SubmitButton length={() => desc().length} />
      <ToggleButton
        toggle={props.toggle}
        cont=""
        defaultCont="الغاء"
        tButton={() => true}
      />
    </form>
  );
}
