import { invoke } from "@tauri-apps/api"
import { createResource, createSignal, For, JSXElement, Show } from "solid-js"
import { css } from "solid-styled-components"
import { Name,departmentsNames, NativeDepartment, PermissionsClassified } from "../.."
import PermissionsTemplate from "../atoms/permissionsTemplate"
import { ButtonsOrElement } from "./buttonsOrElement"

export default function ControllDepartments(){
  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
    <Show when={departmentsNames()}>
      {notNullDepartments =>
        <section class={container}>
          {<ButtonsOrElement
               returnButtonText="العودة لاعدادات الاقسام"
            buttonElementPairs={() => notNullDepartments()
                .filter(d => d.id !== "00000000-0000-0000-0000-000000000000")
                .map(d => [d.name,() => <DepartmentSettings departmentId={d.id} />])}
               num={[-1]}
               fun={() => console.log("later")}/>}
        </section>
      }
    </Show>
  )
}

type Department = {
   id            : string,
   boss          : Name   | null,
   department_id : string | null,
   name          : string,
   employees     : Name[]
}

const department_fetcher = async ({departmentId} : {departmentId : string}) => {
  let department : Department
  let nd = (await invoke("find_department",{id : departmentId})) as NativeDepartment
  let employees = (await invoke("department_employees",{id : departmentId})) as Name[]
  if (nd.boss_id){
    let name : string = (await invoke("employee_name",{id : nd.boss_id})) as string
    let boss : Name = {id : nd.boss_id,name}
    department = {id : nd.id ,boss,name : nd.name,department_id : nd.department_id,employees}
  } else {
    department = {id:nd.id,department_id : nd.department_id,name : nd.name,boss : null,employees}
  }
  return department
}


const department_permissions_fetcher = async ({departmentId} : {departmentId : string}) => {
    return (await invoke("department_permissions", { departmentId })
        .catch(err => {
            console.log(err)
            return []
    })) as PermissionsClassified
}

function DepartmentSettings({departmentId} : {departmentId : string}){
  const [permissions,dbf]      = createResource({departmentId},department_permissions_fetcher)
  const [department,{refetch}] = createResource({departmentId},department_fetcher)

  const allowedHandler    = async (id : string,permission : string) => {
      await invoke("permission_forbid",{id,permission})
      dbf.refetch()
  }

  const forbiddenHandler  = async (id : string,permission : string) => {
      await invoke("permission_allow",{id,permission})
      dbf.refetch()
  }
  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
    <section class={container}>
      <Show when={department()}>
        {notNullDepartment =>
          <ButtonsOrElement
             returnButtonText={"العودة الي " + notNullDepartment().name}
             buttonElementPairs={() => {
                 const pairs : (string | (() => JSXElement))[][] = [
                 ["اختيار رئيس القسم",() => <ChooseBoss
                                        department={() => notNullDepartment()}
                                        refetch={() => refetch()} />],
                 ["صلاحيات القسم",() => <PermissionsTemplate
                                        allowedHandler={allowedHandler}
                                        forbiddenHandler={forbiddenHandler}
                                        permissions={() => permissions()!} />], //TODO remove this (!)
               ]

               return pairs;
             }}
             num={[-1]}
             fun={() => console.log("later")}/>
        }
      </Show>
    </section>
  )
}

function ChooseBoss({department,refetch} : {department : () => Department,refetch : Function}){
  const [target, setTarget]     = createSignal<string>('')

  const optionHandler = async (id : string) => {
      await invoke("boss_employee",{id})
      refetch()
  }

  const viewMember = css({
    display: "inline-block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
    borderLeft: "solid 5px",
    borderRight: "solid 5px",
    borderBottom: "solid 5px",
    borderTop: "none",
    borderBottomLeftRadius : "20px",
    borderBottomRightRadius : "20px",
  })

  const inputStyle = css({
    display: "block",
    backgroundColor: "transparent",
    fontSize: "24px",
    width: "70%",
    padding: ".1em",
    margin: ".1em auto",
  })

  return (
    <section>
      <h1 class={css({fontSize: "20px"})}>رئيس القسم : {department().boss?.name ? department().boss?.name: 'لا يوجد'}</h1>
      <input
        class={inputStyle}
        type="text"
        value={target()}
        onInput={e => {
          setTarget(e.currentTarget.value)
        }}/>
      <select multiple size={department().employees.length} class={viewMember}>
        <For each={department().employees.filter(m => m.name.includes(target()!))}>
          {item => <option onClick={() => optionHandler(item.id)}>{item.name}</option>}
        </For>
      </select>
    </section>
  )
}
