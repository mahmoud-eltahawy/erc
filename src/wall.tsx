import { invoke } from "@tauri-apps/api"
import { createSignal, Setter } from "solid-js"
import DefineProblem from "./components/molecules/defineProblem"
import HistoryShow from "./components/organisms/HistoryShow"
import { Employee } from "./index"
import ProblemForm from "./components/organisms/ProblemForm"
import ShiftProblems from "./components/organisms/ShiftProblems"
import { ButtonsOrElement } from "./components/molecules/buttonsOrElement"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"

export default function Wall({
          employee,
          shiftId,
          setEmployee,
          setShiftId
        } : {
          employee : Employee,
          shiftId  : string,
          setEmployee : Setter<Employee | null>,
          setShiftId  : Setter<string | null>
        }){
  const [last,setLast] = createStore([-1])

  setInterval(() => {
    invoke('check_shift_time',{departmentId : employee.department_id})
      .then(() => console.log("employee checked"))
      .catch(err => console.log(err))
  },60000)

  return (
    <section>
      <AboutParagraph employee={employee} />
      <LogoutButton setEmployee={setEmployee} setShiftId={setShiftId} />
      <ButtonsOrElement returnButtonText="الصفحة الرئيسية"
            buttonElementPairs={() => [
              ["اضافة عطل"          ,<ProblemForm
                                         toggle={() => setLast([0])}
                                         shiftId={shiftId}
                                         writerId={employee.id}
                                         departmentId={employee.department_id}/>],
              ["تعريف مشكلة"        ,<DefineProblem
                                         employee={employee}
                                         toggle={() => setLast([1])}/>],
              ["اظهار الاعطال"         ,<ShiftProblems
                                         shiftId={shiftId}/>],
              ["السجل"              ,<HistoryShow
                                         department_id={employee.department_id} />]
            ]}
            num={last}
            fun={() => setLast([-1])}/>
    </section>
  )
}

function AboutParagraph({employee} : {employee : Employee}){
  const style = css({
    position: "absolute",
    top: "0px",
    left: "0px",
    width: "15%",
    padding: ".5em",
    borderRight: "2px solid",
    borderBottom: "2px solid",
    borderTopRightRadius: "20px",
    borderBottomLeftRadius: "20px",
  })
  return (
    <p class={style}>{`${employee.first_name} ${employee.middle_name} ${employee.last_name}`}</p>
  )
}

function LogoutButton({
          setEmployee,
          setShiftId
        } : {
          setEmployee : Setter<Employee | null>,
          setShiftId  : Setter<string | null>
        }){
  const [hover, setHover] = createSignal(false)

  const logout = () => {
    invoke('logout')
      .then(() => {
        setEmployee(null)
        setShiftId(null)
      }
    ).catch(err => console.log(err))
  }

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
