import { createEffect, createSignal } from "solid-js"
import { Note } from "../.."
import togglingButton from "./problemTogglingButton"

export default function LongNote({note} : {note : Note}){
  const content = note.content
  const limit = 15
  const [state,setState]     = createSignal(content)
  const [tooLong,setTooLong] = createSignal(note.content.length > limit)

  createEffect(() => {
    if(tooLong()) {
      setState(state => state.slice(0,limit))
    } else {
      setState(content)
    }
  })

  return (
    <section>
      <p>{state}</p>
        {togglingButton({
          showButton : () => content.length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () =>setTooLong(!tooLong())})}
    </section>
  )
}
