import { createEffect, createResource, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { css } from "solid-styled-components";
import { employees_names_fetcher } from "../..";
import { permissions } from "../../App";
import ShowAllToggleButton from "../atoms/showAllToggleButton";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function HistoryEmployees(props: { rank: number }) {
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
        when={permissions()?.includes("AccessHistoryEmployees")}
        fallback={<h1>ليس لديك الصلاحيات للاطلاع علي سجل الموظفين</h1>}
      >
        <div class={container}>
          <input
            value={target[0]!}
            onInput={(e) => setTarget([e.currentTarget.value])}
            class={targetStyle}
            type="text"
            placeholder="ادخل اسم الموظف"
            required
          />
        </div>
        <ShowAllToggleButton target={target} toggle={toggle} />
        <ShowHistory rank={props.rank + 1} target={target} />
      </Show>
    </section>
  );
}

function ShowHistory(props: { rank: number; target: [string | null] },
) {
  const [employees, { refetch }] = createResource(
    { name: () => props.target[0] },
    employees_names_fetcher,
  );

  createEffect(() => {
    if (props.target[0]) {
      refetch();
    }
  });

  return (
    <section>
      <Show
        when={employees()}
        fallback={<h1>جاري التحميل ...</h1>}
      >
        {(notNullEmployees) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() =>
              notNullEmployees()
                .filter((d) => d.id !== "00000000-0000-0000-0000-000000000000")
                .map((x) => [x.name, <h1>employee profile</h1>])}
          />
        )}
      </Show>
    </section>
  );
}
