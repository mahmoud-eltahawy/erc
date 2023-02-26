import LoginForm from './components/organisms/LoginForm'
import ShiftIdentity from './components/molecules/ShiftIdentity'
import Wall from './wall';
import { createSignal, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api';
import { Employee } from './index';

function App() {
  const [employee,setEmployee] = createSignal<Employee | null>(null)
  const [shiftId,setShiftId]   = createSignal<string | null>(null)

  const isLogedIn = async function(){
    try{
      const [employee,shiftId] = await invoke('check_login') as [Employee,string]
      setEmployee(employee)
      setShiftId(shiftId)
    }catch(err){
      console.log(err)
    }
  }

  onMount(() => {
    isLogedIn()
  })

  setInterval(async () => {
    try{
      isLogedIn()
      await invoke("update")
    }catch(err){
      console.log(err)
    }
  },60000)

  return (
    <section>
      <ShiftIdentity/>
      <Show
          when={employee() && shiftId()}
          fallback={<LoginForm setEmployee={setEmployee} setShiftId={setShiftId}/>}>
        <Wall employee={employee()!} shiftId={shiftId()!} setEmployee={setEmployee} setShiftId={setShiftId}/>
      </Show>
    </section>
  );
}

export default App;
