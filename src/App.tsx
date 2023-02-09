import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import { useEmployeeAndShiftID, useEmployeeAndShiftIDUpdate } from './employeeProvider'
import Wall from './Wall';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { Name } from './main';

function App() {
  const [employee,shiftId]    = useEmployeeAndShiftID()
  const setEmployeeAndShiftId = useEmployeeAndShiftIDUpdate()
  const [shiftBegin, setShiftBegin] = useState('')
  const [shiftEnd,setShiftEnd]       = useState('')
  const [machines  ,setMachines]     = useState<Name[]>([])
  const [employees ,setEmployees]    = useState<Name[]>([])
  const [spareParts,setSpareParts]   = useState<Name[]>([])

  useEffect(() => {
    const isLogedIn = async function(){
      try{
        const employee = await invoke('check_login')
        setEmployeeAndShiftId(employee)
      }catch(err){
        console.log(err)
      }
    }

    const employeesFun = async function(){
      try{
        const names : Name[] = await invoke('employees_selection')
        setEmployees(names)
      } catch(err){
        console.log(err)
      }
    }

    const machinesFun = async function(){
      try{
        const names : Name[] = await invoke('machines_selection')
        setMachines(names)
      } catch(err){
        console.log(err)
      }
    }

    const partsFun = async function(){
      try{
        const names : Name[] = await invoke('spare_parts_selection')
        setSpareParts(names)
      } catch(err){
        console.log(err)
      }
    }

    const bordersFun = async function(){
      try{
        const [begin,end] = await invoke("current_shift_borders") as [string,string]
        setShiftBegin(begin)
        setShiftEnd(end)
      } catch(err){
        console.log(err)
      }
    }
    employeesFun()
    machinesFun()
    partsFun()
    bordersFun()
    isLogedIn()
  },[])
  return (
    <section>
      <ShiftIdentity/>
      {employee && shiftId ? <Wall
          employees={employees}
          machines={machines}
          spareParts={spareParts}
          shiftBegin={shiftBegin}
          employee={employee}
          shiftId={shiftId}
          shiftEnd={shiftEnd} /> : <LoginForm/>}
    </section>
  );
}

export default App;
