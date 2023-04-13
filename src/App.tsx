import ShiftIdentity from './components/molecules/ShiftIdentity'
import Wall from './wall';
import { createSignal, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api';
import { Employee, Permissions } from './index';
import { listen } from '@tauri-apps/api/event';
import { css } from 'solid-styled-components';
import SubmitButton from './components/atoms/submitButton';

export const [employee,setEmployee] = createSignal<Employee | null>(null)
export const [shiftId,setShiftId]   = createSignal<string | null>(null)
export const [permissions,setPermissions]   = createSignal<Permissions | null>(null)


function App() {
  const isLogedIn = async function(){
    try{
      const [employee,shiftId] = await invoke('check_login') as [Employee,string]
      const permissions = await invoke('employee_permissions',{id : employee.id}) as Permissions
      setEmployee(employee)
      setShiftId(shiftId)
      setPermissions(permissions)
    }catch(err){
      console.log(err)
    }
  }

  onMount(() => {
    isLogedIn()
  })

  listen("shift_ended", () => {
    isLogedIn()
  })

  listen("update_permissions",() => {
    isLogedIn()
  })

  listen("new_login",async () => {
    isLogedIn()
  })

  onMount(() => invoke("update"))

  return (
    <section>
      <ShiftIdentity/>
      <Show
          when={shiftId() && employee() && permissions()}
          fallback={<LoginForm/>}>
           <Wall/>
      </Show>
    </section>
  )
}

function LoginForm(){
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
        <input
            ref={cardR}
            class={inputStyle}
            type="number"
            placeholder="رقم التعريف"
            required/>
        <input
            ref={passwordR}
            class={inputStyle}
            type="password"
            placeholder="كلمة السر"
            required/>
        <SubmitButton length={undefined} />
      </form>
    </section>
  )
}

export default App;
