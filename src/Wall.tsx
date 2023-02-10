import { invoke } from "@tauri-apps/api"
import { useEffect, useState } from "react"
import DefineProblem from "./components/molecules/defineProblem"
import { useEmployeeAndShiftIDUpdate } from "./components/providers/employeeProvider"
import HistoryShow from "./components/organisms/HistoryShow"
import { Employee, Machine, Name, Problem, ShiftProblem, ShiftProblemMini, SparePart } from "./main"
import ProblemForm from "./components/organisms/ProblemForm"
import ShiftProblems from "./components/organisms/ShiftProblems"
import { ToggleButtons } from './components/molecules/toggleButtons'

const bId = {
  problemAdd    : '0',
  problemDefine : '1',
  problemsShow  : '2',
  shiftsHistory : '3'
}

export default function Wall({
    shiftBegin,
    shiftEnd  ,
    machines  ,
    employees ,
    spareParts,
    employee,
    shiftId
} : {
    shiftBegin : string,
    shiftEnd   : string,
    machines   : Name[],
    employees  : Name[],
    spareParts : Name[],
    employee   : Employee,
    shiftId    : string
}){
  const [problems  ,setProblems]     = useState<Name[]>([])
  const [shiftProblems,setShiftProblems] = useState<ShiftProblem[]>([])
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()
  const [emptyPlayGround,setEmptyPlayGround] = useState(true)
  const [toggleButtons, setToggleButtons] = useState<Array<boolean>>(Array(4).fill(false))

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

    const problemsFun = async function(){
      try{
        const names : Name[] = await invoke('problems_selection',
                            {departmentId : employee!.department_id})
        setProblems(names)
      } catch(err){
        console.log(err)
      }
    }

    shiftProblemsFun()
    problemsFun()
  },[])

  const logout = () => {
    invoke('logout').then(
      setEmployeeAndShiftId([null,null])
    )
  }

  const toggle = (id : string) => {
    setToggleButtons(buttons => buttons.map((cond,condId) => {
        if (condId !== +id) {
          return false
        }
        if(cond){
          setEmptyPlayGround(true)
          return false
        }
        setEmptyPlayGround(false)
        return true
      })
    )
  }
  const theButtons = <ToggleButtons defaultContent="الصفحة الرئيسية"
                                    isEmptyGround={emptyPlayGround}
                                    idToggle={toggle}
                                    tbuttons={toggleButtons}
                                    idContent={[
                                      {id : bId.problemAdd    ,content : "اضافة عطل"},
                                      {id : bId.problemDefine ,content : "تعريف مشكلة"},
                                      {id : bId.problemsShow  ,content : "اظهار الاعطال"},
                                      {id : bId.shiftsHistory ,content : "السجل"}
                                    ]}/>
  const historyShow   = <HistoryShow />
  const defineProblem = <DefineProblem toggle={() => toggle(bId.problemDefine)}
                                       addDefinition={(name : Name) => setProblems(list => [name,...list])}/>
  const problemShow   = <ShiftProblems shiftProblems={shiftProblems}/>
  const logoutButton  = <button className={"LogoutButton"} onClick={() => logout()}>تسجيل خروج</button>
  const employeeName  =<p className={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
  const problemForm   = <ProblemForm
            add={(problem : ShiftProblem) =>setShiftProblems(problems => [problem,...problems])}
            toggle={() => toggle(bId.problemAdd)}
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
      {toggleButtons[+bId.problemAdd]    ? problemForm   : null}
      {toggleButtons[+bId.problemDefine] ? defineProblem : null}
      {toggleButtons[+bId.problemsShow]  ? problemShow   : null}
      {toggleButtons[+bId.shiftsHistory] ? historyShow   : null}
    </section>
  )
}

export const shiftProblemFromMinimal = async function(mp : ShiftProblemMini) : Promise<ShiftProblem> {
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
