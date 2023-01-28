import { invoke } from '@tauri-apps/api/tauri';
import { BaseSyntheticEvent, useRef } from 'react';
import { useEmployeeUpdate } from './employeeProvider';

export default function LoginForm(){
  const setEmployee = useEmployeeUpdate()
  const cardR = useRef<HTMLInputElement>(null)
  const passwordR = useRef<HTMLInputElement>(null)

  function handleSubmit(e : BaseSyntheticEvent) {
    e.preventDefault()
    let cardC   = cardR.current!
    let passwordC = passwordR.current!
    invoke('login',{cardId: +cardC.value,password: passwordC.value})
      .then(() => invoke('check_login')
        .then(employee => {
          setEmployee(employee as any)
        })
      .catch(err =>{
          alert(err);
      }))
      .catch(err => {
          alert(err);
      })
      passwordC.value = ""
      cardC.value = ""
      cardC.focus()
  }


  return (
    <section className={"LoginFormContainer"}>
      <form onSubmit={handleSubmit}>
        <input ref={cardR} className={"LoginFormInput"} type="number" placeholder="رقم التعريف" required/>
        <input ref={passwordR} className={"LoginFormInput"} type="password" placeholder="كلمة السر" required/>
        <button type="submit">تاكيد</button>
      </form>
    </section>
  )
}
