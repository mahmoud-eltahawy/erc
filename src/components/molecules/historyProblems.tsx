import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { Name } from '../..'
import { ButtonsOrElement } from "./buttonsOrElement"

export default function HistoryProblems({department_id}:{department_id : string}){
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
               placeholder="ادخل اسم المشكلة"
               required/>
      </div>
      <button onClick={toggle}>{target[0] === '*' ? "شاهد اقل" : "شاهد الكل"}</button>
      <ShowHistory target={target} departmentId={department_id}/>
    </section>
  )
}

const fetcher = async ({departmentId,name} : {departmentId : string, name : () => string | null}) => {
  return (await invoke("search_problem",{name : name() !== ' ' ? name() : null,departmentId})) as Name[]
}

function ShowHistory({target,departmentId} :{departmentId : string,target : [string | null]}){
  const [problems,{refetch}] = createResource({departmentId,name :() => target[0]},fetcher)

  createEffect(() => {
    if (target[0]) {
      refetch()
    }
  })

  return (
    <section>
        <Show when={!problems.loading} fallback={<h1>جاري التحميل ...</h1>}>
          <ButtonsOrElement
            buttonElementPairs={() => (problems() || []).map(x => [x.name, () => <Profile id={x.id}/>])}
            num={[-1]}
            fun={() => console.log("fun")}
            returnButtonText="العودة لنتائج البحث"/>
        </Show>
    </section>
  )
}

export type Profile = {
    department_name : string,
    writer_name     : string,
    title           : string,
    description     : string
}

const profiler = async ({id} : {id : string}) => {
  return (await invoke("profile_problem", { id })) as Profile
}

function Profile({id} : {id : string}){
    const [profile] = createResource({id},profiler)
    return (
        <section>
          <table class="profileTable">
                <thead>
                  <tr>
                    <th>الاسم</th>
                    <th>القسم</th>
                    <th>المؤلف</th>
                    <th>الوصف</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>{profile()?.title}</td>
                    <td>{profile()?.department_name}</td>
                    <td>{profile()?.writer_name}</td>
                    <td><p>{profile()?.description}</p></td>
                  </tr>
                </tbody>
            </table>
        </section>
    )
}
