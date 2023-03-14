import { invoke } from "@tauri-apps/api"
import { createEffect,createResource,Show } from "solid-js"
import { createStore } from "solid-js/store"
import { Name } from '../..'
import { ButtonsOrElement } from "./buttonsOrElement"

export default function HistoryEmployees() {
  const [target,setTarget] = createStore<[string | null]>([null])

  const toggle = () => {
      if(target[0] === '*'){
          setTarget([' '])
          setTarget([null])
      } else {
          setTarget(['*'])
      }
  }

  return (
    <section>
      <div class={"problemFormTimeBlock"}>
        <input value={target[0]!}
               onInput={e => setTarget([e.currentTarget.value])}
               class={"problemFormTimeInput"}
               type="text"
               placeholder="ادخل اسم الموظف"
               required/>
      </div>
      <button onClick={toggle}>{target[0] === '*' ? "شاهد اقل" : "شاهد الكل"}</button>
      <ShowHistory target={target}/>
    </section>
  )
}

const fetcher = async ({name} : {name : () => string | null}) => {
  return (await invoke("search_employees",{name : name() !== ' ' ? name() : null})) as Name[]
}

function ShowHistory({target} :{target : [string | null]}){
  const [employees,{refetch}] = createResource({name :() => target[0]},fetcher)

  createEffect(() => {
    if (target[0]) {
      refetch()
    }
  })

  return (
    <section>
        <Show when={!employees.loading} fallback={<h1>جاري التحميل ...</h1>}>
          <ButtonsOrElement
            buttonElementPairs={() => (employees() || []).map(x => [x.name, () => <h1> employee profile </h1>])}
            num={[-1]}
            fun={() => console.log("fun")}
            returnButtonText="العودة لنتائج البحث"/>
        </Show>
    </section>
  )
}
