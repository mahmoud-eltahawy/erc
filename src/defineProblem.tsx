import { invoke } from '@tauri-apps/api';
import { BaseSyntheticEvent, useRef, useState } from 'react'

export default function DefineProblem({
    toggle,
    addDefinition
} : {
    toggle : Function,
    addDefinition :Function

}){
  const [descLength, setDescLength] = useState(0);
  const titleR = useRef<HTMLInputElement>(null)
  const descriptionR = useRef<HTMLTextAreaElement>(null)

  async function handleSubmit(e : BaseSyntheticEvent) {
    e.preventDefault()
    toggle()
    try{
      const title = titleR.current!.value
      const description = descriptionR.current!.value
      const id : string = await invoke('define_problem',{
        title,description})
      if(id){
        addDefinition({id : id , name : title})
      }
    } catch(err){
        console.log(err)
    }
  }
    return (
    <section className={"LoginFormContainer"}>
      <form onSubmit={handleSubmit}>
        <input ref={titleR} className={"LoginFormInput"} type="text" placeholder="اسم المشكلة" required/>
        <textarea ref={descriptionR}
                  onChange={e => setDescLength(e.currentTarget.value.length)}
                  maxLength={349}
                  cols={30} rows={5}
                  className={"LoginFormInput"}
                  placeholder="وصف المشكلة في اقل من 350 حرف"
                  required></textarea>
        <button type="submit">تاكيد {descLength}</button>
      </form>
    </section>
    )
}
