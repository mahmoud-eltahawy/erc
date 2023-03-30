import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, createSignal, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { departmentsNames, Name } from '../..'
import { ButtonsOrElement } from "./buttonsOrElement"

export default function HistoryProblems({department_id}:{department_id : string | null}){
  const [target,setTarget] = createStore<[string | null]>([null])

  const toggle = () => {
      if(target[0] === '*'){
          setTarget([' '])
          setTarget([null])
      } else {
          setTarget(['*'])
      }
  }

  const container = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  })

  const targetStyle = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor:"lightyellow",
    borderRadius: "20px",
  })

  return (
    <section>
      <div class={container}>
        <input value={target[0]!}
               onInput={e => setTarget([e.currentTarget.value])}
               class={targetStyle}
               type="text"
               placeholder="ادخل اسم المشكلة"
               required/>
      </div>
      <ShowAllToggleButton target={target} toggle={toggle}/>
      <Show
          when={department_id}
          fallback={<ShowAllHistory target={() => target}/>}>
        <ShowHistory
            target={target}
            departmentId={department_id!}/>
      </Show>
    </section>
  )
}

function ShowAllToggleButton({toggle,target} : {toggle : Function,target : [string | null]}){
  const [hover, setHover] = createSignal(false)

  const style = () => css({
   display: "block",
   width: "25%",
   borderRadius: hover() ? "5px" : "20px",
   fontSize: hover() ? "24px" : "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
    <button
        onClick={() => toggle()}
        class={style()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        type="submit">{target[0] === '*' ? "شاهد اقل" : "شاهد الكل"}</button>
  )
}

const fetcher = async ({departmentId,name} : {departmentId : string, name : () => string | null}) => {
  return (await invoke("search_problem",{name : name() !== ' ' ? name() : null,departmentId})) as Name[]
}

function ShowAllHistory({target} : {target :() => [string | null]}){
    return (
        <ButtonsOrElement
              returnButtonText="الرجوع الي الاقسام"
              buttonElementPairs={() => (departmentsNames() || [])
                .map(d => [d.name, <ShowHistory
                                      departmentId={d.id}
                                      target={target()}/>])}
              num={[-1]}
              fun={() => console.log("later")}/>
    )
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

  const tableStyle = css({
    width: "95%",
    margin: "5px auto",
  })

  return (
      <section>
        <table class={tableStyle}>
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
