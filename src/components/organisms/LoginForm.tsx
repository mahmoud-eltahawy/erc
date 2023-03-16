import { invoke } from '@tauri-apps/api/tauri';
import { Employee } from '../../index';
import {createSignal, Setter} from 'solid-js'
import { css } from 'solid-styled-components';

export default function LoginForm({
      setEmployee,
      setShiftId
    } : {
      setEmployee : Setter<Employee | null>,
      setShiftId  : Setter<string   | null>
    }){
  let cardR     : HTMLInputElement | undefined
  let passwordR : HTMLInputElement | undefined

  function handleSubmit(e : any) {
    e.preventDefault()
    invoke('login',{cardId: +cardR!.value,password: passwordR!.value})
      .then(() => invoke('check_login')
        .then(employee_and_id => {
           let [emp,id] = employee_and_id as [Employee,string]
           setEmployee(emp)
           setShiftId(id)
        })
      .catch(err =>{
          alert(err);
      }))
      .catch(err => {
          alert(err);
      })
      passwordR!.value = ""
      cardR!.value = ""
      cardR!.focus()
  }

  const container = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  const inputStyle = css({
   display: "block",
   width: "50%",
   fontSize: "20px",
   padding: ".5em",
   margin: ".3em auto",
   backgroundColor:"beige",
   border: "solid 3px",
  })

  return (
    <section class={container}>
      <form onSubmit={handleSubmit}>
        <input ref={cardR} class={inputStyle} type="number" placeholder="رقم التعريف" required/>
        <input ref={passwordR} class={inputStyle} type="password" placeholder="كلمة السر" required/>
        <SubmitButton/>
      </form>
    </section>
  )
}



function SubmitButton(){
  const [hover, setHover] = createSignal(false)

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
        type="submit">تاكيد</button>
  )
}
