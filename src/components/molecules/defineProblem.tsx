import { invoke } from '@tauri-apps/api';
import { createSignal, Setter, Show } from 'solid-js'
import { css } from 'solid-styled-components';
import { Employee, permissions } from '../../index';

export default function DefineProblem({
    toggle,
    employee,
} : {
    toggle : Function,
    employee : Employee
}){
  const [title,setTitle] = createSignal('')
  const [desc,setDesc] = createSignal('')

  async function handleSubmit(e : Event) {
    e.preventDefault()
    toggle()
    try{
      await invoke('define_problem',
            {writerId : employee.id,departmentId : employee.department_id,title : title(),description : desc()})
        setTitle('')
        setDesc('')
    } catch(err){
      alert(err)
    }
  }

  const style = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
  <section class={style}>
    <Show
        when={permissions()?.define_problem}
        fallback={<h1>ليس لديك صلاحية تعريف مشكلة</h1>}>
      <form onSubmit={handleSubmit}>
        <TitleInput title={() => title()} setTitle={setTitle} />
        <DescriptionInput desc={() => desc()} setDesc={setDesc}/>
        <SubmitButton length={() => desc().length} />
      </form>
    </Show>
  </section>
  )
}

function SubmitButton({length} : {length : () => number}){
  const [hover,setHover] = createSignal(false)

  const style = () => css({
   display: "block",
   width: "25%",
   borderRadius: hover() ? "5px" : "20px",
   fontSize: hover() ? "24px" : "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
    <button
        class={style()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        type="submit">تاكيد {length()}</button>
  )
}

function DescriptionInput({desc,setDesc} : {desc : () => string,setDesc : Setter<string>}){
  const style = css({
   display: "block",
   width: "50%",
   fontSize: "20px",
   padding: ".5em",
   margin: ".3em auto",
   backgroundColor:"beige",
   border: "solid 3px",
  })
  return (
    <textarea
        value={desc()}
        onInput={e => setDesc(e.currentTarget.value)}
        maxLength={349}
        cols={30} rows={5}
        class={style}
        placeholder="وصف المشكلة في اقل من 350 حرف"
        required></textarea>
  )
}

function TitleInput({title,setTitle} :{title : () => string, setTitle : Setter<string>}){
  const style = css({
   display: "block",
   width: "50%",
   fontSize: "20px",
   padding: ".5em",
   margin: ".3em auto",
   backgroundColor:"beige",
   border: "solid 3px",
  })

  return (
      <input
          value={title()}
          class={style}
          onInput={(e) => setTitle(e.currentTarget.value)}
          type="text"
          placeholder="اسم المشكلة"
          required/>
  )
}
