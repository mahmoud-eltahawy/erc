import { createEffect, createSignal,createResource } from "solid-js"
import { ShiftProblem, problemsFetcher } from "../../index"
import ProblemRow from "../atoms/problemRow"
import TableHead from "../atoms/problemTableHead"
import togglingButton from "../atoms/problemTogglingButton"
import { listen } from '@tauri-apps/api/event'

export default function ShiftProblems({
    shiftId,
    } : {
    shiftId : string,
}){
  const limit = 4
  const [shiftProblems,{refetch}] = createResource(shiftId,problemsFetcher)
  const [state,setState]  = createSignal<ShiftProblem[]>([])
  const [tooLong,setTooLong] = createSignal(state.length > limit)

  listen("update_shift_problem",() => {
    setTimeout(() => refetch(),2000)
  })

  createEffect(() => {
    if(tooLong()) {
      setState((shiftProblems() || []).slice(0,limit))
    } else {
      setState(shiftProblems() || [])
    }
    console.log("too long is " + tooLong())
  })

  return (
    <section>
      <table>
        <TableHead/>
        <tbody>
          {(state() || []).map(problem => <ProblemRow problem={problem}/>)}
        </tbody>
      </table>
      {togglingButton({
          showButton : () => (shiftProblems() || []).length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () => setTooLong(!tooLong())})}
    </section>
  )
}
