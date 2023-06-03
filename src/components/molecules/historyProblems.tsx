import { invoke } from "@tauri-apps/api";
import {
  Accessor,
  createEffect,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import { css } from "solid-styled-components";
import { departmentsNames, Name } from "../..";
import { employee, permissions } from "../../App";
import Namer from "../atoms/Namer";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function HistoryProblems(props: { rank: number }) {
  const [target, setTarget] = createSignal<string | null>(null);

  const toggle = () => {
    if (target() === "*") {
      setTarget(" ");
      setTarget(null);
    } else {
      setTarget("*");
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
            value={target()!}
            onInput={(e) => setTarget(e.currentTarget.value)}
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
                rank={props.rank + 1}
                target={target}
                departmentId={employee()!.department_id}
              />
            </div>
          }
        >
          <ShowAllHistory rank={props.rank + 1} target={target} />
        </Show>
      </Show>
    </section>
  );
}

function ShowAllToggleButton(
  props: { toggle: () => void; target: Accessor<string | null> },
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
      onClick={() => props.toggle()}
      class={style()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      type="submit"
    >
      {props.target() === "*" ? "شاهد اقل" : "شاهد الكل"}
    </button>
  );
}

const fetcher = async (
  props: { departmentId: string; name: () => string | null },
) => {
  return (await invoke("search_problem", {
    name: props.name() !== " " ? props.name() : null,
    departmentId: props.departmentId,
  })) as Name[];
};

function ShowAllHistory(
  props: { target: Accessor<string | null>; rank: number },
) {
  return (
    <Show when={departmentsNames()}>
      {(notNullDepartments) => (
        <ButtonsOrElementLite
          rank={props.rank}
          buttonElementPairs={() =>
            notNullDepartments()
              .filter((d) => d.id !== "00000000-0000-0000-0000-000000000000")
              .map((d) => [
                d.name,
                <ShowHistory
                  rank={props.rank + 1}
                  departmentId={d.id}
                  target={props.target}
                />,
              ])}
        />
      )}
    </Show>
  );
}

function ShowHistory(props: {
  rank: number;
  departmentId: string;
  target: Accessor<string | null>;
}) {
  const [problems, { refetch }] = createResource({
    departmentId: props.departmentId,
    name: () => props.target(),
  }, fetcher);

  createEffect(() => {
    if (props.target()) {
      refetch();
    }
  });

  return (
    <section>
      <Show when={problems()} fallback={<h1>جاري التحميل ...</h1>}>
        {(notNullProblems) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() =>
              notNullProblems()
                .map((x) => [x.name, <Profile id={x.id} />])}
          />
        )}
      </Show>
    </section>
  );
}

export type Problem = {
  id: string;
  department_id: string;
  title: string;
  description: string;
};

const problem_fetcher = async ({ id }: { id: string }) => {
  return (await invoke("get_problem_problem_by_id", { id })) as Problem;
};

function Profile(props: { id: string }) {
  const [profile] = createResource({ id: props.id }, problem_fetcher);

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
            <th>الوصف</th>
          </tr>
        </thead>
        <tbody>
          <Show when={profile()}>
            {(notNullProfile) => (
              <tr>
                <td>{notNullProfile().title}</td>
                <td>
                  <Namer
                    command="get_department_name_by_id"
                    id={() => notNullProfile()!.department_id}
                  />
                </td>
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
