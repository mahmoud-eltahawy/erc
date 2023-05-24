import { createSignal, Show } from "solid-js";
import { css } from "solid-styled-components";

export default function ModfiyButtons({
  permission,
  setUpdating,
  deleteFunc,
}: {
  permission: () => boolean;
  setUpdating: () => void;
  deleteFunc: () => void;
}) {
  enum ModifyButton {
    MODIFY,
    DELETE,
    NONE,
  }
  const [hover, setHover] = createSignal(ModifyButton.NONE);
  const modifyStyle = () =>
    css({
      margin: "7px",
      border: "solid 2px",
      borderRadius: hover() === ModifyButton.MODIFY ? "15px" : "0px",
      color: hover() === ModifyButton.MODIFY ? "blue" : "inherit",
      fontSize: hover() === ModifyButton.MODIFY
        ? "30px"
        : hover() === ModifyButton.DELETE
        ? "10px"
        : "20px",
      width: hover() === ModifyButton.MODIFY
        ? "90%"
        : hover() === ModifyButton.DELETE
        ? "40%"
        : "70%",
      height: hover() === ModifyButton.MODIFY
        ? "70%"
        : hover() === ModifyButton.DELETE
        ? "15%"
        : "40%",
    });

  const deleteStyle = () =>
    css({
      margin: "7px",
      border: "solid 2px",
      color: hover() === ModifyButton.DELETE ? "red" : "inherit",
      borderRadius: hover() === ModifyButton.DELETE ? "15px" : "0px",
      fontSize: hover() === ModifyButton.DELETE
        ? "30px"
        : hover() === ModifyButton.MODIFY
        ? "10px"
        : "20px",
      width: hover() === ModifyButton.DELETE
        ? "90%"
        : hover() === ModifyButton.MODIFY
        ? "40%"
        : "70%",
      height: hover() === ModifyButton.DELETE
        ? "70%"
        : hover() === ModifyButton.MODIFY
        ? "15%"
        : "40%",
    });

  const onLeave = () => setHover(ModifyButton.NONE);
  return (
    <div>
      <Show
        when={permission()}
        fallback={<p>ليس لديك صلاحية التعديل</p>}
      >
        <button
          class={modifyStyle()}
          onMouseOver={() => setHover(ModifyButton.MODIFY)}
          onMouseLeave={onLeave}
          onclick={() => setUpdating()}
        >
          تعديل
        </button>
        <button
          class={deleteStyle()}
          onMouseOver={() => setHover(ModifyButton.DELETE)}
          onMouseLeave={onLeave}
          onClick={() => deleteFunc()}
        >
          حذف
        </button>
      </Show>
    </div>
  );
}
