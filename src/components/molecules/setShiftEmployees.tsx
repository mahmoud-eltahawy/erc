import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createResource, createSignal, For, Show } from "solid-js";
import { css } from "solid-styled-components";
import { employee, shiftId } from "../../App";
import Namer from "../atoms/Namer";

const non_existing_fetcher = async () => {
  return (await invoke("shift_non_existing_employees", {
    shiftId: shiftId(),
    departmentId: employee()!.department_id,
  })) as string[];
};

export const existing_employees_fetcher = async (
  props: { shift_id: () => string | null },
) => {
  return (await invoke("shift_existing_employees", {
    shiftId: props.shift_id(),
  })) as string[];
};

export default function SetShiftEmployees() {
  const [target, setTarget] = createSignal<string | null>(null);
  const [nonExisting, nex] = createResource(non_existing_fetcher);
  const [existing, ex] = createResource(
    { shift_id: shiftId },
    existing_employees_fetcher,
  );

  listen("update_shift_add_employee", (e) => {
    const [shift_id, emp_id] = e.payload as [string, string];
    if (shift_id === shiftId()) {
      nex.mutate((emps) => (emps || []).filter((x) => x !== emp_id));
      if (!existing()?.includes(emp_id)) {
        ex.mutate((emps) => [emp_id, ...(emps || [])]);
      }
    }
  });

  listen("update_shift_delete_employee", (e) => {
    const [shift_id, emp_id] = e.payload as [string, string];
    if (shift_id === shiftId()) {
      ex.mutate((emps) => (emps || []).filter((x) => x !== emp_id));
      if (!existing()?.includes(emp_id)) {
        nex.mutate((emps) => [emp_id, ...(emps || [])]);
      }
    }
  });

  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  });

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
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
        placeholder={"ابحث عن موظف للتسجيل"}
        class={inputStyle}
        type="text"
        value={target()!}
        onInput={(e) => {
          setTarget(e.currentTarget.value);
          nex.refetch();
        }}
      />
      <section class={viewContainer}>
        <ExistingSection existing={() => existing() || []} />
        <NonExistingSection nonExisting={() => nonExisting() || []} />
      </section>
    </div>
  );
}

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

function ExistingSection(props: {
  existing: () => string[];
}) {
  const handler = async ({ employeeId }: { employeeId: string }) => {
    await invoke("remove_shift_employee", {
      employeeId,
    });
  };

  return (
    <select multiple size={props.existing().length} class={viewMember}>
      {
        <For each={props.existing()}>
          {(item) => (
            <option onClick={() => handler({ employeeId: item })}>
              <Namer
                command="employee_name"
                id={() => item}
              />
            </option>
          )}
        </For>
      }
      <Show when={!(props.existing() || []).length}>
        <option disabled>لا يوجد موظفين مسجلين</option>
      </Show>
    </select>
  );
}

function NonExistingSection(props: {
  nonExisting: () => string[];
}) {
  const handler = async ({ employeeId }: { employeeId: string }) => {
    await invoke("add_shift_employee", {
      employeeId,
    });
  };

  return (
    <select multiple size={props.nonExisting().length} class={viewMember}>
      {
        <For each={props.nonExisting()}>
          {(item) => (
            <option onClick={() => handler({ employeeId: item })}>
              <Namer command="employee_name" id={() => item} />
            </option>
          )}
        </For>
      }
      <Show when={!(props.nonExisting() || []).length}>
        <option disabled>لا يوجد موظفين</option>
      </Show>
    </select>
  );
}
