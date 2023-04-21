import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { createEffect, createResource, createSignal, For, Show } from "solid-js"
import { css } from "solid-styled-components"
import { Employee, Name, Note, ShiftProblem } from "../.."
import { permissions } from "../../App"
import ProblemRow from "../atoms/problemRow"
import TableHead from "../atoms/problemTableHead"
import togglingButton from "../atoms/problemTogglingButton"
import { ProblemUpdateForm } from "../organisms/ProblemForm"
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

function ExistingEmployees({
    shiftId
    } : {
    shiftId : string
}){
  const [existing,{refetch}] = createResource({shift_id :() => shiftId},existing_employees_fetcher)

  listen("update_department_shift_employee",() => refetch())

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

function ShiftNotes({
    shiftId,
    } : {
    shiftId : string,
}){
  const notes_fetcher = async ({shiftId} : {shiftId : string}) => {
      return (await invoke("fetch_shift_notes",{shiftId})
          .catch(err => console.log(err))) as [string,string][]
  }

  const [notes,{refetch}] = createResource({shiftId},notes_fetcher)

  listen("update_shift_note", () => refetch())

  function Note({employee_id,note} : {employee_id : string,note : string}){
    const fetcher = async ({id} : {id : string}) => {
        return (await invoke("get_employee_by_id",{id})
            .catch(err => console.log(err))) as Employee
    }
    const [employee] = createResource({id : employee_id},fetcher)

    const style = css({
        padding : "5px",
        margin : "3px",
        borderBottom: "solid 2px"
    })

    return (
      <Show when={employee()}>
        <tr>
            <td class={style}>{employee()?.first_name} {employee()?.middle_name} {employee()?.last_name}</td>
            <td class={style}>{note}</td>
        </tr>
      </Show>
    )
  }

  const style = css({
    borderCollapse: "collapse",
    width: "99%"
  })

  return (
    <table class={style}>
      <thead>
        <tr>
          <th>الكاتب</th>
          <th>الملحوظة</th>
        </tr>
      </thead>
      <tbody>
        <For each={notes()}>
          {
            (note) => <Note employee_id={note[0]} note={note[1]}/>
          }
        </For>
      </tbody>
    </table>
  )
}

export const shift_shift_problems_ids_fetcher = async ({
  id,
  } : {
  id : string,
}) => {
  return (await invoke("get_shift_problems_ids_by_shift_id",{id})) as string[]
}

function ShiftProblems({
    shiftId,
    } : {
    shiftId : string,
}){
  const limit = 4
  let problem_to_update : [ShiftProblem,Name[],Name[] | undefined,Note | undefined] | null = null
  const get_problem_to_update = () => problem_to_update

  const [shiftProblemsIds, { refetch,mutate }] = createResource({id : shiftId}, shift_shift_problems_ids_fetcher)
  const [state, setState] = createSignal<string[] | undefined>([])
  const [tooLong,setTooLong] = createSignal((state() || []).length < limit)
  const [updatating,setUpdating] = createSignal(false)

  listen("delete_shift_problem",(e) => {
      let [shift_id,problemId] = e.payload as [string,string]
      if (shift_id === shiftId){
          if((shiftProblemsIds() || []).length > limit){
              mutate(list => list!.filter(x => x !== problemId))
          } else {
              refetch()
          }
      }
  })
  listen("create_shift_problem",(e) => {
      let [shift_id,problemId] = e.payload as [string,string]
      if (shift_id === shiftId){
          setTimeout(() => {
              if((shiftProblemsIds() || []).length > limit){
                  mutate(list => [problemId,...(list || [])])
              } else {
                  refetch()
              }
          },1000)
      }
  })

  createEffect(() => {
    if(tooLong()) {
        if(shiftProblemsIds()){
          setState(shiftProblemsIds()!.slice(0,limit))
        } else {
          setState(undefined)
        }
    } else {
      setState(shiftProblemsIds())
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
           <Show
               when={!updatating()}
                  fallback={<ProblemUpdateForm
                    beginValues={get_problem_to_update()!}
                    toggle={() => setUpdating(false)}/>}>
                <table class={style}>
                  <TableHead/>
                  <Show when={state()} fallback={<h1>جاري التحميل ...</h1>}>
                    {notNullIdList =>
                      <tbody>
                        <For each={notNullIdList()}>
                          {id => <ProblemRow
                                       problemUpdating={(values) => {
                                           problem_to_update = values
                                           setUpdating(true)
                                       }}
                                       shiftId={shiftId}
                                       id={id}/>}
                        </For>
                      </tbody>
                    }
                  </Show>
                </table>
           </Show>
          {togglingButton({
            showButton : () => (shiftProblemsIds() || []).length > limit,
            showMore   : () => tooLong(),
            doOnClick  : () => setTooLong(!tooLong())})}
      </Show>
    </section>
  )
}
