import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import { useEmployeeAndShiftID, useEmployeeAndShiftIDUpdate } from './employeeProvider'
import Wall from './Wall';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api';

function App() {
  const [employee,shiftId]    = useEmployeeAndShiftID()
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()

  useEffect(() => {
    invoke('check_login')
      .then(emp => setEmployeeAndShiftId(emp))
      .catch(err => console.log(err))
  },[])
  return (
    <section>
      <ShiftIdentity/>
      {employee && shiftId ? <Wall/> : <LoginForm/>}
    </section>
  );
}

export default App;
