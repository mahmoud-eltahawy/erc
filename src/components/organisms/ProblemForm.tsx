import { invoke } from "@tauri-apps/api";
import {
  Accessor,
  createEffect,
  createResource,
  createSignal,
  For,
  Setter,
  Show,
} from "solid-js";
import { Name, ShiftProblem } from "../../index";
import { listen } from "@tauri-apps/api/event";
import { css } from "solid-styled-components";
import SubmitButton from "../atoms/submitButton";
import { employee, permissions, shiftId } from "../../App";
import { ToggleButton } from "../atoms/toggleButton";

type InitType = {
  maintainer: Name;
  machine: Name;
  begin_time: string;
  end_time: string;
  problems: Name[];
  spare_parts: Name[];
  note: string | null;
};

async function init_fetcher(props: {
  id: string;
}): Promise<InitType> {
  const { begin_time, end_time, machine_id, maintainer_id } = await invoke(
    "get_shift_problem_by_id",
    { id: props.id },
  )
    .catch((err) => console.log(err)) as ShiftProblem;
  console.log(begin_time);
  const t_name = await invoke("employee_name", { id: maintainer_id }) as string;
  const maintainer = { id: maintainer_id, name: t_name } as Name;
  const m_name = await invoke("get_machine_name_by_id", {
    id: machine_id,
  }) as string;
  const machine = { id: machine_id, name: m_name } as Name;
  const note = await invoke("get_shift_problem_note_by_id", { id: props.id }) as
    | string
    | null;
  const problems_ids = await invoke("get_shift_problem_problems_ids_by_id", {
    id: props.id,
  }) as string[];
  const spare_parts_ids = await invoke(
    "get_shift_problem_spare_parts_ids_by_id",
    { id: props.id },
  ) as string[];
  const problems: Name[] = [];
  for (const id of problems_ids) {
    const name = await invoke("get_problem_name_by_id", { id }) as string;
    problems.push({ id, name });
  }
  const spare_parts: Name[] = [];
  for (const id of spare_parts_ids) {
    const name = await invoke("get_spare_part_name_by_id", { id }) as string;
    spare_parts.push({ id, name });
  }

  return {
    maintainer,
    machine,
    begin_time,
    end_time,
    problems,
    spare_parts,
    note,
  };
}

export function ProblemUpdateForm(props: {
  toggle: () => void;
  id: string;
}) {
  const [init] = createResource({ id: props.id }, init_fetcher);

  function Core({
    init,
  }: {
    init: InitType;
  }) {
    const {
      machine,
      maintainer,
      begin_time,
      end_time,
      problems,
      spare_parts,
      note,
    } = init;

    const [beginTimeI, setBeginTime] = createSignal(begin_time);
    const [endTimeI, setEndTime] = createSignal(end_time);
    const [employeesI, setEmployees] = createSignal<Name[]>([maintainer]);
    const [machinesI, setMachines] = createSignal<Name[]>([machine]);
    const [problemsI, setProblems] = createSignal<Name[]>([...problems]);
    const [sparePartsI, setSpareParts] = createSignal<Name[]>([
      ...(spare_parts || []),
    ]);
    const [noteI, setNote] = createSignal(note);

    listen_selections_updates();

    const handleSubmit = async (e: Event) => {
      e.preventDefault();
      if (!machinesI().at(0)) {
        alert("يجب تحديد الالة التي تمت عليها الصيانة");
        return;
      }
      if (!employeesI().at(0)) {
        alert("يجب تحديد الموظف الذي قام بالصيانة");
        return;
      }
      if (!problemsI().length) {
        alert("يجب تحديد مشكلة واحدة علي الاقل");
        return;
      }
      props.toggle();
      try {
        await invoke("update_problem_detail", {
          shiftProblemId: props.id,
          core: [
            [maintainer.id, employeesI().at(0)!.id],
            [machine.id, machinesI().at(0)!.id],
            [
              begin_time,
              beginTimeI().length === 8 ? beginTimeI() : beginTimeI() + ":00",
            ],
            [
              end_time,
              endTimeI().length === 8 ? endTimeI() : endTimeI() + ":00",
            ],
          ],
          problems: [
            problems.map((p) => p.id),
            problemsI().map((problem) => problem.id),
          ],
          spareParts: [
            spare_parts?.map((s) => s.id) || [],
            sparePartsI()?.map((part) => part.id) || [],
          ],
          note: [note, noteI()?.trim()],
        });
      } catch (err) {
        alert(err);
      }
    };

    return (
      <div class={container}>
        <Show
          when={permissions()?.includes("WriteDepartmentProblem") &&
            shiftBorders()}
          fallback={<h1>ليس لديك صلاحية تسجيل عطل</h1>}
        >
          <form onSubmit={handleSubmit}>
            <TimeConstraint
              beginTime={() => beginTimeI()}
              endTime={() => endTimeI()}
              setBeginTime={setBeginTime}
              setEndTime={setEndTime}
            />
            <SearchBars
              employees={employeesI()}
              setEmployees={setEmployees}
              problems={problemsI()}
              setProblems={setProblems}
              spareParts={sparePartsI()}
              setSpareParts={setSpareParts}
              machines={machinesI()}
              setMachines={setMachines}
            />
            <ExtraNote note={() => noteI() || ""} setNote={setNote} />
            <SubmitButton length={undefined} />
            <ToggleButton
              toggle={props.toggle}
              cont=""
              defaultCont="الغاء"
              tButton={() => true}
            />
          </form>
        </Show>
      </div>
    );
  }
  return (
    <Show when={init()}>
      {(notNullInit) => <Core init={notNullInit()} />}
    </Show>
  );
}

export function ProblemSaveForm(props: {
  toggle: () => void;
}) {
  const [beginTime, setBeginTime] = createSignal("");
  const [endTime, setEndTime] = createSignal("");
  const [employees, setEmployees] = createSignal<Name[]>([]);
  const [machines, setMachines] = createSignal<Name[]>([]);
  const [spareParts, setSpareParts] = createSignal<Name[]>([]);
  const [problems, setProblems] = createSignal<Name[]>([]);
  const [note, setNote] = createSignal("");

  listen_selections_updates();

  const restore = () => {
    setBeginTime("");
    setEndTime("");
    setEmployees([]);
    setMachines([]);
    setSpareParts([]);
    setProblems([]);
    setNote("");
  };

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (!machines().at(0)) {
      alert("يجب تحديد الالة التي تمت عليها الصيانة");
      return;
    }
    if (!employees().at(0)) {
      alert("يجب تحديد الموظف الذي قام بالصيانة");
      return;
    }
    if (!problems().length) {
      alert("يجب تحديد مشكلة واحدة علي الاقل");
      return;
    }
    props.toggle();
    try {
      const problemDetail = {
        shift_id: shiftId(),
        maintainer_id: employees().at(0)!.id,
        machine_id: machines().at(0)!.id,
        begin_time: beginTime().length === 8
          ? beginTime()
          : beginTime() + ":00",
        end_time: endTime().length === 8 ? endTime() : endTime() + ":00",
        problems_ids: problems().map((problem) => problem.id),
        spare_parts_ids: spareParts().length
          ? spareParts().map((part) => part.id)
          : null,
        note: note() ? note().trim() : null,
      };
      await invoke("save_problem_detail", {
        problemDetail,
      });
      restore();
    } catch (err) {
      alert(err);
    }
  };

  return (
    <div class={container}>
      <Show
        when={permissions()?.includes("WriteDepartmentProblem") &&
          shiftBorders()}
        fallback={<h1>ليس لديك صلاحية تسجيل عطل</h1>}
      >
        <form onSubmit={handleSubmit}>
          <TimeConstraint
            beginTime={() => beginTime()}
            endTime={() => endTime()}
            setBeginTime={setBeginTime}
            setEndTime={setEndTime}
          />
          <SearchBars
            employees={employees()}
            setEmployees={setEmployees}
            problems={problems()}
            setProblems={setProblems}
            spareParts={spareParts()}
            setSpareParts={setSpareParts}
            machines={machines()}
            setMachines={setMachines}
          />
          <ExtraNote note={() => note()} setNote={setNote} />
          <SubmitButton length={undefined} />
        </form>
      </Show>
    </div>
  );
}

const borders_fetcher = async () => {
  return (await invoke("current_shift_borders")) as [string, string];
};
const [shiftBorders, { refetch }] = createResource(borders_fetcher);

listen("shift_ended", () => refetch());

type SearchDeps = {
  command: string;
  the_name: string | null;
  canceled: () => string[];
  limit: () => number;
};

const fetcher = async ({
  limit,
  command,
  canceled,
  the_name,
}: SearchDeps) => {
  let name = null;
  if (the_name) {
    if (the_name !== " ") {
      name = the_name;
    }
  }
  return (await invoke(command, {
    name,
    canceled: canceled(),
    limit: limit(),
  })) as Name[];
};
const department_fetcher = async ({
  limit,
  command,
  canceled,
  the_name,
}: SearchDeps) => {
  let name = null;
  if (the_name) {
    if (the_name !== " ") {
      name = the_name;
    }
  }
  return (await invoke(command, {
    departmentId: employee()!.department_id,
    name,
    canceled: canceled(),
    limit: limit(),
  })) as Name[];
};

const listen_selections_updates = () => {
  listen("create_problem", () => {
    setUpdates("Problem");
  });
  listen("delete_problem", () => {
    setUpdates("Problem");
  });

  listen("create_employee", () => {
    setUpdates("Employee");
  });
  listen("delete_employee", () => {
    setUpdates("Employee");
  });

  listen("create_machine", () => {
    setUpdates("Machine");
  });
  listen("delete_machine", () => {
    setUpdates("Machine");
  });

  listen("create_spare_part", () => {
    setUpdates("SparePart");
  });
  listen("delete_spare_part", () => {
    setUpdates("SparePart");
  });
};

export type Updates =
  | "Problem"
  | "SparePart"
  | "Machine"
  | "Employee"
  | "None";

const [updates, setUpdates] = createSignal<Updates>("None");

const container = css({
  display: "block",
  fontSize: "x-large",
  borderTop: "solid 2px",
  borderBottom: "solid 9px",
  margin: "1% auto",
  padding: "1%",
});

function TimeConstraint(props: {
  endTime: () => string;
  setEndTime: (s: string) => void;
  beginTime: () => string;
  setBeginTime: (s: string) => void;
}) {
  const timeLabel = css({
    display: "inline-block",
    width: "35%",
    padding: ".1em",
    margin: ".1em auto",
  });
  const timeContainer = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  });

  const timeInput = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor: "lightyellow",
    borderRadius: "20px",
  });
  return (
    <section>
      <div class={timeContainer}>
        <input
          value={props.endTime()}
          onChange={(e) => props.setEndTime(e.currentTarget.value)}
          class={timeInput}
          type="time"
          min={props.beginTime()}
          max={shiftBorders()!.at(1)!.slice(0, 5)}
          required
        />
        <label class={timeLabel}>
          <h4>وقت النهاية</h4>
        </label>
      </div>
      <div class={timeContainer}>
        <input
          value={props.beginTime()}
          onChange={(e) => props.setBeginTime(e.currentTarget.value)}
          class={timeInput}
          type="time"
          min={shiftBorders()!.at(0)!.slice(0, 5)}
          max={props.endTime()}
          required
        />
        <label class={timeLabel}>
          <h4>وقت البداية</h4>
        </label>
      </div>
    </section>
  );
}

function ExtraNote(
  props: { note: () => string | null; setNote: Setter<string> },
) {
  const [displayNote, setDisplayNote] = createSignal(false);

  const toggleNote = () => {
    if (displayNote()) {
      setDisplayNote(false);
    } else {
      setDisplayNote(true);
    }
  };

  return (
    <section>
      <NoteButton
        length={() => props.note()?.length || 0}
        toggleNote={toggleNote}
      />
      <Show when={displayNote()}>
        <NoteText
          note={() => props.note() || ""}
          setNote={props.setNote}
        />
      </Show>
    </section>
  );
}

function NoteText(props: { note: () => string; setNote: Setter<string> }) {
  const style = css({
    fontSize: "x-large",
    width: "90%",
    backgroundColor: "blanchedalmond",
  });

  return (
    <textarea
      value={props.note()}
      onInput={(e) => props.setNote(e.currentTarget.value)}
      class={style}
      cols={30}
      rows={4}
      maxLength={499}
      placeholder="اكتب ما لا يتجاوز 500 حرف"
    >
    </textarea>
  );
}

function NoteButton(props: { length: () => number; toggleNote: () => void }) {
  const [hover, setHover] = createSignal(false);

  const style = () =>
    css({
      display: "block",
      width: "15%",
      borderRadius: hover() ? "5px" : "20px",
      fontSize: hover() ? "22px" : "16px",
      border: "solid 1px",
      margin: "2px auto",
      padding: "2px",
    });

  return (
    <button
      type="button"
      onClick={() => props.toggleNote()}
      class={style()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      اضافة ملحوظة {props.length()}
    </button>
  );
}

function SearchBars(props: {
  machines: Name[];
  setMachines: Setter<Name[]>;
  employees: Name[];
  setEmployees: Setter<Name[]>;
  problems: Name[];
  setProblems: Setter<Name[]>;
  spareParts: Name[];
  setSpareParts: Setter<Name[]>;
}) {
  function fetcher_object({
    name,
    command,
    collection,
  }: {
    name: () => string | null;
    command: string;
    collection: () => string[];
  }): SearchDeps {
    const LIMIT = 5;
    return {
      command,
      the_name: name(),
      canceled: collection,
      limit: () => collection().length > LIMIT ? collection().length : LIMIT,
    } as SearchDeps;
  }

  return (
    <section>
      <SearchBar
        subject="Machine"
        updates={updates}
        chosens={props.machines}
        setChosens={props.setMachines}
        isMulti={false}
        mtMessage="لا يوجد ماكينة بهذا الاسم"
        defaultPlaceholder="ابحث عن الماكينة التي تمت عليها الصيانة"
        resultPlaceholder="الماكينة"
        selection_fetcher={(name: () => string | null) =>
          fetcher(fetcher_object({
            command: "machines_selection",
            name,
            collection: () => props.machines.map((m) => m.name),
          }))}
        nyMessage={null}
      />
      <SearchBar
        subject="Employee"
        updates={updates}
        chosens={props.employees}
        setChosens={props.setEmployees}
        isMulti={false}
        mtMessage="لا يوجد موظف بهذا الاسم"
        defaultPlaceholder="ابحث عن الموظف الذي قام بالصيانة"
        resultPlaceholder="الموظف"
        selection_fetcher={(name: () => string | null) =>
          fetcher(fetcher_object({
            command: "employees_selection",
            name,
            collection: () => props.employees.map((m) => m.id),
          }))}
        nyMessage={null}
      />
      <SearchBar
        subject="Problem"
        updates={updates}
        chosens={props.problems}
        setChosens={props.setProblems}
        isMulti={true}
        mtMessage="لا يوجد مشكلة بهذا الاسم"
        defaultPlaceholder="ابحث عن مشكلة او مشاكل"
        resultPlaceholder="عدد المشاكل"
        selection_fetcher={(name: () => string | null) =>
          department_fetcher(fetcher_object({
            command: "problems_selection",
            name,
            collection: () => props.problems.map((m) => m.name),
          }))}
        nyMessage={"لم يتم اختيار اي مشكلة حتي الان <اجباري> ا"}
      />
      <SearchBar
        subject="SparePart"
        updates={updates}
        chosens={props.spareParts}
        setChosens={props.setSpareParts}
        isMulti={true}
        mtMessage="لا توجد قطعة غيار بهذا الاسم"
        defaultPlaceholder="ابحث عن قطع الغيار المستخدمة في الصيانة"
        resultPlaceholder="عدد قطع الغيار المستخدمة"
        selection_fetcher={(name: () => string | null) =>
          fetcher(fetcher_object({
            command: "spare_parts_selection",
            name,
            collection: () => props.spareParts.map((m) => m.name),
          }))}
        nyMessage={"لم يتم تسجيل اي قطع غيار <اختياري> ا"}
      />
    </section>
  );
}

function SearchBar(props: {
  subject: string;
  updates: Accessor<Updates>;
  defaultPlaceholder: string;
  resultPlaceholder: string;
  mtMessage: string;
  nyMessage: string | null;
  isMulti: boolean;
  selection_fetcher: (name: () => string | null) => Promise<Name[]>;
  chosens: Name[];
  setChosens: Setter<Name[]>;
}) {
  const [target, setTarget] = createSignal<string | null>(null);
  const [optionsList, { refetch }] = createResource(
    () => target,
    props.selection_fetcher,
  );

  createEffect(() => {
    if (updates() === props.subject || target()) {
      refetch();
    }
  });

  const getChosenOne = () => {
    if (props.chosens.at(0)) {
      return props.resultPlaceholder + " : " + props.chosens.at(0)!.name;
    } else {
      return props.defaultPlaceholder;
    }
  };

  const choiceOptionHandler = (member: Name) => {
    props.setChosens((prev) => {
      if (props.isMulti) {
        if (!prev.includes(member)) {
          return [member, ...prev];
        }
        return prev;
      }
      return [member];
    });
    if (!props.isMulti) {
      setTarget("");
    }
    refetch();
  };

  const resultOptionHandler = (chosen: Name) => {
    props.setChosens((prev) => prev.filter((c) => c.id !== chosen.id));
    refetch();
  };

  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  });

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
  });

  const viewMember = css({
    display: "inline-block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
    borderLeft: "solid 5px",
    borderRight: "solid 5px",
    borderBottom: "solid 5px",
    borderTop: "none",
    borderBottomLeftRadius: "20px",
    borderBottomRightRadius: "20px",
  });

  const inputStyle = css({
    display: "block",
    backgroundColor: "transparent",
    fontSize: "24px",
    width: "70%",
    padding: ".1em",
    margin: ".1em auto",
  });

  return (
    <div class={container}>
      <input
        placeholder={props.isMulti
          ? `${props.resultPlaceholder} : ${props.chosens.length}`
          : getChosenOne()}
        class={inputStyle}
        type="text"
        value={target()!}
        onInput={(e) => {
          setTarget(e.currentTarget.value);
          refetch();
        }}
      />
      <Show when={(target() || "").length > 0}>
        <section class={viewContainer}>
          <Show when={props.isMulti}>
            <select multiple size={props.chosens.length} class={viewMember}>
              {
                <For each={props.chosens}>
                  {(item) => (
                    <option onClick={() => resultOptionHandler(item)}>
                      {item.name}
                    </option>
                  )}
                </For>
              }
              <Show when={!props.chosens.length}>
                <option disabled>{props.nyMessage}</option>
              </Show>
            </select>
          </Show>
          <select
            multiple
            size={(optionsList() || []).length}
            class={viewMember}
          >
            {
              <For each={optionsList()}>
                {(item) => (
                  <option onClick={() => choiceOptionHandler(item)}>
                    {item.name}
                  </option>
                )}
              </For>
            }
            <Show when={!(optionsList() || []).length}>
              <option disabled>{props.mtMessage}</option>
            </Show>
          </select>
        </section>
      </Show>
    </div>
  );
}
