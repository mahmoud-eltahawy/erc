import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, For } from "solid-js"
import { createStore } from "solid-js/store"
import {Name} from '../..'

export default function HistoryMachines() {
  const [target,setTarget] = createStore<[string | null]>([null])

  return (
    <section>
      <div class={"problemFormTimeBlock"}>
        <input value={target[0]!}
               onInput={e => setTarget([e.currentTarget.value])}
               class={"problemFormTimeInput"}
               type="text"
               placeholder="ادخل اسم الماكينة"
               required/>
      </div>
      <ShowHistoryParts target={target}/>
    </section>
  )
}

const fetcher = async ({name} : {name : () => string | null}) => {
  return (await invoke("search_machines",{name : name() !== ' ' ? name() : null})) as Name[]
}

function ShowHistoryParts({target} :{target : [string | null]}){
  const [problems,{refetch}] = createResource({name :() => target[0]},fetcher)

  createEffect(() => {
    if (target[0]) {
      refetch()
    }
  })

  return (
    <section>
        <For each={problems()}>
        {item => <div>
                <button>{item.name}</button>
            </div>}
        </For>
    </section>
  )
}
