import { createEffect, createSignal } from "solid-js"
import { Name } from "../.."
import togglingButton from "./problemTogglingButton"

export default function SparePartsList({parts} : {parts : Name[]}){
  const limit = 3
  const [state,setState] = createSignal(parts)
  const [tooLong,setTooLong] = createSignal(parts.length > limit)

  createEffect(() => {
    if(tooLong()) {
       setState(parts.slice(0,limit))
    } else {
       setState(parts)
    }
  },[tooLong])

  return (
    <ul>
      {state().map(part => <li>{part.name}</li>)}
        {togglingButton({
          showButton : () => parts.length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () => setTooLong(!tooLong())})}

    </ul>
  )
}
