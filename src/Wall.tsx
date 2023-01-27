import { useEffect, useRef } from "react"
import { useEmployee, useEmployeeUpdate } from "./employeeProvider"


export default function Wall(props:{children:any}){
  const employee = useEmployee()
  const setEmployee = useEmployeeUpdate()
  const the_wall = useRef<HTMLHtmlElement>(null)

  useEffect(() => {
    if(employee) {
        the_wall.current!.style.display = 'block'
    } else {
        the_wall.current!.style.display = 'none'
    }
  },[employee])
  return (
    <section ref={the_wall} style={{display : 'none'}}>
      <button className={"LogoutButton"} onClick={() => {setEmployee(null)}}>تسجيل خروج</button>
      <p className={"NameP"}>
          {employee ? `${employee.first_name} ${employee.middle_name} ${employee.last_name}` : ''}</p>
      {props.children}
    </section>
  )
}
