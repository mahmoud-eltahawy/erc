import { For, Show } from "solid-js"
import { css } from "solid-styled-components"
import { PermissionsClassified } from "../.."

export default function PermissionsTemplate({
    allowedHandler,
    forbiddenHandler,
    permissions
    } : {
    allowedHandler : Function,
    forbiddenHandler : Function,
    permissions : () => PermissionsClassified | undefined
}){
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
