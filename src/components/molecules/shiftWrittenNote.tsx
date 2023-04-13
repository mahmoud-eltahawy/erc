import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { createEffect, createResource, createSignal, For, Show } from "solid-js"
import { css } from "solid-styled-components"
import { problemsFetcher, ShiftProblem } from "../.."
import { permissions } from "../../App"
import ProblemRow from "../atoms/problemRow"
import TableHead from "../atoms/problemTableHead"
import togglingButton from "../atoms/problemTogglingButton"
import { ButtonsOrElementLite } from "./buttonsOrElement"
import { existing_employees_fetcher } from "./setShiftEmployees"

export default function ShiftWrittenShow({
    shiftId,
  } : {
    shiftId : () => string,
  }
){
  const container = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
    <section class={container}>
      <ButtonsOrElementLite
        returnButtonText="العودة لبيانات الوردية"
        buttonElementPairs={() => [
              ["اظهار الاعطال",<ShiftProblems
                                     shiftId={shiftId()}/>],
              ["اظهار الملحوظات",<ShiftNotes
                                     shiftId={shiftId()} />],
              ["اظهار الموظفين",<ExistingEmployees
                                     shiftId={shiftId()} />]
        ]}/>
    </section>
  )
}

function ExistingEmployees({shiftId} : {shiftId : string}){
  const [existing] = createResource({shift_id :() => shiftId},existing_employees_fetcher)

  const viewMember = css({
    display: "block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
  })

  return (
      <ol class={viewMember}>
           <For each={existing()}>
               {
                   (item) => (
                     <li>{item.name}</li>
                   )
               }
           </For>
         <Show when={!(existing() || []).length}><li>لا يوجد موظفين مسجلين</li></Show>
     </ol>
  )
}

const notes_fetcher = async ({shiftId} : {shiftId : string}) => {
  return (await invoke("fetch_shift_notes",{shiftId})
      .catch(err => console.log(err))) as string[]
}

function ShiftNotes({
    shiftId,
    } : {
    shiftId : string,
}){
  const [notes] = createResource({shiftId},notes_fetcher)

  return (
    <section>
      <For each={notes()}>
          {
              (note) => <p>{note}</p>
          }
      </For>
    </section>
  )
}

function ShiftProblems({
    shiftId,
    } : {
    shiftId : string,
}){
  const limit = 4
  const [shiftProblems, { refetch }] = createResource(shiftId, problemsFetcher)
  const [state, setState] = createSignal<ShiftProblem[] | undefined>([])
  const [tooLong,setTooLong] = createSignal(state.length > limit)

  listen("update_shift_problem",() => {
    setTimeout(() => refetch(),2000)
  })

  createEffect(() => {
    if(tooLong()) {
        if(shiftProblems()){
          setState(shiftProblems()!.slice(0,limit))
        } else {
          setState(undefined)
        }
    } else {
      setState(shiftProblems())
    }
  })

  const style = css({
    borderCollapse: "collapse",
    width: "99%"
  })

  return (
    <section>
       <Show when={permissions()?.read_department_problems}
            fallback={<h1>ليس لديك صلاحية قراءة اعطال الوردية</h1>} >
        <Show when={permissions()?.modify_department_problems}>
          <h1>امكانية التعديل قريبا</h1>
        </Show>
        <table class={style}>
        <TableHead/>
          <Show when={state()} fallback={<h1>جاري التحميل ...</h1>}>
            {notNullState =>
              <tbody>
                <For each={notNullState()}>
                 {problem => <ProblemRow problem={problem}/>}
                </For>
              </tbody>
            }
          </Show>
        </table>
          {togglingButton({
            showButton : () => (shiftProblems() || []).length > limit,
            showMore   : () => tooLong(),
            doOnClick  : () => setTooLong(!tooLong())})}
      </Show>
    </section>
  )
}
