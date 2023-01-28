import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import {useEmployee, useEmployeeUpdate} from './employeeProvider'
import Wall from './Wall';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api';

function App() {
  const employee    = useEmployee()
  const setEmployee = useEmployeeUpdate()

  useEffect(() => {
    invoke('check_login')
      .then(emp => setEmployee(emp))
      .catch(err => console.log(err))
  },[])
  return (
    <section>
      <ShiftIdentity/>
      {employee ? <Wall/> : <LoginForm/>}
    </section>
  );
}

export default App;
