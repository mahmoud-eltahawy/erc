import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { createResource, For, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { Name } from "../.."
import { employee, shiftId } from "../../App"

const non_existing_fetcher = async () => {
  return (await invoke("shift_non_existing_employees",{shiftId : shiftId(),departmentId : employee()!.department_id})) as Name[]
}

export const existing_employees_fetcher = async ({shift_id} : {shift_id : () => string | null}) => {
  return (await invoke("shift_existing_employees",{shiftId : shift_id()})) as Name[]
}

export default function SetShiftEmployees() {
  const [target, setTarget] = createStore<[string | null]>([null])
  const [nonExisting,na] = createResource(non_existing_fetcher)
  const [existing,a] = createResource({shift_id : shiftId},existing_employees_fetcher)

  listen("update_department_shift_employee",(e) => {
    if(e.payload === shiftId()) {
      na.refetch()
      a.refetch()
    }
  })

  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  })

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
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
    <div class={container} >
        <input
          placeholder={"ابحث عن موظف للتسجيل"}
          class={inputStyle}
          type="text"
          value={target[0]!}
          onInput={e => {
            setTarget([e.currentTarget.value])
            na.refetch()
          }} />
        <section class={viewContainer}>
          <ExistingSection existing={() => existing() || []}/>
          <NonExistingSection nonExisting={() => nonExisting() || []}/>
        </section>
    </div>
  )
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

function ExistingSection({
    existing,
    } : {
    existing : () => Name[],
}){

  const handler = async ({employeeId} :{employeeId : string}) => {
      await invoke("remove_shift_employee",{employeeId,shiftId : shiftId()})
  }

  return (
    <select multiple size={existing().length} class={viewMember}>
         {
             <For each={existing()}>
                 {
                     (item) => (
                       <option onClick={() => handler({employeeId : item.id})}>{item.name}</option>
                     )
                 }
             </For>
         }
         <Show when={!(existing() || []).length}><option disabled>لا يوجد موظفين مسجلين</option></Show>
     </select>
  )
}

function NonExistingSection({
    nonExisting,
    } : {
    nonExisting : () => Name[],
}){
  const handler = async ({employeeId} :{employeeId : string}) => {
      await invoke("add_shift_employee",{employeeId,shiftId : shiftId()})
  }

  return (
    <select multiple size={nonExisting().length} class={viewMember}>
      {
          <For each={nonExisting()}>
              {
                  (item) => (
                      <option onClick={() => handler({employeeId : item.id})}>{item.name}</option>
                  )
              }
          </For>
      }
      <Show when={!(nonExisting() || []).length}><option disabled>لا يوجد موظفين</option></Show>
    </select>
  )
}
