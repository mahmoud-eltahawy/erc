import { invoke } from "@tauri-apps/api"
import { useEffect, useState } from "react"
import { useEmployee, useEmployeeUpdate } from "./employeeProvider"
import { Name, ProblemDeps } from "./main"
import ProblemForm from "./ProblemForm"

export default function Wall(){
  const employee = useEmployee()
  const setEmployee = useEmployeeUpdate()
  const [shiftBegin,setShiftBegin]   = useState('')
  const [shiftEnd,setShiftEnd]       = useState('')
  const [machines  ,setMachines]     = useState<Name[]>([])
  const [employees ,setEmployees]    = useState<Name[]>([])
  const [problems  ,setProblems]     = useState<Name[]>([])
  const [spareParts,setSpareParts]   = useState<Name[]>([])
  const [problemFormDeps,setProblemFormDeps] = useState<ProblemDeps>()

  useEffect(() => {
      const deps : ProblemDeps = {
          machines : machines,
          employees: employees,
          problems : problems,
          spareParts : spareParts,
          shiftBegin : shiftBegin,
          shiftEnd : shiftEnd
      }
      setProblemFormDeps(deps)
  },[shiftBegin,shiftEnd,machines,employees,problems,spareParts])

  useEffect(() => {
      invoke('employees_selection')
        .then(result => setEmployees(result as Name[]))
        .catch(err => {
          console.log(err)
          invoke('update_employees_selection')
            .then(() => invoke('employees_selection')
            .then(result => setEmployees(result as Name[]))
            .catch(err => console.log(err))
      )})
      invoke('problems_selection')
        .then(result => setProblems(result as Name[]))
        .catch(err => {
          console.log(err)
          invoke('update_problems_selection')
            .then(() => invoke('problems_selection')
            .then(result => setProblems(result as Name[]))
            .catch(err => console.log(err))
      )})
      invoke('machines_selection')
        .then(result => setMachines(result as Name[]))
        .catch(err => {
          console.log(err)
          invoke('update_machines_selection')
            .then(() => invoke('machines_selection')
            .then(result => setMachines(result as Name[]))
            .catch(err => console.log(err))
      )})
      invoke('spare_parts_selection')
        .then(result => setSpareParts(result as Name[]))
        .catch(err => {
          console.log(err)
          invoke('update_spare_parts_selection')
            .then(() => invoke('spare_parts_selection')
            .then(result => setSpareParts(result as Name[]))
            .catch(err => console.log(err))
      )});
    invoke("current_shift_borders")
        .then(beginEnd => {
          let [begin,end] = beginEnd as string[]
          setShiftBegin(begin)
          setShiftEnd(end)
        })
        .catch(err => console.log(err))
    },[])

  const [toggleButtons, setToggleButtons] = useState([
      {id : 'problemAdd'   , display : false},
      {id : 'problemDefine', display : false},
      {id : 'problemsShow' , display : false}
  ])
  const [emptyPlayGround,setEmptyPlayGround] = useState(true)

  const logout = () => {
    invoke('logout').then(
      setEmployee(null)
    )
  }

  const toggle = (id : string) => {
      setToggleButtons(buttons => buttons.map(obj => {
          if (obj.id === id) {
            if(obj.display){
              setEmptyPlayGround(true)
              return { id: id, display: false }
            }
            setEmptyPlayGround(false)
            return { id: id, display: true }
          } else {
            return { ...obj, display: false }
          }
        })
      )
  }

  const theButtons = <div>
    { emptyPlayGround || toggleButtons[0].display ?
      <button id="problemAdd" onClick={e => toggle(e.currentTarget.id)}>اضافة عطل</button> : <></> }
    { emptyPlayGround || toggleButtons[1].display ?
      <button id="problemDefine" onClick={e => toggle(e.currentTarget.id)}>تعريف مشكلة</button> : <></>}
    { emptyPlayGround || toggleButtons[2].display ?
      <button id="problemsShow"  onClick={e => toggle(e.currentTarget.id)}>اظهار الاعطال</button> : <></>}
  </div>

  return (
    <section>
      <button className={"LogoutButton"} onClick={() => logout()}>تسجيل خروج</button>
      <p className={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
          {theButtons}
      {toggleButtons[0].display ? <ProblemForm deps={problemFormDeps!} /> : <></>}
    </section>
  )
}
