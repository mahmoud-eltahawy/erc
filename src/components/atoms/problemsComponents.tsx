import { createEffect, createSignal } from "solid-js"
import { Problem } from "../.."
import togglingButton from "./problemTogglingButton"

export default function ProblemsComps({problems} : {problems : Problem[]}){
  const limit = 3
  const [state,setState] = createSignal(problems)
  const [tooLong,setTooLong] = createSignal(problems.length > limit)

  createEffect(() => {
    if(tooLong()) {
       setState(problems.slice(0,limit))
    } else {
       setState(problems)
    }
  })

  return (
    <ul>
      {state().map(problem => <ProblemCom problem={problem} />)}
        {togglingButton({
          showButton : () => problems.length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () => setTooLong(!tooLong())})}
    </ul>
  )
}

export function ProblemCom({problem} : {problem : Problem}){
  return (
    <li>
        {problem.title}
    </li>
  )
}
