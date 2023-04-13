import { invoke } from "@tauri-apps/api"
import { createResource, createSignal, Show } from "solid-js"
import HistoryShow from "./components/organisms/HistoryShow"
import { NativeDepartment } from "./index"
import { css } from "solid-styled-components"
import Controlling from "./components/organisms/controlling"
import CurrentShiftData from "./components/organisms/CurrentShiftData"
import { employee, setEmployee, setShiftId } from "./App"
import { ButtonsOrElementLite } from "./components/molecules/buttonsOrElement"
import { listen } from "@tauri-apps/api/event"

export default function Wall(){
  setInterval(() => {
    invoke('check_shift_time', { departmentId: employee()!.department_id })
      .catch(err => console.log(err))
  }, 60000)

  return (
    <section>
      <AboutParagraph/>
      <LogoutButton/>
      {employee()?.card_id === 0 ? <Controlling/> :
        <ButtonsOrElementLite
            returnButtonText="الصفحة الرئيسية"
            buttonElementPairs={() => [
                  ["التحكم", <Controlling/>],
                  ["بيانات الوردية الحالية", <CurrentShiftData/>],
                  ["السجل", <HistoryShow/>]
                ]
            }/>}
    </section>
  )
}

const department_fetcher = async ({id} : {id : string}) => {
  return (await invoke("find_department",{id})) as NativeDepartment
}

function AboutParagraph(){
  const [hover,setHover] = createSignal(false)
  const [department] = createResource({id : employee()!.department_id},department_fetcher)

  const position = () => {
    const superiority = employee()!.position === "SUPER_USER" ? "مشرف" : "مستخدم"
    const is_boss = employee()!.id === department()?.boss_id ? " و رئيس القسم" : ""
    return superiority + is_boss
  }

  const container = () => css({
    backgroundColor: hover() ? "lightyellow" : "transparent",
    position: "absolute",
    top: "0px",
    left: "0px",
    width: hover() ? "35%" : "15%" ,
    padding: ".5em",
    borderRight: "2px solid",
    borderBottom: "2px solid",
    borderTopRightRadius: "20px",
    borderBottomLeftRadius: "20px",
  })
  return (
  <div
      onMouseOver={() => employee()!.card_id !== 0 ? setHover(true) : setHover(false)}
      onMouseLeave={() => setHover(false)}
      class={container()}>
    <Show
        when={employee()!.card_id !== 0}
        fallback={<h1>الحساب الرئيسي</h1>}>
      <p>{`الاسم : ${employee()!.first_name} ${employee()!.middle_name} ${employee()!.last_name}`}</p>
      <Show when={department()}>
        {notNullDepartment => <Show when={hover()}>
          <p>{`رقم التعريف : ${employee()!.card_id}`}</p>
          <p>{`القسم : ${notNullDepartment().name}`}</p>
          <p>{`الرتبة : ${position()}`}</p>
        </Show>}
      </Show>
    </Show>
  </div>
  )
}

function LogoutButton(){
  const [hover, setHover] = createSignal(false)

  const logout = () => {
    invoke('logout')
    .catch(err => console.log(err))
  }

  listen("logout",() => {
    setEmployee(null)
    setShiftId(null)
  })

  listen("shift_ended",() => logout())

  const style = () => css({
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
  })

  return (
    <button
        class={style()}
        onClick={() => logout()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
    >تسجيل خروج</button>
  )
}
