import ShiftIdentity from "./components/molecules/ShiftIdentity";
import Wall from "./wall";
import { createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { Employee } from "./index";
import { listen } from "@tauri-apps/api/event";
import { css } from "solid-styled-components";
import SubmitButton from "./components/atoms/submitButton";

export const [employee, setEmployee] = createSignal<Employee | null>(null);
export const [shiftId, setShiftId] = createSignal<string | null>(null);
export const [permissions, setPermissions] = createSignal<string[]>([]);

listen("update_employee_allow_permission", (e) => {
  const [id, permission] = e.payload as [string, string];
  if (employee()?.id === id) {
    setPermissions((permissions) => [permission, ...permissions]);
  }
});

listen("update_employee_forbid_permission", (e) => {
  const [id, permission] = e.payload as [string, string];
  if (employee()?.id === id) {
    setPermissions((permissions) =>
      permissions.filter((x) => x !== permission)
    );
  }
});

listen("update_employee_forbid_all_permissions", (e) => {
  const id = e.payload as string;
  if (employee()?.id === id) {
    setPermissions([]);
  }
});

listen("update_employee_up", (e) => {
  const id = e.payload as string;
  if (employee()?.id === id) {
    setEmployee((emp) => {
      return { ...emp!, position: "SUPER_USER" };
    });
  }
});

listen("update_employee_down", (e) => {
  const id = e.payload as string;
  if (employee()?.id === id) {
    setEmployee((emp) => {
      return { ...emp!, position: "USER" };
    });
  }
});

const isLogedIn = async function () {
  try {
    const [employeeId, shiftId] = await invoke("check_login") as [
      string,
      string,
    ];
    const employee = await invoke("get_employee_by_id", {
      id: employeeId,
    }) as Employee;
    const permissions = await invoke("employee_permissions", {
      id: employeeId,
    }) as string[];
    setEmployee(employee);
    setShiftId(shiftId);
    setPermissions(permissions);
  } catch (err) {
    console.log(err);
  }
};

function App() {
  onMount(() => {
    isLogedIn();
    invoke("update");
  });

  listen("shift_ended", () => {
    setPermissions([]);
  });

  listen("new_login", () => isLogedIn());

  return (
    <section>
      <ShiftIdentity />
      <MajorUpdate />
      <Show
        when={shiftId() && employee()}
        fallback={<LoginForm />}
      >
        <Wall rank={1} />
      </Show>
    </section>
  );
}
function LoginForm() {
  let cardR: HTMLInputElement | undefined;
  let passwordR: HTMLInputElement | undefined;

  async function handleSubmit(e: Event) {
    e.preventDefault();
    await invoke("login", {
      cardId: +cardR!.value,
      password: passwordR!.value,
    });
    const employee_and_id = await invoke("check_login");
    const [employeeId, shift_id] = employee_and_id as [string, string];
    const employee = await invoke("get_employee_by_id", {
      id: employeeId,
    }) as Employee;
    setEmployee(employee);
    setShiftId(shift_id);
    passwordR!.value = "";
    cardR!.value = "";
    cardR!.focus();
  }

  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  });

  const inputStyle = css({
    display: "block",
    width: "50%",
    fontSize: "20px",
    padding: ".5em",
    margin: ".3em auto",
    backgroundColor: "beige",
    border: "solid 3px",
  });

  return (
    <section class={container}>
      <form onSubmit={handleSubmit}>
        <input
          ref={cardR}
          class={inputStyle}
          type="number"
          placeholder="رقم التعريف"
          required
        />
        <input
          ref={passwordR}
          class={inputStyle}
          type="password"
          placeholder="كلمة السر"
          required
        />
        <SubmitButton length={undefined} />
      </form>
    </section>
  );
}

function MajorUpdate() {
  const [successAndFailures, setSucessesAndFailures] = createSignal([0, 0]);
  const [updatesNumber, setUpdatesNumber] = createSignal<number | null>(null);

  listen("begin_major_update", (e) => setUpdatesNumber(e.payload as number));
  listen("end_major_update", () => setUpdatesNumber(null));
  listen(
    "major_update_step",
    (e) => setSucessesAndFailures([...e.payload as [number, number]]),
  );

  const style = css({
    display: "block",
    backgroundColor: "lightyellow",
    border: "2px solid",
    borderRadius: "200px",
    position: "absolute",
    left: "15%",
    top: "15%",
    width: "70%",
    height: "70%",
  });
  const elementStyle = css({
    fontSize: "25px",
    margin: "5%",
    padding: "1%",
  });
  return (
    <Show when={(updatesNumber() || 0) > 50 && updatesNumber()}>
      {(notNullUpdatesNumber) => (
        <section class={style}>
          <h1 class={elementStyle}>تحديث بيانات</h1>
          <p class={elementStyle}>
            البيانات المتاحة : {notNullUpdatesNumber()}
          </p>
          <p class={elementStyle}>
            تمت محاولة تحديث :{" "}
            {successAndFailures().at(0)! + successAndFailures().at(1)!}
          </p>
          <p class={elementStyle}>
            {"التحديثات الناجحة الي البيانات المتاحة : " +
              `${successAndFailures().at(0)} الي ${notNullUpdatesNumber()} ` +
              `(${
                Math.round(
                  (successAndFailures().at(0)! / notNullUpdatesNumber()) * 100,
                )
              } %) `}
          </p>
          <p class={elementStyle}>
            {"التحديثات الفاشلة الي البيانات المتاحة : " +
              `${successAndFailures().at(1)} الي ${notNullUpdatesNumber()} ` +
              `(${
                Math.round(
                  (successAndFailures().at(1)! / notNullUpdatesNumber()) * 100,
                )
              } %) `}
          </p>
        </section>
      )}
    </Show>
  );
}

export default App;
