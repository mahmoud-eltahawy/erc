import { invoke } from '@tauri-apps/api/tauri';
import { BaseSyntheticEvent, useRef, useEffect, useState } from 'react';

export default function LoginForm(){
  let [current_employee, setEmployee] = useState(null)
  const formR = useRef<HTMLFormElement>(null)
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

  useEffect(() => {
      if(current_employee){
          formR.current!.style.display = "none"
      }
  },[current_employee])

  return (
    <section ref={formR} className={"LoginFormContainer"}>
      <form onSubmit={handleSubmit}>
        <input ref={cardR} className={"LoginFormInput"} type="number" placeholder="رقم التعريف" required/>
        <input ref={passwordR} className={"LoginFormInput"} type="password" placeholder="كلمة السر" required/>
        <button type="submit">تاكيد</button>
      </form>
    </section>
  )
}
