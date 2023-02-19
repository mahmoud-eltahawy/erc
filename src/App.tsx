import LoginForm from './components/organisms/LoginForm'
import ShiftIdentity from './components/molecules/ShiftIdentity'
import { useEmployeeAndShiftID, useEmployeeAndShiftIDUpdate } from './components/providers/employeeProvider'
import Wall from './Wall';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { Employee } from './main';

function App() {
  const [employee,shiftId]    = useEmployeeAndShiftID()
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()

  useEffect(() => {
    const isLogedIn = async function(){
      try{
        const [employee,shiftId] = await invoke('check_login') as [Employee,string]
        setEmployeeAndShiftId([employee,shiftId])
      }catch(err){
        console.log(err)
      }
    }
    isLogedIn()
  },[])
  return (
    <section>
      <ShiftIdentity/>
      {employee && shiftId ? <Wall/> : <LoginForm/>}
    </section>
  );
}

export default App;
