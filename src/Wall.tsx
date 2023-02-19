import { invoke } from "@tauri-apps/api"
import { useEffect, useState } from "react"
import DefineProblem from "./components/molecules/defineProblem"
import { useEmployeeAndShiftID, useEmployeeAndShiftIDUpdate } from "./components/providers/employeeProvider"
import HistoryShow from "./components/organisms/HistoryShow"
import { Employee, Machine, Name, Problem, ShiftProblem, ShiftProblemMini, SparePart } from "./main"
import ProblemForm from "./components/organisms/ProblemForm"
import ShiftProblems from "./components/organisms/ShiftProblems"
import { ButtonsOrElement } from "./components/molecules/buttonsOrElement"

export default function Wall(){
  const [shiftProblems,setShiftProblems] = useState<ShiftProblem[]>([])
  const [employee,shiftId] = useEmployeeAndShiftID()
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()
  const [lastElement,setLastElement] = useState(-1)
  const [machines  ,setMachines]     = useState<Name[]>([])
  const [employees ,setEmployees]    = useState<Name[]>([])
  const [spareParts,setSpareParts]   = useState<Name[]>([])
  const [problems  ,setProblems]     = useState<Name[]>([])

  useEffect(() => {
    const shiftProblemsFun = async function() {
      const shotTry = async function(){
        const miniProblems : ShiftProblemMini[] = await invoke('get_current_shift_problems',
                                                               {departmentId : employee.department_id})
        let result : ShiftProblem[]= []
        for(let i = 0; i < miniProblems.length ; i++){
          let p = await shiftProblemFromMinimal(miniProblems[i])
          result.push(p)
        }
        setShiftProblems(result)
      }
      try{
        await shotTry()
      } catch(err) {
        try{
          console.log(err)
          await invoke('update_current_shift_problems',
                {ids : {writer_id : employee!.id,shift_id : shiftId,department_id : employee.department_id}})
          await shotTry()
        }catch(err){
          console.log(err)
        }
      }
    }
    const employeesFun = async function(){
      try{
        const names : Name[] = await invoke('employees_selection')
        setEmployees(names)
      } catch(err){
        console.log(err)
      }
    }

    const machinesFun = async function(){
      try{
        const names : Name[] = await invoke('machines_selection')
        setMachines(names)
      } catch(err){
        console.log(err)
      }
    }

    const partsFun = async function(){
      try{
        const names : Name[] = await invoke('spare_parts_selection')
        setSpareParts(names)
      } catch(err){
        console.log(err)
      }
    }

    const problemsFun = async function(){
      try{
        const names : Name[] = await invoke('problems_selection',
                            {departmentId : employee!.department_id})
        setProblems(names)
      } catch(err){
        console.log(err)
      }
    }

    problemsFun()
    employeesFun()
    machinesFun()
    partsFun()
    shiftProblemsFun()
  },[])

  const logout = () => {
    invoke('logout').then(
      setEmployeeAndShiftId([null,null])
    )
  }
  const historyShow   = <HistoryShow />
  const defineProblem = <DefineProblem toggle={() => setLastElement(1)}/>
  const problemShow   = <ShiftProblems shiftProblems={shiftProblems}/>
  const logoutButton  = <button className={"LogoutButton"} onClick={() => logout()}>تسجيل خروج</button>
  const employeeName  =<p className={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
  const problemForm   = <ProblemForm
            add={(problem : ShiftProblem) =>setShiftProblems(problems => [problem,...problems])}
            toggle={() => setLastElement(0)}
            shiftId={shiftId!}
            writerId={employee!.id}
            departmentId={employee!.department_id}
            employees={employees}
            spareParts={spareParts}
            problems={problems}
            machines={machines}
  />

  const buttonsOrElement = <ButtonsOrElement returnButtonText="الصفحة الرئيسية"
                                    buttonElementPairs={[
                                      ["اضافة عطل"  ,problemForm],
                                      ["تعريف مشكلة",defineProblem],
                                      ["اظهار الاعطال" ,problemShow],
                                      ["السجل"      ,historyShow]
                                    ]} num={lastElement} fun={() => setLastElement(-1)}/>
  return (
    <section>
      {logoutButton}
      {employeeName}
      {buttonsOrElement}
    </section>
  )
}

export async function shiftProblemFromMinimal(mp : ShiftProblemMini) : Promise<ShiftProblem> {
  const problems : Problem[] = []
  for(let j =0; j < mp.problems_ids.length; j++){
    problems.push(await invoke('get_problem_by_id',{id : mp.problems_ids[j]}) as Problem)
  }

  const spareParts : SparePart[] | null = mp.spare_parts_ids ? [] : null
  if(mp.spare_parts_ids){
    for(let j =0; j < mp.spare_parts_ids.length; j++){
      spareParts!.push(await invoke('get_spare_part_by_id',{id : mp.spare_parts_ids[j]}) as SparePart)
    }
  }

  return {
      id          : mp.id,
      shiftId     : mp.shift_id,
      note        : mp.note,
      writer      : await invoke('get_employee_by_id',{id : mp.writer_id})     as Employee,
      maintainer  : await invoke('get_employee_by_id',{id : mp.maintainer_id}) as Employee,
      machine     : await invoke('get_machine_by_id' ,{id : mp.machine_id})    as Machine,
      beginTime   : mp.begin_time,
      endTime     : mp.end_time,
      problems    : problems,
      spareParts  : spareParts
  }
}
