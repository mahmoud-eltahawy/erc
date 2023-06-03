import { invoke } from "@tauri-apps/api";
import { createResource, createSignal, Show } from "solid-js";
import HistoryShow from "./components/organisms/HistoryShow";
import { NativeDepartment } from "./index";
import { css } from "solid-styled-components";
import Controlling from "./components/organisms/controlling";
import CurrentShiftData from "./components/organisms/CurrentShiftData";
import { employee, setEmployee, setShiftId } from "./App";
import { ButtonsOrElementLite } from "./components/molecules/buttonsOrElement";
import { listen } from "@tauri-apps/api/event";
import NavBar, { setButtons } from "./navBar";

const [changePassword, setChangePassword] = createSignal(false);

export default function Wall(props: { rank: number }) {
  setInterval(() => {
    invoke("check_shift_time", {
      departmentId: employee()!.department_id,
    })
      .catch((err) => console.log(err));
  }, 60000);

  return (
    <section>
      <AboutParagraph />
      <LogoutButton />
      <PasswordUpdate />
      <NavBar />
      {employee()?.card_id === 0
        ? <Controlling rank={props.rank} />
        : (
          <ButtonsOrElementLite
            rank={props.rank}
            buttonElementPairs={() => [
              ["التحكم", <Controlling rank={props.rank + 1} />],
              [
                "بيانات الوردية الحالية",
                <CurrentShiftData rank={props.rank + 1} />,
              ],
              ["السجل", <HistoryShow rank={props.rank + 1} />],
            ]}
          />
        )}
    </section>
  );
}

const department_fetcher = async ({ id }: { id: string }) => {
  return (await invoke("find_department", { id })) as NativeDepartment;
};

function AboutParagraph() {
  const [hover, setHover] = createSignal(false);
  const [department] = createResource(
    { id: employee()!.department_id },
    department_fetcher,
  );

  const position = () => {
    const superiority = employee()!.position === "SUPER_USER"
      ? "مشرف"
      : "مستخدم";
    const is_boss = employee()!.id === department()?.boss_id
      ? " و رئيس القسم"
      : "";
    return superiority + is_boss;
  };

  const container = () =>
    css({
      backgroundColor: hover() ? "lightyellow" : "transparent",
      position: "absolute",
      top: "0px",
      left: "0px",
      width: hover() ? "35%" : "15%",
      height: hover() ? "25%" : "8%",
      padding: ".5em",
      borderRight: "2px solid",
      borderBottom: "2px solid",
      borderTopRightRadius: "20px",
      borderBottomLeftRadius: "20px",
    });
  return (
    <div
      onMouseOver={() =>
        employee()!.card_id !== 0 ? setHover(true) : setHover(false)}
      onMouseLeave={() => setHover(false)}
      class={container()}
    >
      <Show
        when={employee()!.card_id !== 0}
        fallback={<h1>الحساب الرئيسي</h1>}
      >
        <p>
          {`الاسم : ${employee()!.first_name} ${employee()!.middle_name} ${
            employee()!.last_name
          }`}
        </p>
        <Show when={department()}>
          {(notNullDepartment) => (
            <Show when={hover()}>
              <p>{`رقم التعريف : ${employee()!.card_id}`}</p>
              <p>{`القسم : ${notNullDepartment().name}`}</p>
              <p>{`الرتبة : ${position()}`}</p>
              <button
                class={css({
                  fontSize: "25px",
                  margin: "2%",
                  padding: "1%",
                })}
                onClick={() => setChangePassword(true)}
              >
                تغيير كلمة السر
              </button>
            </Show>
          )}
        </Show>
      </Show>
    </div>
  );
}

function LogoutButton() {
  const [hover, setHover] = createSignal(false);

  const logout = () => {
    invoke("logout")
      .catch((err) => console.log(err));
  };

  listen("logout", () => {
    setButtons([]);
    setEmployee(null);
    setShiftId(null);
  });

  listen("shift_ended", () => logout());

  const style = () =>
    css({
      position: "absolute",
      fontSize: hover() ? "20px" : "16px",
      top: "0px",
      right: "0px",
      width: "15%",
      padding: ".5em",
      borderBottom: "2px solid",
      borderLeft: "2px solid",
      borderTop: "none",
      borderTopLeftRadius: "20px",
    });

  return (
    <button
      class={style()}
      onClick={() => logout()}
      onMouseOver={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      تسجيل خروج
    </button>
  );
}

function PasswordUpdate() {
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
  const inputStyle = css({
    fontSize: "20px",
    margin: "5%",
    padding: "1%",
  });
  const buttonStyle = css({
    fontSize: "25px",
    margin: "2%",
    padding: "1%",
  });
  let old_password: HTMLInputElement | undefined = undefined;
  let new_password1: HTMLInputElement | undefined = undefined;
  let new_password2: HTMLInputElement | undefined = undefined;

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (new_password1?.value === new_password2?.value) {
      await invoke("change_password", {
        employeeId: employee()?.id,
        oldPassword: old_password?.value,
        newPassword: new_password1?.value,
      }).catch((err) => alert(err));
      setChangePassword(false);
    } else {
      alert("كلمة السر الجديدة غير متطابقة");
    }
  };

  return (
    <Show when={changePassword()}>
      <section class={style}>
        <form onSubmit={handleSubmit}>
          <input
            ref={old_password}
            class={inputStyle}
            type="password"
            placeholder="كلمة السر الحالية"
          />
          <br />
          <input
            ref={new_password1}
            class={inputStyle}
            type="password"
            placeholder="كلمة السر الجديدة"
          />
          <br />
          <input
            ref={new_password2}
            class={inputStyle}
            type="password"
            placeholder="كلمة السر الجديدة مرة اخري"
          />
          <br />
          <button class={buttonStyle} type="submit">تاكيد</button>
          <button
            class={buttonStyle}
            type="reset"
            onClick={() => setChangePassword(false)}
          >
            الغاء
          </button>
        </form>
      </section>
    </Show>
  );
}
