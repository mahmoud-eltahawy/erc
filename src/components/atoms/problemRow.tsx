import { Show } from "solid-js";
import { css } from "solid-styled-components";
import { Employee, Note, Problem, ShiftProblem, SparePart } from "../..";
import LongNote from "./longNote";
import ProblemsComps from "./problemsComponents";
import SparePartsList from "./sparePartsList";

export default function ProblemRow({problem} : {problem : ShiftProblem}){
  const begin : string = JSON.parse(problem.beginTime)
  const end   : string = JSON.parse(problem.endTime)

  const style = css({
    width: "9%",
    paddingLeft: "7px",
    paddingRight: "7px",
    borderRight: "dotted 1px",
    borderLeft: "dotted 1px",
    borderBottom: "dotted 1px",
  })

  return (
    <tr>
      <td class={style}> { employeeName(problem.writer)                     } </td>
      <td class={style}> { noteTd(problem.note)                             } </td>
      <td class={style}> {`من ${begin.slice(0, 5) } الي ${end.slice(0, 5)}`  } </td>
      <td class={style}> { sparePartsList(problem.spareParts)               } </td>
      <td class={style}> { problems(problem.problems)                       } </td>
      <td class={style}> { employeeName(problem.maintainer)                 } </td>
      <td class={style}> { problem.machine.name                             } </td>
    </tr>
  )
}

const employeeName = (
    {first_name,
     middle_name,
     last_name} : Employee) => <p> { `${first_name} ${middle_name} ${last_name}` } </p>

const noteTd = (note: Note | null) => note ? <LongNote note={note}/> : <p>لا يوجد ملحوظات اضافية</p>

const problems = (problems: Problem[]) => <ProblemsComps problems={problems}/>


const sparePartsList = (spareParts : SparePart[] | null) => {
  const elsing = <p>لم تستخدم اي قطعة غيار</p>
  return (
    <Show when={spareParts} fallback={elsing}>
      <Show when={spareParts!.length !== 0} fallback={elsing}>
        <SparePartsList parts={spareParts!}/>
      </Show>
    </Show>
  )
}
