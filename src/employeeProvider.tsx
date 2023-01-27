import React, { useContext, useState } from 'react'

const employeeContext = React.createContext<any>(null)
const employeeUpdateContext = React.createContext<any>(null)

export function useEmployee(){
    return useContext(employeeContext)
}

export function useEmployeeUpdate(){
    return useContext(employeeUpdateContext)
}

export function EmployeeProvider(probs:{children : any}){
    const [employee,setEmployee] = useState<any>(null)

    return (
        <employeeContext.Provider value={employee}>
            <employeeUpdateContext.Provider value={setEmployee}>
                {probs.children}
            </employeeUpdateContext.Provider>
        </employeeContext.Provider>
    )
}
