import { invoke } from "@tauri-apps/api"
import { createResource, createSignal, For, Show } from "solid-js"
import { css } from "solid-styled-components"
import { Name } from "../.."
import { ButtonsOrElement } from "./buttonsOrElement"

const departments_fetcher = async () => {
  return (await invoke("list_departments")) as Name[]
}

export default function ControllDepartments(){
  const [departments] = createResource(departments_fetcher)

  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
      <section class={container}>
          {<ButtonsOrElement
               returnButtonText="العودة لاعدادات الاقسام"
               buttonElementPairs={() => (departments() || [])
                 .map(d => [d.name, <DepartmentSettings id={d.id} />])}
               num={[-1]}
               fun={() => console.log("later")}/>}
      </section>
  )
}

type NativeDepartment = {
   id            : string,
   boss_id       : string | null,
   department_id : string | null,
   name          : string,
}

type Department = {
   id            : string,
   boss          : Name   | null,
   department_id : string | null,
   name          : string,
   employees     : Name[]
}

const department_fetcher = async ({id} : {id : string}) => {
  let department : Department
  let nd = (await invoke("find_department",{id})) as NativeDepartment
  let employees = (await invoke("department_employees",{id})) as Name[]
  if (nd.boss_id){
    let name : string = (await invoke("employee_name",{id : nd.boss_id})) as string
    let boss : Name = {id : nd.boss_id,name}
    department = {id : nd.id ,boss,name : nd.name,department_id : nd.department_id,employees}
  } else {
    department = {id:nd.id,department_id : nd.department_id,name : nd.name,boss : null,employees}
  }
  return department
}

function DepartmentSettings({id} : {id : string}){
  const [department,{refetch}]  = createResource({id},department_fetcher)
  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
    <Show when={department()}>
      <section class={container}>
          {<ButtonsOrElement
               returnButtonText={"العودة الي " + department()?.name}
               buttonElementPairs={() => [
                 ["اختيار رئيس القسم", () => <ChooseBoss
                                                department={() => department()}
                                                refetch={() => refetch()} />],
                 ["صلاحيات القسم", () => <PermissionsElem
                                            departmentId={id} />],
               ]}
               num={[-1]}
               fun={() => console.log("later")}/>}
      </section>
    </Show>
  )
}

const department_permissions_fetcher = async ({departmentId} : {departmentId : string}) => {
  return (await invoke("department_permissions",{departmentId})) as {
    id        : string,
               //client backend
    allowed   : [string,string][],
    forbidden : [string,string][],
  }
}

function PermissionsElem({departmentId} : {departmentId : string}){
  const [permissions,{refetch}] = createResource({departmentId},department_permissions_fetcher)

  const allowedHandler    = async (id : string,permission : string) => {
      await invoke("permission_forbid",{id,permission})
      refetch()
  }

  const forbiddenHandler  = async (id : string,permission : string) => {
      await invoke("permission_allow",{id,permission})
      refetch()
  }

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
  })

  const viewMember = css({
    display: "inline-block",
    fontSize: "20px",
    margin: "20px auto",
    width: "48%",
    backgroundColor: "inherit",
    borderLeft: "solid 5px",
    borderRight: "solid 5px",
    borderBottom: "solid 5px",
    borderTop: "none",
    borderBottomLeftRadius : "20px",
    borderBottomRightRadius : "20px",
  })

  const allowed   = () => permissions()?.allowed
  const forbidden = () => permissions()?.forbidden

  return (
    <section class={viewContainer}>
      <select multiple size={(allowed() || []).length + 1} class={viewMember}>
        {
            <For each={allowed()}>
                {
                    (item) => (
                      <option onClick={() => allowedHandler(permissions()!.id,item[1])}>{item[0]}</option>
                    )
                }
            </For>
        }
        <Show when={!(allowed() || []).length}><option disabled>{"لا توجد صلاحيات"}</option></Show>
      </select>
      <select multiple size={(forbidden() || []).length + 1} class={viewMember}>
        {
            <For each={forbidden()}>
                {
                    (item) => (
                      <option onClick={() => forbiddenHandler(permissions()!.id,item[1])}>{item[0]}</option>
                    )
                }
            </For>
        }
        <Show when={!(forbidden() || []).length}><option disabled>{"لا توجد صلاحيات"}</option></Show>
      </select>
    </section>
  )
}

function ChooseBoss({department,refetch} : {department : () => Department | undefined,refetch : Function}){
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
    <Show when={department()} fallback={<h1> ...جاري التحميل</h1>}>
      <h1 class={css({fontSize: "20px"})}>رئيس القسم : {department()?.boss?.name ? department()?.boss?.name: 'لا يوجد'}</h1>
      <input
        class={inputStyle}
        type="text"
        value={target()}
        onInput={e => {
          setTarget(e.currentTarget.value)
        }}/>
      <select multiple size={department()?.employees.length} class={viewMember}>
        <For each={department()?.employees.filter(m => m.name.includes(target()!))}>
          {item => <option onClick={() => optionHandler(item.id)}>{item.name}</option>}
        </For>
      </select>
    </Show>
  )
}
