import { invoke } from '@tauri-apps/api/tauri';
import { Employee } from '../../index';
import {Setter} from 'solid-js'

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


  return (
    <section class={"LoginFormContainer"}>
      <form onSubmit={handleSubmit}>
        <input ref={cardR} class={"LoginFormInput"} type="number" placeholder="رقم التعريف" required/>
        <input ref={passwordR} class={"LoginFormInput"} type="password" placeholder="كلمة السر" required/>
        <button type="submit">تاكيد</button>
      </form>
    </section>
  )
}
