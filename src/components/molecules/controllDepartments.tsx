import { invoke } from "@tauri-apps/api";
import { createResource, createSignal, For, Show } from "solid-js";
import { css } from "solid-styled-components";
import {
  departmentsNames,
  Name,
  NativeDepartment,
  PermissionsClassified,
} from "../..";
import PermissionsTemplate from "../atoms/permissionsTemplate";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function ControllDepartments(props: { rank: number }) {
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <Show when={departmentsNames()}>
      {(notNullDepartments) => (
        <section class={container}>
          {
            <ButtonsOrElementLite
              rank={props.rank}
              buttonElementPairs={() =>
                notNullDepartments()
                  .filter((d) =>
                    d.id !== "00000000-0000-0000-0000-000000000000"
                  )
                  .map(
                    (d) => [
                      d.name,
                      <DepartmentSettings
                        rank={props.rank + 1}
                        departmentId={d.id}
                      />,
                    ],
                  )}
            />
          }
        </section>
      )}
    </Show>
  );
}

type Department = {
  id: string;
  boss: Name | null;
  name: string;
  employees: Name[];
};

const department_fetcher = async (
  { departmentId }: { departmentId: string },
) => {
  let department: Department;
  const nd =
    (await invoke("find_department", { id: departmentId })) as NativeDepartment;
  const employees =
    (await invoke("department_employees", { id: departmentId })) as Name[];
  if (nd.boss_id) {
    const name: string =
      (await invoke("employee_name", { id: nd.boss_id })) as string;
    const boss: Name = { id: nd.boss_id, name };
    department = { id: nd.id, boss, name: nd.name, employees };
  } else {
    department = { id: nd.id, name: nd.name, boss: null, employees };
  }
  return department;
};

const department_permissions_fetcher = async (
  { departmentId }: { departmentId: string },
) => {
  const [id, allowed, forbidden] = await invoke("department_permissions", {
    departmentId,
  })
    .catch((err) => {
      console.log(err);
      return [];
    }) as [
      string | null,
      [string, string][],
      [string, string][],
    ];
  return { id, allowed, forbidden } as PermissionsClassified;
};
function DepartmentSettings(
  props: { departmentId: string; rank: number },
) {
  const [permissions, dbf] = createResource(
    { departmentId:props.departmentId },
    department_permissions_fetcher,
  );
  const [department, { refetch }] = createResource(
    { departmentId:props.departmentId },
    department_fetcher,
  );

  const allowedHandler = async (employeeId: string, permission: string) => {
    await invoke("permission_forbid", {
      employeeId,
      permission,
    })
      .catch((err) => console.log(err));
    dbf.refetch();
  };

  const forbiddenHandler = async (employeeId: string, permission: string) => {
    await invoke("permission_allow", {
      employeeId,
      permission,
    })
      .catch((err) => console.log(err));
    dbf.refetch();
  };
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <Show when={department()}>
        {(notNullDepartment) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() => [
              [
                "اختيار رئيس القسم",
                <ChooseBoss
                  department={() => notNullDepartment()}
                  refetch={() => refetch()}
                />,
              ],
              [
                "صلاحيات القسم",
                <PermissionsTemplate
                  allowedHandler={allowedHandler}
                  forbiddenHandler={forbiddenHandler}
                  permissions={() => permissions()!}
                />,
              ],
            ]}
          />
        )}
      </Show>
    </section>
  );
}

function ChooseBoss(props: {
    department: () => Department;
    refetch: () => void;
  },
) {
  const [target, setTarget] = createSignal<string>("");

  const optionHandler = async (newBossId: string) => {
    await invoke("boss_employee", { newBossId });
    props.refetch();
  };

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

  const filtered = () =>
    props.department().employees.filter((m) => m.name.includes(target()!));

  return (
    <section>
      <h1 class={css({ fontSize: "20px" })}>
        رئيس القسم :{" "}
        {props.department().boss?.name ? props.department().boss?.name : "لا يوجد"}
      </h1>
      <input
        class={inputStyle}
        type="text"
        value={target()}
        onInput={(e) => {
          setTarget(e.currentTarget.value);
        }}
      />
      <select multiple size={filtered().length} class={viewMember}>
        <For each={filtered()}>
          {(item) => (
            <option onClick={() => optionHandler(item.id)}>{item.name}</option>
          )}
        </For>
      </select>
    </section>
  );
}
