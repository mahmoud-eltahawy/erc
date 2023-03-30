import { createEffect,createResource,Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { employees_names_fetcher } from '../..'
import ShowAllToggleButton from "../atoms/showAllToggleButton"
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
               placeholder="ادخل اسم الموظف"
               required/>
      </div>
      <ShowAllToggleButton target={target} toggle={toggle}/>
      <ShowHistory target={target}/>
    </section>
  )
}

function ShowHistory({target} :{target : [string | null]}){
  const [employees,{refetch}] = createResource({name :() => target[0]},employees_names_fetcher)

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
