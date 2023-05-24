import { invoke } from "@tauri-apps/api";
import { createEffect, createResource, createSignal, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { css } from "solid-styled-components";
import { Name } from "../..";
import { permissions } from "../../App";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function HistoryMachines({ rank }: { rank: number }) {
  const [target, setTarget] = createStore<[string | null]>([null]);

  const toggle = () => {
    if (target[0] === "*") {
      setTarget([" "]);
      setTarget([null]);
    } else {
      setTarget(["*"]);
    }
  };

  const container = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  });

  const targetStyle = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor: "lightyellow",
    borderRadius: "20px",
  });

  return (
    <section>
      <Show
        when={permissions()?.includes("AccessHistoryMachines")}
        fallback={<h1>ليس لديك الصلاحيات للاطلاع علي الماكينات</h1>}
      >
        <div class={container}>
          <input
            value={target[0]!}
            onInput={(e) => setTarget([e.currentTarget.value])}
            class={targetStyle}
            type="text"
            placeholder="ادخل اسم الماكينة"
            required
          />
        </div>
        <ShowAllToggleButton target={target} toggle={toggle} />
        <ShowHistory rank={rank} target={target} />
      </Show>
    </section>
  );
}

function ShowAllToggleButton(
  { toggle, target }: { toggle: () => void; target: [string | null] },
) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      display: "block",
      width: "25%",
      borderRadius: hover() ? "5px" : "20px",
      fontSize: hover() ? "24px" : "18px",
      border: "solid 3px",
      margin: "2px auto",
      padding: "2px",
    });

  return (
    <button
      onClick={() => toggle()}
      class={style()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      type="submit"
    >
      {target[0] === "*" ? "شاهد اقل" : "شاهد الكل"}
    </button>
  );
}

const fetcher = async ({ name }: { name: () => string | null }) => {
  return (await invoke("search_machines", {
    name: name() !== " " ? name() : null,
  })) as Name[];
};

function ShowHistory(
  { rank, target }: { rank: number; target: [string | null] },
) {
  const [machines, { refetch }] = createResource(
    { name: () => target[0] },
    fetcher,
  );

  createEffect(() => {
    if (target[0]) {
      refetch();
    }
  });

  return (
    <section>
      <Show
        when={machines()}
        fallback={<h1>جاري التحميل ...</h1>}
      >
        {(notNullMachines) => (
          <ButtonsOrElementLite
            rank={rank}
            buttonElementPairs={() =>
              notNullMachines()
                .map((x) => [x.name, <h1>machine profile</h1>])}
          />
        )}
      </Show>
    </section>
  );
}
