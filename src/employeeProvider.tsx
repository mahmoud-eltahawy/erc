import React, { useContext, useState } from 'react'
import { EmployeeAndShiftID } from './main'

const employeeAndShiftIDContext = React.createContext<EmployeeAndShiftID | [null,null]>([null,null])
const employeeAndShiftIDUpdateContext = React.createContext<any>(null)

export function useEmployeeAndShiftID(){
    return useContext(employeeAndShiftIDContext)
}

export function useEmployeeAndShiftIDUpdate(){
    return useContext(employeeAndShiftIDUpdateContext)
}

export function EmployeeAndShiftIDProvider(probs:{children : any}){
    const [employee,setEmployee] = useState<EmployeeAndShiftID | [null,null]>([null,null])

    return (
        <employeeAndShiftIDContext.Provider value={employee}>
            <employeeAndShiftIDUpdateContext.Provider value={setEmployee}>
                {probs.children}
            </employeeAndShiftIDUpdateContext.Provider>
        </employeeAndShiftIDContext.Provider>
    )
}
