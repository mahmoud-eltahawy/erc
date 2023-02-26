import { invoke } from "@tauri-apps/api"
import { Setter } from "solid-js"
import DefineProblem from "./components/molecules/defineProblem"
import HistoryShow from "./components/organisms/HistoryShow"
import { Employee } from "./index"
import ProblemForm from "./components/organisms/ProblemForm"
import ShiftProblems from "./components/organisms/ShiftProblems"
import { ButtonsOrElement } from "./components/molecules/buttonsOrElement"
import { createStore } from "solid-js/store"

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
  const [last,setLast]                       = createStore([-1])

  const logout = () => {
    invoke('logout')
      .then(() => {
        setEmployee(null)
        setShiftId(null)
      }
    ).catch(err => console.log(err))
  }

  setInterval(() => {
    invoke('check_shift_time',{departmentId : employee.department_id})
      .then(() => console.log("employee checked"))
      .catch(err => console.log(err))
  },60000)

  return (
    <section>
      <button class={"LogoutButton"} onClick={() => logout()}>تسجيل خروج</button>
      <p class={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
      <ButtonsOrElement returnButtonText="الصفحة الرئيسية"
            buttonElementPairs={[
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
              ["السجل"              ,<HistoryShow />]
            ]}
            num={last}
            fun={() => setLast([-1])}/>
    </section>
  )
}
