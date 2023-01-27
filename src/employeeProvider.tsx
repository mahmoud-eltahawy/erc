import React, { useContext, useState } from 'react'
import { Employee } from './main'

const employeeContext = React.createContext<Employee | null>(null)
const employeeUpdateContext = React.createContext<any>(null)

export function useEmployee(){
    return useContext(employeeContext)
}

export function useEmployeeUpdate(){
    return useContext(employeeUpdateContext)
}

export function EmployeeProvider(probs:{children : any}){
    const [employee,setEmployee] = useState<Employee | null>(null)

    return (
        <employeeContext.Provider value={employee}>
            <employeeUpdateContext.Provider value={setEmployee}>
                {probs.children}
            </employeeUpdateContext.Provider>
        </employeeContext.Provider>
    )
}
