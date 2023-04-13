import { invoke } from "@tauri-apps/api"
import { createEffect,createResource,Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { departmentsNames, PermissionsClassified,Name } from '../..'
import PermissionsTemplate from "../atoms/permissionsTemplate"
import ShowAllToggleButton from "../atoms/showAllToggleButton"
import { ButtonsOrElementLite } from "./buttonsOrElement"

export default function ControllEmployees() {
  const [target,setTarget] = createStore<[string | null]>([null])

  const toggle = () => {
      if(target[0] === '*'){
          setTarget([' '])
          setTarget([null])
      } else {
          setTarget(['*'])
      }
  }

  const container = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  })

  const targetStyle = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor:"lightyellow",
    borderRadius: "20px",
  })

  return (
    <section>
      <div class={container}>
        <input value={target[0]!}
               onInput={e => setTarget([e.currentTarget.value])}
               class={targetStyle}
               type="text"
               placeholder="ادخل اسم الموظف"
               required/>
      </div>
      <ShowAllToggleButton target={target} toggle={toggle}/>
      <Show when={departmentsNames()}>
       {notNullDepartments =>
         <ButtonsOrElementLite
           returnButtonText="العودة لاعدادات الاقسام"
           buttonElementPairs={() => notNullDepartments()
             .map(d => [d.name,<DepartmentEmployees
                                     target={target}
                                     departmentId={d.id}/>])}/>
       }
      </Show>
    </section>
  )
}

const department_employees_names_fetcher = async ({name,departmentId} : {name : () => string | null,departmentId : string}) => {
  return (await invoke("search_department_employees",{name : name() !== ' ' ? name() : null,departmentId})) as Name[]
}

function DepartmentEmployees({target,departmentId} :{target : [string | null],departmentId : string}){
  const [employees,{refetch}] = createResource({name :() => target[0],departmentId},
                                               department_employees_names_fetcher)

  createEffect(() => {
    if (target[0]) {
      refetch()
    }
  })

  return (
    <section>
        <Show when={employees()} fallback={<h1>جاري التحميل ...</h1>}>
          {notNullEmployees =>
            <ButtonsOrElementLite
              buttonElementPairs={
                () => notNullEmployees()
                  .map(x => [x.name, <EmployeePermissions employeeId={x.id}/> ])}
              returnButtonText="العودة لنتائج البحث"/>
          }
        </Show>
    </section>
  )
}

const employee_permissions_fetcher = async ({employeeId} : {employeeId : string}) => {
  return (await invoke("employee_permissions_classified",{employeeId})) as PermissionsClassified
}

function EmployeePermissions({employeeId} : {employeeId : string}){
  const [permissions,{refetch}] = createResource({employeeId},employee_permissions_fetcher)

  const allowedHandler    = async (id : string,permission : string) => {
      await invoke("permission_forbid",{id,permission})
      refetch()
  }

  const forbiddenHandler  = async (id : string,permission : string) => {
      await invoke("permission_allow",{id,permission})
      refetch()
  }

  return (
    <section>
      <Show when={permissions()}>
        {notNullPermissions =>
          <PermissionsTemplate
            allowedHandler={allowedHandler}
            forbiddenHandler={forbiddenHandler}
            permissions={() => notNullPermissions()}/>
        }
      </Show>
    </section>
  )
}
