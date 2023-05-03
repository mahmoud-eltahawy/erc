import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createResource, createSignal, Show } from "solid-js";
import { css } from "solid-styled-components";
import { Note, ShiftProblem } from "../..";
import { employee, permissions } from "../../App";
import LongNote from "./longNote";
import ModfiyButtons from "./modifyButtons";
import Namer from "./Namer";
import { ReliableToggelableList } from "./sparePartsList";

async function shift_problem_fetcher({
  the_id,
}: {
  the_id: string;
}): Promise<SpreadedProblem> {
  const {
    begin_time,
    end_time,
    id,
    machine_id,
    maintainer_id,
    shift_id,
    writer_id,
  } = await invoke("get_shift_problem_by_id", { id: the_id })
    .catch((err) => console.log(err)) as ShiftProblem;

  const note = await invoke("get_shift_problem_note_by_id", { id })
    .catch((err) => console.log(err)) as string | null;

  const problems = await invoke("get_shift_problem_problems_ids_by_id", { id })
    .catch((err) => console.log(err)) as string[];

  const spare_parts = await invoke("get_shift_problem_spare_parts_ids_by_id", {
    id,
  })
    .catch((err) => console.log(err)) as string[] | null;

  return {
    id,
    shift_id,
    writer_id,
    machine_id,
    maintainer_id,
    begin_time,
    end_time,
    problems,
    spare_parts,
    note,
  };
}

type SpreadedProblem = {
  id: string;
  shift_id: string;
  writer_id: string;
  machine_id: string;
  maintainer_id: string;
  begin_time: string;
  end_time: string;
  problems: string[];
  spare_parts: string[] | null;
  note: string | null;
};
export default function ProblemRow({
  id,
  problemUpdating,
}: {
  id: string;
  problemUpdating: (id: string) => void;
}) {
  const [init] = createResource({ the_id: id }, shift_problem_fetcher);
  return (
    <Show when={init()}>
      {(notNullInit) => (
        <Core init={notNullInit()} problemUpdating={problemUpdating} />
      )}
    </Show>
  );
}

function Core({
  init,
  problemUpdating,
}: {
  init: SpreadedProblem;
  problemUpdating: (id: string) => void;
}) {
  let id = init.id;
  const [begin_time, set_begin_time] = createSignal(init.begin_time);
  const [end_time, set_end_time] = createSignal(init.end_time);
  const [machine_id, set_machine_id] = createSignal(init.machine_id);
  const [maintainer_id, set_maintainer_id] = createSignal(init.maintainer_id);
  const [problems, set_problems] = createSignal(init.problems);
  const [spare_parts, set_spare_parts] = createSignal(init.spare_parts || []);
  const [note, set_note] = createSignal(init.note);

  listen("update_shift_problem_begin_time", (e) => {
    let [problemId, begin_time] = e.payload as [string, string];
    if (id === problemId) {
      set_begin_time(begin_time);
    }
  });
  listen("update_shift_problem_end_time", (e) => {
    let [problemId, end_time] = e.payload as [string, string];
    if (id === problemId) {
      set_end_time(end_time);
    }
  });
  listen("update_shift_problem_maintainer", (e) => {
    let [problemId, maintainer_id] = e.payload as [string, string];
    if (id === problemId) {
      set_maintainer_id(maintainer_id);
    }
  });
  listen("update_shift_problem_machine", (e) => {
    let [problemId, machine_id] = e.payload as [string, string];
    if (id === problemId) {
      set_machine_id(machine_id);
    }
  });
  listen("update_shift_problem_add_problem", (e) => {
    let [sp_id, problem_id] = e.payload as [string, string];
    if (id === sp_id) {
      set_problems((prs) => [problem_id, ...prs]);
    }
  });
  listen("update_shift_problem_delete_problem", (e) => {
    let [sp_id, problem_id] = e.payload as [string, string];
    if (id === sp_id) {
      set_problems((prs) => prs.filter((x) => x !== problem_id));
    }
  });
  listen("update_shift_problem_add_spare_part", (e) => {
    let [sp_id, part_id] = e.payload as [string, string];
    if (id === sp_id) {
      set_spare_parts((prs) => [part_id, ...prs]);
    }
  });
  listen("update_shift_problem_delete_spare_part", (e) => {
    let [sp_id, part_id] = e.payload as [string, string];
    if (id === sp_id) {
      set_spare_parts((prs) => prs.filter((x) => x !== part_id));
    }
  });
  listen("update_shift_problem_add_note", (e) => {
    let [sp_id, note] = e.payload as [string, string];
    if (id === sp_id) {
      set_note(note);
    }
  });
  listen("update_shift_problem_update_note", (e) => {
    let [sp_id, note] = e.payload as [string, string];
    if (id === sp_id) {
      set_note(note);
    }
  });
  listen("update_shift_problem_delete_note", (e) => {
    let sp_id = e.payload as string;
    if (id === sp_id) {
      set_note("");
    }
  });
  const machine_name = () => {
    let id = machine_id();
    return (
      <Namer
        command="get_machine_name_by_id"
        id={() => id}
      />
    );
  };
  const maintainer_name = () => {
    let id = maintainer_id();
    return (
      <Namer
        command="employee_name"
        id={() => id}
      />
    );
  };

  const [hover, setHover] = createSignal(false);

  const style = () => {
    const padding = () => hover() ? "17px" : "7px";
    const sides = () => hover() ? "none" : "solid 2px";
    const sandwitch = () => hover() ? "solid 7px" : "dotted 1px";
    return css({
      paddingLeft: padding(),
      paddingRight: padding(),
      borderRight: sides(),
      borderLeft: sides(),
      borderBottom: sandwitch(),
      borderTop: sandwitch(),
    });
  };

  return (
    <tr>
      <td
        class={style()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
      >
        {hover()
          ? (
            <ModfiyButtons
              permission={() =>
                permissions()?.includes("ModifyDepartmentProblems")}
              setUpdating={() => problemUpdating(id)}
              deleteFunc={async () =>
                invoke("remove_shift_problem", {
                  problemId: init.id,
                  updaterId: employee()?.id,
                })
                  .catch((err) => console.log(err))}
            />
          )
          : (
            <p>
              <Namer command="employee_name" id={() => init.writer_id} />
            </p>
          )}
      </td>
      <Show
        when={note()}
        fallback={
          <td>
            <p>لا يوجد ملحوظات اضافية</p>
          </td>
        }
      >
        {(notNullNote) => (
          <td class={style()}>
            <LongNote content={() => notNullNote()} />
          </td>
        )}
      </Show>
      <td class={style()}>
        {`من ${begin_time().slice(0, 5)} الي ${end_time().slice(0, 5)}`}
      </td>
      <Show
        when={spare_parts()?.length}
        fallback={
          <td>
            <p>لم تستخدم اي قطعة غيار</p>
          </td>
        }
      >
        {
          <td
            class={style()}
          >
            <ReliableToggelableList
              command="get_spare_part_name_by_id"
              ids={() => spare_parts() || []}
            />
          </td>
        }
      </Show>
      <Show when={problems()}>
        {(notNullProblems) => (
          <td
            class={style()}
          >
            <ReliableToggelableList
              command="get_problem_name_by_id"
              ids={() => notNullProblems()}
            />
          </td>
        )}
      </Show>
      <td class={style()}>{maintainer_name()}</td>
      <td class={style()}>{machine_name()}</td>
    </tr>
  );
}
