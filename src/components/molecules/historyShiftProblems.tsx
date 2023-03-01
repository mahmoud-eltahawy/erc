import { createEffect, createSignal,createResource } from "solid-js"
import { ShiftProblem, problemsFetcher } from "../../index"
import ProblemRow from "../atoms/problemRow"
import TableHead from "../atoms/problemTableHead"
import togglingButton from "../atoms/problemTogglingButton"

export default function HistoryShiftProblems({
    shiftId,
    } : {
    shiftId : string,
}){
  const limit = 3
  const [shiftProblems]      = createResource(shiftId, problemsFetcher)
  const [state,setState]     = createSignal<ShiftProblem[]>([])
  const [tooLong,setTooLong] = createSignal(state.length > limit)

  createEffect(() => {
    if(tooLong()) {
      setState((shiftProblems() || []).slice(0,limit))
    } else {
      setState(shiftProblems() || [])
    }
  })

  return (
    <section>
      <table>
        {<TableHead/>}
        <tbody>
            {(state() || []).map(problem => <ProblemRow problem={problem}/>)}
        </tbody>
      </table>
        {togglingButton({
          showButton : () => (shiftProblems() || []).length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () =>setTooLong(!tooLong)})}
    </section>
  )
}
