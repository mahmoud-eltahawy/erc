import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, For } from "solid-js"
import { createStore } from "solid-js/store"
import {Problem} from '../..'
import LongNote from "../atoms/longNote"

export default function HistoryProblems({department_id}:{department_id : string}){
  const [target,setTarget] = createStore<[string | null]>([null])

  return (
    <section>
      <div class={"problemFormTimeBlock"}>
        <input value={target[0]!}
               onInput={e => setTarget([e.currentTarget.value])}
               class={"problemFormTimeInput"}
               type="text"
               placeholder="ادخل اسم المشكلة"
               required/>
      </div>
      <ShowHistoryProblems target={target} departmentId={department_id}/>
    </section>
  )
}

const fetcher = async ({departmentId,name} : {departmentId : string, name : () => string | null}) => {
  return (await invoke("search_problem",{name : name() !== ' ' ? name() : null,departmentId})) as Problem[]
}

function ShowHistoryProblems({target,departmentId} :{departmentId : string,target : [string | null]}){
  const [problems,{refetch}] = createResource({departmentId,name :() => target[0]},fetcher)

  createEffect(() => {
    if (target[0]) {
      refetch()
    }
  })

  return (
    <section>
        <For each={problems()}>
        {item => <div>
                <button>{item.title}</button>
                <LongNote note={{id : item.id ,content :item.description}} />
            </div>}
        </For>
    </section>
  )
}
