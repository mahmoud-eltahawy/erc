import { invoke } from "@tauri-apps/api/tauri"
import { createSignal } from "solid-js"
import { employee, shiftId } from "../../App"
import SubmitButton from "../atoms/submitButton"
import { DescriptionInput } from "./defineProblem"

export default function AddShiftNote({
    toggle
  } : {
    toggle : Function
}){
  const [desc,setDesc] = createSignal('')

  async function handleSubmit(e : Event) {
    e.preventDefault()
    toggle()
    try{
      await invoke('save_shift_note',
            {shiftId : shiftId(),writerId : employee()?.id,content : desc()})
        setDesc('')
    } catch(err){
      alert(err)
    }
  }
  return (
    <form onSubmit={handleSubmit}>
     <DescriptionInput desc={() => desc()} setDesc={setDesc}/>
      <SubmitButton length={() => desc().length} />
    </form>
  )
}
