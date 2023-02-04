import { invoke } from "@tauri-apps/api"
import { useEffect, useState } from "react"
import DefineProblem from "./defineProblem"
import { useEmployeeAndShiftIDUpdate } from "./employeeProvider"
import { Employee, Machine, Name, Problem, ShiftProblem, ShiftProblemMini, SparePart } from "./main"
import ProblemForm from "./ProblemForm"
import ShiftProblems from "./ShiftProblems"

export default function Wall({
    shiftBegin,
    shiftEnd  ,
    machines  ,
    employees ,
    problems  ,
    addProblem,
    spareParts,
    employee,
    shiftId
} : {
    shiftBegin : string,
    shiftEnd   : string,
    machines   : Name[],
    employees  : Name[],
    problems   : Name[],
    addProblem : Function,
    spareParts : Name[],
    employee   : Employee,
    shiftId    : string
}){
  const [shiftProblems,setShiftProblems] = useState<ShiftProblem[]>([])
  const [emptyPlayGround,setEmptyPlayGround] = useState(true)
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()
  const [toggleButtons, setToggleButtons] = useState([
      {id : 'problemAdd'   , display : false},
      {id : 'problemDefine', display : false},
      {id : 'problemsShow' , display : false}
  ])


  const shiftProblemFromMinimal = async function(mp : ShiftProblemMini) : Promise<ShiftProblem> {
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

    let p : ShiftProblem = {
        id          : mp.id,
        shiftId     : mp.shift_id,
        note        : mp.note,
        writer      : await invoke('get_employee_by_id',{id : mp.writer_id}) as Employee,
        maintainer  : await invoke('get_employee_by_id',{id : mp.maintainer_id}) as Employee,
        machine     : await invoke('get_machine_by_id' ,{id : mp.machine_id}) as Machine,
        beginTime   : mp.begin_time,
        endTime     : mp.end_time,
        problems    : problems,
        spareParts  : spareParts
    }
    return p
  }

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
        console.log(result)
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
    shiftProblemsFun()
  },[])


  const logout = () => {
    invoke('logout').then(
      setEmployeeAndShiftId([null,null])
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

  const defineProblem = <DefineProblem toggle={() => toggle("problemDefine")}
                                       addDefinition={(name : Name) => addProblem(name)}/>
  const problemShow   = <ShiftProblems shiftProblems={shiftProblems}/>
  const logoutButton  = <button className={"LogoutButton"} onClick={() => logout()}>تسجيل خروج</button>
  const employeeName  =<p className={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
  const problemForm   = <ProblemForm
            add={(problem : ShiftProblem) =>setShiftProblems(problems => [...problems,problem])}
            toggle={() => toggle("problemAdd")}
            convert={shiftProblemFromMinimal}
            shiftId={shiftId!}
            writerId={employee!.id}
            departmentId={employee!.department_id}
            deps={{machines : machines,
              employees: employees,
              problems : problems,
              spareParts : spareParts,
              shiftBegin : shiftBegin,
              shiftEnd : shiftEnd
    }} />

  return (
    <section>
      {logoutButton}
      {employeeName}
      {theButtons}
      {toggleButtons[0].display ? problemForm   : <></>}
      {toggleButtons[1].display ? defineProblem : <></>}
      {toggleButtons[2].display ? problemShow   : <></>}
    </section>
  )
}
