import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createResource, createSignal, Show } from "solid-js";
import { css } from "solid-styled-components";
import { Name, Note, ShiftProblem } from "../..";
import { permissions } from "../../App";
import LongNote from "./longNote";
import ToggelableList from "./sparePartsList";

const shift_problem_fetcher = async ({
  id,
  } : {
  id : string,
}) => {
  return (await invoke("get_shift_problem_names_by_id", { id })
      .catch(err => console.log(err))) as ShiftProblem
}

const shift_problem_note_fetcher = async ({
  id,
  } : {
  id : string,
}) => {
  return (await invoke("get_shift_problem_note_by_id",{id})
      .catch(err => console.log(err))) as Note | undefined
}

const shift_problem_problems_fetcher = async ({
  id,
  } : {
  id : string,
}) => {
  return (await invoke("get_shift_problem_problems_by_id",{id})
      .catch(err => console.log(err))) as Name[]
}

const shift_problem_spare_parts_fetcher = async ({
  id,
  } : {
  id : string,
}) => {
  return (await invoke("get_shift_problem_spare_parts_by_id",{id})
      .catch(err => console.log(err))) as Name[] | undefined
}

export default function ProblemRow({
    id,
    shiftId,
    problemUpdating,
   } : {
    id : string
    shiftId : string,
    problemUpdating : (beginValues : [ShiftProblem,Name[],Name[] | undefined,Note | undefined]) => void,
}){
  const [problem,pr] = createResource({id}, shift_problem_fetcher)
  const [problems,prs] = createResource({id}, shift_problem_problems_fetcher)
  const [spareParts,sps] = createResource({id}, shift_problem_spare_parts_fetcher)
  const [note,nt] = createResource({id}, shift_problem_note_fetcher)

  listen("update_shift_problem",(e) => {
      let [shift_id,problemId] = e.payload as [string,string]
      if (shift_id === shiftId && problem()?.id === problemId){
          pr.refetch()
      }
  })
  listen("update_shift_problem_note",(e) => {
      if (e.payload === problem()?.id){
          nt.refetch()
      }
  })
  listen("update_shift_problem_problem",(e) => {
      if (e.payload === problem()?.id){
          prs.refetch()
      }
  })
  listen("update_shift_problem_parts",(e) => {
      if (e.payload === problem()?.id){
          sps.refetch()
      }
  })

  const [hover, setHover] = createSignal(false)

  const style = () => {
    const padding = () => hover() ? "17px" : "7px"
    const sides   = () => hover() ? "none" : "solid 2px"
    const sandwitch = () => hover() ? "solid 7px" : "dotted 1px"
    return css({
      paddingLeft: padding(),
      paddingRight: padding(),
      borderRight: sides(),
      borderLeft:  sides(),
      borderBottom: sandwitch(),
      borderTop: sandwitch(),
  })}

  return (
    <tr>
      <Show when={problem()}>
        {
            notNullProblem => <td class={style()}
            onMouseOver={() => setHover(true)}
            onMouseLeave={() => setHover(false)}
        >{
            hover() ? <ModfiyButtons
                        setUpdating={() => problemUpdating([problem()!,problems()!,spareParts(),note()])}
                        problemId={notNullProblem().id}/>
                    :<p>{notNullProblem().writer.name}</p>
        }</td>
        }
      </Show>
      <Show
          when={note()}
          fallback={<td><p>لا يوجد ملحوظات اضافية</p></td>}>
          {
            notNullNote => <td class={style()}> <LongNote content={() => notNullNote().content}/> </td>
          }
      </Show>
      <Show when={problem()}>
          {
            notNullProblem => <td
                                 class={style()}>
                {`من ${notNullProblem().begin_time.slice(0, 5) } الي ${notNullProblem().end_time.slice(0, 5)}`  } </td>
          }
      </Show>
      //TODO ToggelableList Component could work the same with list of strings (string[])
      <Show
          when={spareParts()?.length}
          fallback={<td><p>لم تستخدم اي قطعة غيار</p></td>}>
          {
              <td
                class={style()}> <ToggelableList elements={() => spareParts()!}/>
              </td>
          }
      </Show>
      <Show when={problems()}>
          {
              notNullProblems => <td
                class={style()}><ToggelableList elements={() => notNullProblems()}/>
              </td>
          }
      </Show>
      <Show when={problem()}>
          {
              notNullProblem => <td class={style()}><p> { notNullProblem().maintainer.name} </p></td>
          }
      </Show>
      <Show when={problem()}>
          {
              notNullProblem =><td class={style()}> { notNullProblem().machine.name } </td>
          }
      </Show>
    </tr>
  )
}

function ModfiyButtons({
    problemId,
    setUpdating
    } : {
    problemId : string,
    setUpdating : Function
}){
  enum ModifyButton {
      MODIFY,
      DELETE,
      NONE
  }
  const [hover,setHover] = createSignal(ModifyButton.NONE)
  const modifyStyle = () => css({
      margin: "7px",
      border: "solid 2px",
      borderRadius: hover() === ModifyButton.MODIFY ? "15px" : "0px",
      color: hover() === ModifyButton.MODIFY ? "blue" : "inherit",
      fontSize: hover() === ModifyButton.MODIFY ? "30px" : hover() === ModifyButton.DELETE ? "10px" : "20px",
      width: hover() === ModifyButton.MODIFY ? "90%" : hover() === ModifyButton.DELETE ? "40%" : "70%",
      height: hover() === ModifyButton.MODIFY ? "70%" : hover() === ModifyButton.DELETE ? "15%" : "40%"
  })

  const deleteStyle = () => css({
      margin: "7px",
      border: "solid 2px",
      color: hover() === ModifyButton.DELETE ? "red" : "inherit",
      borderRadius: hover() === ModifyButton.DELETE ? "15px" : "0px",
      fontSize: hover() === ModifyButton.DELETE ? "30px" : hover() === ModifyButton.MODIFY ? "10px" : "20px",
      width: hover() === ModifyButton.DELETE ? "90%" : hover() === ModifyButton.MODIFY ? "40%" : "70%",
      height: hover() === ModifyButton.DELETE ? "70%" : hover() === ModifyButton.MODIFY ? "15%" : "40%"
  })

  const onLeave = () => setHover(ModifyButton.NONE)
  return (
    <div>
      <Show
          when={permissions()?.modify_department_problems}
          fallback={<p>ليس لديك صلاحية التعديل</p>} >
        <button
            class={modifyStyle()}
            onMouseOver={() => setHover(ModifyButton.MODIFY)}
            onMouseLeave={onLeave}
            onclick={() => setUpdating()}
        >تعديل</button>
        <button
            class={deleteStyle()}
            onMouseOver={() => setHover(ModifyButton.DELETE)}
            onMouseLeave={onLeave}
            onClick={async () => invoke("remove_shift_problem", { problemId })
              .catch(err => console.log(err))}
        >حذف</button>
      </Show>
    </div>
  )
}
