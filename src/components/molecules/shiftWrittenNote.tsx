import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import {
  createEffect,
  createResource,
  createSignal,
  For,
  Show,
} from "solid-js";
import { css } from "solid-styled-components";
import { Employee } from "../..";
import { employee, permissions } from "../../App";
import ModfiyButtons from "../atoms/modifyButtons";
import ProblemRow from "../atoms/problemRow";
import TableHead from "../atoms/problemTableHead";
import togglingButton from "../atoms/problemTogglingButton";
import { ProblemUpdateForm } from "../organisms/ProblemForm";
import { UpdateShiftNote } from "./AddShiftNote";
import { ButtonsOrElementLite } from "./buttonsOrElement";
import { existing_employees_fetcher } from "./setShiftEmployees";

export default function ShiftWrittenShow({
  rank,
  shiftId,
}: {
  rank: number;
  shiftId: () => string;
}) {
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <ButtonsOrElementLite
        rank={rank}
        buttonElementPairs={() => [
          [
            "اظهار الاعطال",
            <ShiftProblems
              shiftId={shiftId()}
            />,
          ],
          [
            "اظهار الملحوظات",
            <ShiftNotes
              shiftId={shiftId()}
            />,
          ],
          [
            "اظهار الموظفين",
            <ExistingEmployees
              shiftId={shiftId()}
            />,
          ],
        ]}
      />
    </section>
  );
}

function ExistingEmployees({
  shiftId,
}: {
  shiftId: string;
}) {
  const [existing, { refetch }] = createResource(
    { shift_id: () => shiftId },
    existing_employees_fetcher,
  );

  listen("update_department_shift_employee", () => refetch());

  const viewMember = css({
    display: "block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
  });

  return (
    <ol class={viewMember}>
      <For each={existing()}>
        {(item) => <li>{item.name}</li>}
      </For>
      <Show when={!(existing() || []).length}>
        <li>لا يوجد موظفين مسجلين</li>
      </Show>
    </ol>
  );
}

export type ShiftNote = {
  id: string;
  shift_id: string;
  writer_id: string;
  content: string;
};

function ShiftNotes({
  shiftId,
}: {
  shiftId: string;
}) {
  const notes_ids_fetcher = async ({ shiftId }: { shiftId: string }) => {
    return (await invoke("fetch_shift_notes_ids", { shiftId })
      .catch((err) => console.log(err))) as string[];
  };
  const [notesIds, { refetch, mutate }] = createResource(
    { shiftId },
    notes_ids_fetcher,
  );

  const LIMIT = 4;

  let the_shift_note: ShiftNote | undefined = undefined;
  const get_the_shift_note = () => the_shift_note;

  const [state, setState] = createSignal<string[] | undefined>([]);
  const [tooLong, setTooLong] = createSignal((state() || []).length < LIMIT);
  const [updating, setUpdating] = createSignal(false);

  createEffect(() => {
    if (tooLong()) {
      if (notesIds()) {
        setState(notesIds()!.slice(0, LIMIT));
      } else {
        setState(undefined);
      }
    } else {
      setState(notesIds());
    }
  });

  listen("create_shift_note", (e) => {
    let [shift_id, note_id] = e.payload as [string, string];
    if (shift_id === shiftId) {
      if (tooLong()) {
        mutate((ids) => [note_id, ...(ids || [])]);
      } else {
        refetch();
      }
    }
  });
  listen("delete_shift_note", (e) => {
    let [shift_id, note_id] = e.payload as [string, string];
    if (shift_id === shiftId) {
      if (tooLong()) {
        mutate((ids) => [...(ids || []).filter((id) => id !== note_id)]);
      } else {
        refetch();
      }
    }
  });

  function Note({ id }: { id: string }) {
    const note_fetcher = async ({ id }: { id: string }) => {
      return (await invoke("fetch_shift_note", { id })
        .catch((err) => console.log(err))) as ShiftNote;
    };

    const [note, { refetch }] = createResource({ id }, note_fetcher);

    listen("update_shift_note", (e) => {
      if (e.payload === id) {
        refetch();
      }
    });

    const style = css({
      padding: "5px",
      margin: "3px",
      borderBottom: "2px solid",
    });

    const noteStyle = css({
      padding: "5px",
      margin: "3px",
      width: "60%",
      borderLeft: "7px solid",
      borderBottom: "2px solid",
    });

    function EmployeeName({ id }: { id: string }) {
      const fetcher = async ({ id }: { id: string }) => {
        return (await invoke("get_employee_by_id", { id })
          .catch((err) => console.log(err))) as Employee;
      };
      const [employee] = createResource({ id }, fetcher);

      return (
        <Show when={employee()}>
          {(e) => <p>{e().first_name} {e().middle_name} {e().last_name}</p>}
        </Show>
      );
    }

    return (
      <Show when={note()}>
        {(n) => (
          <tr>
            <td class={style}>
              <ModfiyButtons
                setUpdating={() => {
                  the_shift_note = note();
                  setUpdating(true);
                }}
                deleteFunc={async () =>
                  invoke("remove_shift_note", {
                    shiftId,
                    noteId: n().id,
                    updaterId: employee()!.id,
                  })
                    .catch((err) => console.log(err))}
                permission={() => true}
              />
            </td>
            <td class={style}>
              <EmployeeName id={n().writer_id} />
            </td>
            <td class={noteStyle}>{n().content}</td>
          </tr>
        )}
      </Show>
    );
  }

  const style = css({
    borderCollapse: "collapse",
    width: "99%",
  });

  return (
    <section>
      <Show
        when={!updating()}
        fallback={
          <UpdateShiftNote
            toggle={() => setUpdating(false)}
            note={() => get_the_shift_note()!}
          />
        }
      >
        <table class={style}>
          <thead>
            <tr>
              <th>تغيير</th>
              <th>الكاتب</th>
              <th>الملحوظة</th>
            </tr>
          </thead>
          <tbody>
            <For each={state()}>
              {(noteId) => (
                <Note
                  id={noteId}
                />
              )}
            </For>
          </tbody>
        </table>
        {togglingButton({
          showButton: () => (notesIds() || []).length > LIMIT,
          showMore: () => tooLong(),
          doOnClick: () => setTooLong(!tooLong()),
        })}
      </Show>
    </section>
  );
}

export const shift_shift_problems_ids_fetcher = async ({
  id,
}: {
  id: string;
}) => {
  return (await invoke("get_shift_problems_ids_by_shift_id", {
    id,
  })) as string[];
};

function ShiftProblems({
  shiftId,
}: {
  shiftId: string;
}) {
  const limit = 4;
  let problem_to_update: string;
  const get_problem_to_update = () => problem_to_update;

  const [updatating, setUpdating] = createSignal(false);
  const [shiftProblemsIds, { refetch, mutate }] = createResource({
    id: shiftId,
  }, shift_shift_problems_ids_fetcher);
  const [state, setState] = createSignal<string[]>([]);
  const [tooLong, setTooLong] = createSignal((state() || []).length < limit);

  createEffect(() => {
    if (tooLong()) {
      if (shiftProblemsIds()) {
        setState(shiftProblemsIds()!.slice(0, limit));
      } else {
        setState([]);
      }
    } else {
      setState(shiftProblemsIds() || []);
    }
  });

  listen("delete_shift_problem", (e) => {
    let [shift_id, problemId] = e.payload as [string, string];
    if (shift_id === shiftId) {
      if ((shiftProblemsIds() || []).length > limit) {
        mutate((list) => list!.filter((x) => x !== problemId));
      } else {
        refetch();
      }
    }
  });
  listen("create_shift_problem", (e) => {
    let [shift_id, problemId] = e.payload as [string, string];
    if (shift_id === shiftId) {
      setTimeout(() => {
        if ((shiftProblemsIds() || []).length > limit) {
          mutate((list) => [problemId, ...(list || [])]);
        } else {
          refetch();
        }
      }, 1000);
    }
  });

  const style = css({
    borderCollapse: "collapse",
    width: "99%",
  });

  return (
    <section>
      <Show
        when={permissions()?.includes("ReadDepartmentProblems")}
        fallback={<h1>ليس لديك صلاحية قراءة اعطال الوردية</h1>}
      >
        <Show
          when={!updatating()}
          fallback={
            <ProblemUpdateForm
              id={get_problem_to_update()!}
              toggle={() => setUpdating(false)}
            />
          }
        >
          <table class={style}>
            <TableHead />
            <Show when={state()} fallback={<h1>جاري التحميل ...</h1>}>
              {(notNullIdList) => (
                <tbody>
                  <For each={notNullIdList()}>
                    {(id) => (
                      <ProblemRow
                        problemUpdating={(values) => {
                          problem_to_update = values;
                          setUpdating(true);
                        }}
                        id={id}
                      />
                    )}
                  </For>
                </tbody>
              )}
            </Show>
          </table>
        </Show>
        {togglingButton({
          showButton: () => (shiftProblemsIds() || []).length > limit,
          showMore: () => tooLong(),
          doOnClick: () => setTooLong(!tooLong()),
        })}
      </Show>
    </section>
  );
}
