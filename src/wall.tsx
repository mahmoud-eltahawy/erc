import { invoke } from "@tauri-apps/api"
import { createResource, createSignal, JSXElement, Setter, Show } from "solid-js"
import DefineProblem from "./components/molecules/defineProblem"
import HistoryShow from "./components/organisms/HistoryShow"
import { Employee, Permissions,NativeDepartment } from "./index"
import ProblemForm from "./components/organisms/ProblemForm"
import ShiftProblems from "./components/organisms/ShiftProblems"
import { ButtonsOrElement } from "./components/molecules/buttonsOrElement"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import Controlling from "./components/organisms/controlling"
import { listen } from "@tauri-apps/api/event"


const permissions_fetcher = async ({id} : {id : string}) => {
  return (await invoke("employee_permissions",{id})
      .catch(err => console.log(err))) as Permissions
}

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
  const [permissions,{refetch}] = createResource({id : employee.id},permissions_fetcher)

  listen("update_permissions",(e) => {
    if(employee.id === e.payload){
      setTimeout(() => refetch(),1000)
    }
  })

  setInterval(() => {
    invoke('check_shift_time',{departmentId : employee.department_id})
      .catch(err => console.log(err))
  },60000)

  return (
    <section>
      <AboutParagraph employee={employee} />
      <LogoutButton setEmployee={setEmployee} setShiftId={setShiftId} />
      <Show when={permissions()}>
        <ButtonsOrElement returnButtonText="الصفحة الرئيسية"
            buttonElementPairs={() => {
             if(employee.card_id === 0){
                 return [["التحكم", <Controlling/>]]
             }

             const pairs : [string,JSXElement][] = []
             const {
                 define_problem,
                 modify_department_problems,
                 read_department_problems,
                 write_department_problem,
                 access_history_spare_parts,
                 access_history_all_departments_department_problems,
                 access_history_all_departments_problems,
                 access_history_department_department_problems,
                 access_history_department_problems,
                 access_history_employees,
                 access_history_machines
             } = permissions()!

             if(write_department_problem){
               pairs.unshift(["اضافة عطل",<ProblemForm
                                            toggle={() => setLast([0])}
                                            shiftId={shiftId}
                                            writerId={employee.id}
                                            departmentId={employee.department_id}/>])
             }

             if(define_problem){
               pairs.unshift(["تعريف مشكلة" ,<DefineProblem
                                            employee={employee}
                                            toggle={() => setLast([1])}/>])
             }

             if(read_department_problems){
               pairs.unshift(["اظهار الاعطال",<ShiftProblems
                                            mutable={modify_department_problems}
                                            shiftId={shiftId}/>])
             }

             if(access_history_all_departments_department_problems ||
                access_history_all_departments_problems ||
                access_history_department_department_problems ||
                access_history_department_problems ||
                access_history_employees ||
                access_history_machines ||
                access_history_spare_parts){
               pairs.unshift(["السجل", <HistoryShow
                                           permissions={() => {return {
                                               access_history_all_departments_department_problems,
                                               access_history_all_departments_problems,
                                               access_history_department_department_problems,
                                               access_history_department_problems,
                                               access_history_employees,
                                               access_history_machines,
                                               access_history_spare_parts,
                                           }}}
                                           department_id={employee.department_id} />])
             }

             if (employee.position === 'SUPER_USER') {
               pairs.unshift(["التحكم", <Controlling/>])
             }

             return pairs
            }}
            num={last}
            fun={() => setLast([-1])}/>
      </Show>
    </section>
  )
}


const department_fetcher = async ({id} : {id : string}) => {
  return (await invoke("find_department",{id})) as NativeDepartment
}

function AboutParagraph({employee} : {employee : Employee}){
  const [hover,setHover] = createSignal(false)
  const [department] = createResource({id : employee.department_id},department_fetcher)

  const position = () => {
    const superiority = employee.position === "SUPER_USER" ? "مشرف" : "مستخدم"
    const is_boss = employee.id === department()?.boss_id ? " و رئيس القسم" : ""
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
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        class={container()}>
      <p>{`الاسم : ${employee.first_name} ${employee.middle_name} ${employee.last_name}`}</p>
      <Show when={hover() && department()}>
        <p>{`القسم : ${department()?.name}`}</p>
        <p>{`الرتبة : ${position()}`}</p>
      </Show>
    </div>
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
