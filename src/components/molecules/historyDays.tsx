import { invoke } from "@tauri-apps/api";
import { createEffect, createResource, createSignal, Show } from "solid-js";
import { css } from "solid-styled-components";
import { departmentsNames } from "../..";
import { employee, permissions } from "../../App";
import { ButtonsOrElementLite } from "./buttonsOrElement";
import ShiftWrittenShow from "./shiftWrittenNote";

type Day = {
  date: [string, string, string];
  shifts: [string, string][];
};

const [begin, setBegin] = createSignal<string | null>();
const [end, setEnd] = createSignal<string | null>();

export default function HistoryDays(props: { rank: number }) {
  const dateContainer = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  });

  const dateInput = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor: "lightyellow",
    borderRadius: "20px",
  });

  const dateLabel = css({
    display: "inline-block",
    width: "35%",
    padding: ".1em",
    margin: ".1em auto",
  });

  return (
    <section>
      <Show
        when={permissions()?.includes(
          "AccessHistoryAllDepartmentsDepartmentProblems",
        ) ||
          permissions()?.includes("AccessHistoryDepartmentDepartmentProblems")}
        fallback={<h1>غير مسموح لك بالاطلاع علي سجل الورديات</h1>}
      >
        <div class={dateContainer}>
          <input
            value={end()!}
            onChange={(e) => setEnd(e.currentTarget.value)}
            class={dateInput}
            type="date"
            required
          />
          <label class={dateLabel}>
            <h4>تاريخ النهاية</h4>
          </label>
        </div>
        <div class={dateContainer}>
          <input
            value={begin()!}
            onChange={(e) => setBegin(e.currentTarget.value)}
            class={dateInput}
            type="date"
            required
          />
          <label class={dateLabel}>
            <h4>تاريخ البداية</h4>
          </label>
        </div>
        <Show
          when={permissions()?.includes(
            "AccessHistoryAllDepartmentsDepartmentProblems",
          )}
          fallback={
            <div>
              <h1>مسموح لك بالاطلاع علي سجل ورديات قسمك فقط</h1>
              <ShowHistory
                rank={props.rank}
                departmentId={() => employee()!.department_id}
              />
            </div>
          }
        >
          <ShowAllHistory rank={props.rank} />
        </Show>
      </Show>
    </section>
  );
}

const fetcher = async ({ departmentId }: {
  departmentId: () => string;
}) => {
  return (await invoke("search_shifts", {
    departmentId: departmentId(),
    begin: begin(),
    end: end(),
  }).catch((err) => console.log(err))) as Day[];
};

function ShowAllHistory(props: { rank: number }) {
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
                  departmentId={() => d.id}
                />,
              ])}
        />
      )}
    </Show>
  );
}

function ShowHistory(props: {
  rank: number;
  departmentId: () => string;
}) {
  const [days, { refetch }] = createResource({
    departmentId: props.departmentId,
  }, fetcher);

  createEffect(() => {
    const b = begin();
    const e = end();
    console.log(b + "  " + e);
    refetch();
  });

  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <div class={container}>
      <Show when={days()}>
        {(notNullDays) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() =>
              notNullDays()
                .map(
                  (x) => [
                    x.date.reverse().join(" / "),
                    <Shifts
                      rank={props.rank + 1}
                      shifts={() => x.shifts}
                    />,
                  ],
                )}
          />
        )}
      </Show>
    </div>
  );
}

function Shifts(props: { shifts: () => [string, string][]; rank: number }) {
  return (
    <div>
      <ButtonsOrElementLite
        rank={props.rank}
        buttonElementPairs={() =>
          props.shifts()
            .map((x) => [
              x[1],
              <ShiftWrittenShow
                rank={props.rank + 1}
                shiftId={() => x[0]}
              />,
            ])}
      />
    </div>
  );
}
