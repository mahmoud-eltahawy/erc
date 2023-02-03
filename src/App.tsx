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
  const [problems  ,setProblems]     = useState<Name[]>([])
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
      const shotTry = async function() {
        const names : Name[] = await invoke('employees_selection')
        setEmployees(names)
      }
      try{
        await shotTry()
      } catch(err){
        console.log(err)
        try{
          await invoke('update_employees_selection')
          await shotTry()
        }catch(err){
          console.log(err)
        }
      }
    }

    const problemsFun = async function(){
      const shotTry = async function() {
        const names : Name[] = await invoke('problems_selection')
        setProblems(names)
      }
      try{
        await shotTry()
      } catch(err){
        console.log(err)
        try{
          await invoke('update_problems_selection')
          await shotTry()
        }catch(err){
          console.log(err)
        }
      }
    }

    const machinesFun = async function(){
      const shotTry = async function() {
        const names : Name[] = await invoke('machines_selection')
        setMachines(names)
      }
      try{
        await shotTry()
      } catch(err){
        console.log(err)
        try{
          await invoke('update_machines_selection')
          await shotTry()
        }catch(err){
          console.log(err)
        }
      }
    }

    const partsFun = async function(){
      const shotTry = async function() {
        const names : Name[] = await invoke('spare_parts_selection')
        setSpareParts(names)
      }
      try{
        await shotTry()
      } catch(err){
        console.log(err)
        try{
          await invoke('update_spare_parts_selection')
          await shotTry()
        }catch(err){
          console.log(err)
        }
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
    problemsFun()
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
          problems={problems}
          spareParts={spareParts}
          shiftBegin={shiftBegin}
          employee={employee}
          shiftId={shiftId}
          shiftEnd={shiftEnd} /> : <LoginForm/>}
    </section>
  );
}

export default App;
