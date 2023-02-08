import { invoke } from '@tauri-apps/api';
import { BaseSyntheticEvent, useRef, useState } from 'react'
import { useEmployeeAndShiftID } from './employeeProvider';

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
  const [employee] = useEmployeeAndShiftID();

  async function handleSubmit(e : BaseSyntheticEvent) {
    e.preventDefault()
    toggle()
    try{
      const writerId = employee!.id
      const title = titleR.current!.value
      const description = descriptionR.current!.value
      const id : string = await invoke('define_problem',{
        title,description,writerId})
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
