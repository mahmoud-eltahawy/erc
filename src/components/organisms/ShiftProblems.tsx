import { useEffect, useState } from "react"
import { Name, Problem, ShiftProblem, Note, SparePart, Employee } from "./main"

export default function ShiftProblems({shiftProblems} :{shiftProblems : ShiftProblem[]}){
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

  return (
    <section>
      <table>
        {tableHead}
        <tbody>
          {state.map(problem => <ProblemRow key={problem.id} problem={problem}/>)}
        </tbody>
      </table>
      {togglingButton({
          showButton : shiftProblems.length > limit,
          showMore   : tooLong,
          doOnClick  : () =>setTooLong(!tooLong)})}
    </section>
  )
}
const tableHead = <thead><tr>
                    <td>مسجل العطل</td>
                    <td>ملحوظة جانبية</td>
                    <td>وقت النهاية</td>
                    <td>وقت البداية</td>
                    <td>قطع الغيار</td>
                    <td>المشاكل</td>
                    <td>القائم باصلاح العطل</td>
                    <td>الماكينة التي حدث عليها العطل</td>
                </tr></thead>

function ProblemRow({problem} : {problem : ShiftProblem}){
  return (
    <tr>
      <td> { employeeName(problem.writer)       } </td>
      <td> { noteTd(problem.note)               } </td>
      <td> { problem.endTime                    } </td>
      <td> { problem.beginTime                  } </td>
      <td> { sparePartsList(problem.spareParts) } </td>
      <td> { problems(problem.problems)         } </td>
      <td> { employeeName(problem.maintainer)   } </td>
      <td> { problem.machine.name               } </td>
    </tr>
  )
}

const sparePartsList = (spareParts : SparePart[] | null) => {
    const elsing = <p>لم تستخدم اي قطعة غيار</p>
    const core   = <SparePartsList parts={spareParts!}/>
    const ifNotEmpty = (list : SparePart[]) => list.length !== 0 ? core : elsing
    return (
        spareParts ? ifNotEmpty(spareParts) : elsing
    )
}

function ProblemsComps({problems} : {problems : Problem[]}){
  const limit = 3
  const [state,setState] = useState(problems)
  const [tooLong,setTooLong] = useState(state.length > limit)

  useEffect(() => {
    if(tooLong) {
       setState(state => state.slice(0,limit))
    } else {
       setState(problems)
    }
  },[tooLong])

  return (
    <ul>
      {state.map(problem => <ProblemCom key={problem.id} problem={problem} />)}
      {togglingButton({
          showButton : problems.length > limit,
          showMore   : tooLong,
          doOnClick  : () =>setTooLong(!tooLong)})}
    </ul>
  )
}

function SparePartsList({parts} : {parts : Name[]}){
  const limit = 3
  const [state,setState] = useState(parts)
  const [tooLong,setTooLong] = useState(state.length > limit)

  useEffect(() => {
    if(tooLong) {
       setState(state => state.slice(0,limit))
    } else {
       setState(parts)
    }
  },[tooLong])

  return (
    <ul>
      {state.map(part => <li key={part.id}>{part.name}</li>)}
      {togglingButton({
          showButton : parts.length > limit,
          showMore   : tooLong,
          doOnClick  : () =>setTooLong(!tooLong)})}

    </ul>
  )
}

function LongNote({note} : {note : Note}){
  const content = note.content
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

  return (
    <section>
      <p>{state}</p>
      {togglingButton({
          showButton : content.length > limit,
          showMore   : tooLong,
          doOnClick  : () =>setTooLong(!tooLong)})}
    </section>
  )
}

const togglingButton = (
    {showButton
    ,showMore,
     doOnClick
      } : {
     showButton : boolean,
     showMore   : boolean,
     doOnClick  : Function
      }
) => showButton ? <button className="LongListButton"
                         onClick={() => doOnClick()}
                  >{showMore ? "شاهد اكثر" : "شاهد اقل"}</button> : <></>


function ProblemCom({problem} : {problem : Problem}){
  const {id,title,description} = problem
  const [state, setState] = useState(title)
  return (
    <li onMouseOver={() => setState(description)}
        onMouseLeave={() => setState(title)}
        key={id}>
        {state}
    </li>
  )
}

const noteTd = (note : Note | null) => note ? <LongNote note={note}/> : <p>لا يوجد ملحوظات اضافية</p>
const problems = (problems : Problem[]) => <ProblemsComps problems={problems}/>
const employeeName = (
    {first_name,
     middle_name,
     last_name} : Employee) => <p> { `${first_name} ${middle_name} ${last_name}` } </p>
