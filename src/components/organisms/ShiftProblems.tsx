import { createEffect, createSignal,createResource, Show } from "solid-js"
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
  const [state,setState]  = createSignal<ShiftProblem[] | undefined>([])
  const [tooLong,setTooLong] = createSignal(state.length > limit)

  listen("update_shift_problem",() => {
    setTimeout(() => refetch(),2000)
  })

  createEffect(() => {
    if(tooLong()) {
        if(shiftProblems()){
          setState(shiftProblems()!.slice(0,limit))
        } else {
          setState(undefined)
        }
    } else {
      setState(shiftProblems())
    }
  })

  return (
    <section>
      <table>
        <TableHead/>
        <Show when={state()} fallback={<h1>جاري التحميل ...</h1>}>
          <tbody>
            {state()!.map(problem => <ProblemRow problem={problem}/>)}
          </tbody>
        </Show>
      </table>
      {togglingButton({
          showButton : () => (shiftProblems() || []).length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () => setTooLong(!tooLong())})}
    </section>
  )
}
