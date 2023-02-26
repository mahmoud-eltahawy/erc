import { invoke } from '@tauri-apps/api';
import { createSignal } from 'solid-js'
import { Employee } from '../../index';

export default function DefineProblem({
    toggle,
    employee,
} : {
    toggle : Function,
    employee : Employee
}){
  const [descLength, setDescLength] = createSignal(0);
  let titleR : HTMLInputElement | undefined
  let descriptionR : HTMLTextAreaElement | undefined

  async function handleSubmit(e : any) {
    e.preventDefault()
    toggle()
    try{
      const title = titleR!.value
      const description = descriptionR!.value
      await invoke('define_problem',
            {writerId : employee.id,departmentId : employee.department_id,title,description})
        titleR!.value = ""
        descriptionR!.value = ""
    } catch(err){
      alert(err)
    }
  }
  return (
  <section class={"LoginFormContainer"}>
    <form onSubmit={handleSubmit}>
      <input ref={titleR} class={"LoginFormInput"} type="text" placeholder="اسم المشكلة" required/>
      <textarea ref={descriptionR}
                onInput={e => setDescLength(e.currentTarget.value.length)}
                maxLength={349}
                cols={30} rows={5}
                class={"LoginFormInput"}
                placeholder="وصف المشكلة في اقل من 350 حرف"
                required></textarea>
      <button type="submit">تاكيد {descLength}</button>
    </form>
  </section>
  )
}
