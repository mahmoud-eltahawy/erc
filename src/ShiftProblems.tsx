import { invoke } from "@tauri-apps/api"
import { useEffect, useState } from "react"
import { Employee, Name, Problem, ShiftProblem, ShiftProblemMini,Machine, Note, SparePart } from "./main"


export default function ShiftProblems({writerId ,shiftId} :{writerId : string,shiftId : string}){
  const [shiftProblems,setShiftProblems] = useState<ShiftProblem[]>([])

  useEffect(() => {
    const process = async function() {
      try{
        const miniProblems : ShiftProblemMini[] = await invoke('get_current_shift_problems',
                                                    {ids : {writer_id : writerId,shift_id : shiftId}})
        let result : ShiftProblem[]= []

        for(let i = 0; i < miniProblems.length ; i++){
            let mp = miniProblems[i]

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
            result.push(p)
        }
        setShiftProblems(result)
        console.log(result)
      } catch(err) {
          console.log(err)
      }
    }
    process()
  },[])


    const limit = 4
    const [state,setState] = useState(shiftProblems)
    const [tooLong,setTooLong] = useState(state.length > limit)

    useEffect(() => {
        if(tooLong) {
           setState(state => state.slice(0,limit))
        } else {
           setState(shiftProblems)
        }
    },[tooLong])

    const showButton = <button className="LongListButton" onClick={() => setTooLong(false)}>شاهد الكل</button>
    const disapearButton = <button className="LongListButton" onClick={() =>setTooLong(true)}>اخفاء</button>;

    const sparePartsList = (spareParts : SparePart[] | null) => {
        return (
            spareParts ? <SparePartsList parts={spareParts}/> : <li><p>لم تستخدم اي قطعة غيار</p></li>
        )
    }

    const noteTd = (note : Note | null) => {
        return (
            note ? <LongNote note={note}/> : <td><p>لا يوجد ملحوظات اضافية</p></td>
        )
    }

    return (
        <section>
            <table>
                <thead>
                    <tr>
                        <td>مسجل العطل</td>
                        <td>ملحوظة جانبية</td>
                        <td>وقت النهاية</td>
                        <td>وقت البداية</td>
                        <td>قطع الغيار</td>
                        <td>المشاكل</td>
                        <td>القائم باصلاح العطل</td>
                        <td>الماكينة التي حدث عليها العطل</td>
                    </tr>
                </thead>
                <tbody>
                    {state.map(problem => <tr key={problem.id}>
                        {(() => {
                            const {id,first_name,middle_name,last_name} = problem.writer
                            return (
                                <td key={id}>
                                    <p>{first_name}</p>
                                    <p>{middle_name}</p>
                                    <p>{last_name}</p>
                                </td>
                            )
                        })()}
                        {noteTd(problem.note)}
                        <td> {problem.endTime} </td>
                        <td> {problem.beginTime} </td>
                        <td>
                            <ul>
                                {sparePartsList(problem.spareParts)}
                            </ul>
                        </td>
                        <td>
                          <ProblemsComps problems={problem.problems}/>
                        </td>
                        {(() => {
                            const {id,first_name,middle_name,last_name} = problem.maintainer
                            return (
                                <td key={id}>
                                    <p>{first_name}</p>
                                    <p>{middle_name}</p>
                                    <p>{last_name}</p>
                                </td>
                            )
                        })()}
                        <td key={problem.machine.id}>{problem.machine.name}</td>
                    </tr>)}
                </tbody>
            </table>
            { shiftProblems.length <= limit ? <></>: tooLong ? showButton : disapearButton}
        </section>
    )
}

function ProblemsComps({problems} : {problems : Problem[]}){
    const [state,setState] = useState(problems)
    const [tooLong,setTooLong] = useState(state.length > 3)

    useEffect(() => {
        if(tooLong) {
           setState(state => state.slice(0,3))
        } else {
           setState(problems)
        }
    },[tooLong])

    const showButton = <li><button className="LongListButton" onClick={() => setTooLong(false)}>شاهد الكل</button></li>
    const disapearButton = <li><button className="LongListButton" onClick={() =>setTooLong(true)}>اخفاء</button></li>

    return (
        <ul>
            {state.map(problem => <ProblemCom key={problem.id} problem={problem} />)}
            { problems.length <= 3 ? <></>: tooLong ? showButton : disapearButton}
        </ul>
    )
}

function SparePartsList({parts} : {parts : Name[]}){
    const [state,setState] = useState(parts)
    const [tooLong,setTooLong] = useState(state.length > 3)

    useEffect(() => {
        if(tooLong) {
           setState(state => state.slice(0,3))
        } else {
           setState(parts)
        }
    },[tooLong])

    const showButton = <li><button className="LongListButton" onClick={() => setTooLong(false)}>شاهد الكل</button></li>
    const disapearButton = <li><button className="LongListButton" onClick={() =>setTooLong(true)}>اخفاء</button></li>;

    return (
        <ul>
            {state.map(part => <li key={part.id}>{part.name}</li>)}
            { parts.length <= 3 ? <></>: tooLong ? showButton : disapearButton}
        </ul>
    )
}

function LongNote({note} : {note : Note}){
    const {id,content} = note
    const limit = 50
    const [state,setState] = useState(content)
    const [tooLong,setTooLong] = useState(state.length > limit)

    useEffect(() => {
        if(tooLong) {
           setState(state => state.slice(0,limit))
        } else {
           setState(content)
        }
    },[tooLong])

    const showButton = <button className="LongListButton" onClick={() => setTooLong(false)}>شاهد الكل</button>
    const disapearButton = <button className="LongListButton" onClick={() =>setTooLong(true)}>اخفاء</button>;

    return (
        <td key={id}>
            <p>{state}</p>
            { content.length <= limit ? <></>: tooLong ? showButton : disapearButton}
        </td>
    )
}


function ProblemCom({problem} : {problem : Problem}){
    const {id,title,description} = problem
    const [state, setState] = useState(title)
    return (
        <li onMouseOver={() => setState(description)}
            onMouseLeave={() => setState(title)}
            key={id}>{state}</li>
    )
}
