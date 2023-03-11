import { Employee, Note, Problem, ShiftProblem, SparePart } from "../..";
import LongNote from "./longNote";
import ProblemsComps from "./problemsComponents";
import SparePartsList from "./sparePartsList";

export default function ProblemRow({problem} : {problem : ShiftProblem}){
  const begin : string = JSON.parse(problem.beginTime)
  const end   : string = JSON.parse(problem.endTime)
  return (
    <tr>
      <td> { employeeName(problem.writer)                     } </td>
      <td> { noteTd(problem.note)                             } </td>
      <td> {`من ${begin.slice(0, 5) } الي ${end.slice(0, 5)}`  } </td>
      <td> { sparePartsList(problem.spareParts)               } </td>
      <td> { problems(problem.problems)                       } </td>
      <td> { employeeName(problem.maintainer)                 } </td>
      <td> { problem.machine.name                             } </td>
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
    const core = <SparePartsList parts={spareParts!}/>
    const ifNotEmpty = (list : SparePart[]) => list.length !== 0 ? core : elsing
    return (
        spareParts ? ifNotEmpty(spareParts) : elsing
    )
}
