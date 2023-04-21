import { createSignal } from "solid-js"
import { Name } from "../.."
import togglingButton from "./problemTogglingButton"

export default function ToggelableList({elements} : {elements :() => Name[]}){
  const limit = 3
  const [tooLong,setTooLong] = createSignal(elements().length > limit)

  return (
    <ul>
      {tooLong()
        ? elements().slice(0, limit).map(element => <li>{element.name}</li>)
        : elements().map(element => <li>{element.name}</li>)}
        {togglingButton({
          showButton : () => elements().length > limit,
          showMore   : () => tooLong(),
          doOnClick  : () => setTooLong(!tooLong())})}

    </ul>
  )
}
