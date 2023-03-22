import { invoke } from "@tauri-apps/api"
import { createResource, For, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { Name } from "../.."


const non_admins_fetcher = async ({name} : {name : () => string | null}) => {
  return (await invoke("search_non_admins",{name : name() !== ' ' ? name() : null})) as Name[]
}

const admins_fetcher = async () => {
  return (await invoke("search_admins")) as Name[]
}

const [target, setTarget] = createStore<[string | null]>([null])
const [nonAdmins,na] = createResource({name :() => target[0]},non_admins_fetcher)
const [admins,a]  = createResource(admins_fetcher)
const refetch = () => {
    na.refetch()
    a.refetch()
}

export default function ControllAdmins() {

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
          placeholder={"ابحث عن موظف للتمكين"}
          class={inputStyle}
          type="text"
          value={target[0]!}
          onInput={e => {
            setTarget([e.currentTarget.value])
            na.refetch()
          }} />
        <section class={viewContainer}>
          <AdminsSection/>
          <NonAdminSection/>
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

function AdminsSection(){

  const handler = async (id : string) => {
      await invoke("unadmin_employee",{id})
      refetch()
  }

  return (
    <select multiple size={9} class={viewMember}>
         {
             <For each={admins()}>
                 {
                     (item) => (
                       <option onClick={() => handler(item.id)}>{item.name}</option>
                     )
                 }
             </For>
         }
         <Show when={!(admins() || []).length}><option disabled>لا يوجد موظفين ممكنين</option></Show>
     </select>
  )
}

function NonAdminSection(){

  const handler = async (id : string) => {
      await invoke("admin_employee",{id})
      refetch()
  }

  return (
    <select multiple size={9} class={viewMember}>
      {
          <For each={nonAdmins()}>
              {
                  (item) => (
                    <option onClick={() => handler(item.id)}>{item.name}</option>
                  )
              }
          </For>
      }
      <Show when={!(nonAdmins() || []).length}><option disabled>لا يوجد موظفين</option></Show>
    </select>
  )
}
