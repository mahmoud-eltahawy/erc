import { invoke } from "@tauri-apps/api";
import { createEffect, createResource, createSignal, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { css } from "solid-styled-components";
import { departmentsNames, Name } from "../..";
import { employee, permissions } from "../../App";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function HistoryProblems({ rank }: { rank: number }) {
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
        when={permissions()?.includes("AccessHistoryAllDepartmentsProblems") ||
          permissions()?.includes("AccessHistoryDepartmentProblems")}
        fallback={<h1>ليس لديك الصلاحيات للاطلاع علي سجل المشكلات</h1>}
      >
        <div class={container}>
          <input
            value={target[0]!}
            onInput={(e) => setTarget([e.currentTarget.value])}
            class={targetStyle}
            type="text"
            placeholder="ادخل اسم المشكلة"
            required
          />
        </div>
        <ShowAllToggleButton target={target} toggle={toggle} />
        <Show
          when={permissions()?.includes("AccessHistoryAllDepartmentsProblems")}
          fallback={
            <div>
              <h1>مسموح لك بالاطلاع علي مشكلات قسمك فقط</h1>
              <ShowHistory
                rank={rank + 1}
                target={target}
                departmentId={employee()!.department_id}
              />
            </div>
          }
        >
          <ShowAllHistory rank={rank + 1} target={() => target} />
        </Show>
      </Show>
    </section>
  );
}

function ShowAllToggleButton(
  { toggle, target }: { toggle: Function; target: [string | null] },
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

const fetcher = async (
  { departmentId, name }: { departmentId: string; name: () => string | null },
) => {
  return (await invoke("search_problem", {
    name: name() !== " " ? name() : null,
    departmentId,
  })) as Name[];
};

function ShowAllHistory(
  { target, rank }: { target: () => [string | null]; rank: number },
) {
  return (
    <Show when={departmentsNames()}>
      {(notNullDepartments) => (
        <ButtonsOrElementLite
          rank={rank}
          buttonElementPairs={() =>
            notNullDepartments()
              .filter((d) => d.id !== "00000000-0000-0000-0000-000000000000")
              .map((d) => [
                d.name,
                <ShowHistory
                  rank={rank + 1}
                  departmentId={d.id}
                  target={target()}
                />,
              ])}
        />
      )}
    </Show>
  );
}

function ShowHistory(
  { target, departmentId, rank }: {
    rank: number;
    departmentId: string;
    target: [string | null];
  },
) {
  const [problems, { refetch }] = createResource({
    departmentId,
    name: () => target[0],
  }, fetcher);

  createEffect(() => {
    if (target[0]) {
      refetch();
    }
  });

  return (
    <section>
      <Show when={problems()} fallback={<h1>جاري التحميل ...</h1>}>
        {(notNullProblems) => (
          <ButtonsOrElementLite
            rank={rank}
            buttonElementPairs={() =>
              notNullProblems()
                .map((x) => [x.name, <Profile id={x.id} />])}
          />
        )}
      </Show>
    </section>
  );
}

export type Profile = {
  department_name: string;
  writer_name: string;
  title: string;
  description: string;
};

const profiler = async ({ id }: { id: string }) => {
  return (await invoke("profile_problem", { id })) as Profile;
};

function Profile({ id }: { id: string }) {
  const [profile] = createResource({ id }, profiler);

  const tableStyle = css({
    width: "95%",
    margin: "5px auto",
  });

  return (
    <section>
      <table class={tableStyle}>
        <thead>
          <tr>
            <th>الاسم</th>
            <th>القسم</th>
            <th>المؤلف</th>
            <th>الوصف</th>
          </tr>
        </thead>
        <tbody>
          <Show when={profile()}>
            {(notNullProfile) => (
              <tr>
                <td>{notNullProfile().title}</td>
                <td>{notNullProfile().department_name}</td>
                <td>{notNullProfile().writer_name}</td>
                <td>
                  <p>{notNullProfile().description}</p>
                </td>
              </tr>
            )}
          </Show>
        </tbody>
      </table>
    </section>
  );
}
