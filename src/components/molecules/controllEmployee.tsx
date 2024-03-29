import { invoke } from "@tauri-apps/api";
import {
  Accessor,
  createEffect,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import { css } from "solid-styled-components";
import { departmentsNames, Name, PermissionsClassified } from "../..";
import PermissionsTemplate from "../atoms/permissionsTemplate";
import ShowAllToggleButton from "../atoms/showAllToggleButton";
import { ButtonsOrElementLite } from "./buttonsOrElement";

export default function ControllEmployees(props: { rank: number }) {
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
      <div class={container}>
        <input
          value={target()!}
          onInput={(e) => setTarget(e.currentTarget.value)}
          class={targetStyle}
          type="text"
          placeholder="ادخل اسم الموظف"
          required
        />
      </div>
      <ShowAllToggleButton target={target} toggle={toggle} />
      <Show when={departmentsNames()}>
        {(notNullDepartments) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() =>
              notNullDepartments()
                .map((d) => [
                  d.name,
                  <DepartmentEmployees
                    rank={props.rank + 1}
                    target={target}
                    departmentId={d.id}
                  />,
                ])}
          />
        )}
      </Show>
    </section>
  );
}

const department_employees_names_fetcher = async (
  { name, departmentId }: { name: () => string | null; departmentId: string },
) => {
  return (await invoke("search_department_employees", {
    name: name() !== " " ? name() : null,
    departmentId,
  })) as Name[];
};

function DepartmentEmployees(props: {
  target: Accessor<string | null>;
  departmentId: string;
  rank: number;
}) {
  const [employees, { refetch }] = createResource({
    name: () => props.target(),
    departmentId: props.departmentId,
  }, department_employees_names_fetcher);

  createEffect(() => {
    if (props.target()) {
      refetch();
    }
  });

  return (
    <section>
      <Show when={employees()} fallback={<h1>جاري التحميل ...</h1>}>
        {(notNullEmployees) => (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() =>
              notNullEmployees()
                .map(
                  (x) => [x.name, <EmployeePermissions employeeId={x.id} />],
                )}
          />
        )}
      </Show>
    </section>
  );
}

const employee_permissions_fetcher = async (props: {
  employeeId: string;
}) => {
  const [id, allowed, forbidden] = await invoke(
    "employee_permissions_classified",
    { employeeId: props.employeeId },
  )
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

function EmployeePermissions(props: { employeeId: string }) {
  const [permissions, { refetch }] = createResource(
    { employeeId: props.employeeId },
    employee_permissions_fetcher,
  );

  const allowedHandler = async (employeeId: string, permission: string) => {
    await invoke("permission_forbid", {
      employeeId,
      permission,
    });
    refetch();
  };

  const forbiddenHandler = async (employeeId: string, permission: string) => {
    await invoke("permission_allow", {
      employeeId,
      permission,
    });
    refetch();
  };

  return (
    <section>
      <Show when={permissions()}>
        {(notNullPermissions) => (
          <PermissionsTemplate
            allowedHandler={allowedHandler}
            forbiddenHandler={forbiddenHandler}
            permissions={() => notNullPermissions()}
          />
        )}
      </Show>
    </section>
  );
}
