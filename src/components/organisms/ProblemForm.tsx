import { invoke } from "@tauri-apps/api"
import { createResource, createSignal, Setter, Show} from "solid-js"
import { createStore } from "solid-js/store"
import { Name, permissions} from "../../index"
import { SearchBar } from "../molecules/SearchBar"
import { listen } from '@tauri-apps/api/event'
import { css } from "solid-styled-components"

const borders_fetcher = async () => {
    return (await invoke("current_shift_borders")) as [string,string]
}
const fetcher = async (selection : string,the_name : string | null,canceled :() => string[]) => {
    let name = null;
    if ( the_name ) {
        if ( the_name !== ' ' ){
          name = the_name
        }
    }
    return (await invoke(selection,{name,canceled :canceled()})) as Name[]
}
const department_fetcher = async (selection : string,
                departmentId : string,name : string | null,canceled : () => string[]) => {
    let the_name = null;
    if ( name ) {
        if ( name !== ' ' ){
          the_name = name
        }
    }
    console.log("the name is"+the_name)
    return (await invoke(selection,{departmentId,name : the_name,canceled :canceled()})) as Name[]
}

export type Updates = ["Problem"] | ["SparePart"] | ["Machine"] | ["Employee"] | ["None"]

export default function ProblemForm({
    toggle,
    writerId,
    shiftId,
    departmentId,
} : {
    toggle          : Function,
    writerId        : string,
    shiftId         : string,
    departmentId    : string,
}){
  const [shiftBorders] = createResource(borders_fetcher)

  const [beginTime      ,setBeginTime    ] = createSignal("")
  const [endTime        ,setEndTime      ] = createSignal("")
  const [employees      ,setEmployees    ] = createStore<Name[]>([])
  const [machines       ,setMachines     ] = createStore<Name[]>([])
  const [spareParts     ,setSpareParts   ] = createStore<Name[]>([])
  const [problems       ,setProblems     ] = createStore<Name[]>([])
  const [note           ,setNote         ] = createSignal("")

  const [updates , setUpdates] = createStore<Updates>(["None"])

  listen("update_problem",() => {
      setUpdates(["Problem"])
  })

  listen("update_employee",() => {
      setUpdates(["Employee"])
  })

  listen("update_machine",() => {
      setUpdates(["Machine"])
  })

  listen("update_spare_part",() => {
      setUpdates(["SparePart"])
  })

  const restore = () => {
      setBeginTime("")
      setEndTime("")
      setEmployees([])
      setMachines([])
      setSpareParts([])
      setProblems([])
      setNote("")
  }

  const handleSubmit = async (e : any) => {
    e.preventDefault()
    if (!machines.at(0)){
        alert("يجب تحديد الالة التي تمت عليها الصيانة")
        return;
    }
    if (!employees.at(0)){
        alert("يجب تحديد الموظف الذي قام بالصيانة")
        return;
    }
    if (!problems.length){
        alert("يجب تحديد مشكلة واحدة علي الاقل")
        return;
    }
    toggle()
    try{
      const problemDetail = {
          shift_id             : shiftId,
          writer_id            : writerId,
          maintainer_id        : employees.at(0)!.id,
          machine_id           : machines.at(0)!.id,
          begin_time           : beginTime().length === 8 ? beginTime() : beginTime() + ":00",
          end_time             : endTime().length   === 8 ? endTime()   : endTime()   + ":00",
          problems_ids         : problems.map(problem => problem.id),
          spare_parts_ids      : spareParts.length ? spareParts.map(part => part.id) : null,
          note                 : note() ? note().trim() : null
      }
      await invoke("save_problem_detail",{problemDetail})
      restore()
    }catch(err){
      alert(err)
    }
  }

  const container = css({
    display: "block",
    fontSize: "x-large",
    borderTop: "solid 2px",
    borderBottom: "solid 9px",
    margin: "1% auto",
    padding: "1%",
  })

  const timeContainer = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  })

  const timeInput = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor:"lightyellow",
    borderRadius: "20px",
  })

  const timeLabel = css({
  display: "inline-block",
  width: "35%",
  padding: ".1em",
  margin: ".1em auto",
  })

  return (
    <div class={container}>
      <Show
          when={permissions()?.write_department_problem}
          fallback={<h1>ليس لديك صلاحية تسجيل عطل</h1>}>
        <form onSubmit={handleSubmit}>
        <div class={timeContainer}>
            <input value={endTime()}
                onChange={e => setEndTime(e.currentTarget.value)}
                class={timeInput}
                type="time"
                min={beginTime()}
                max={(shiftBorders() || ["", ""]).at(1)}
                required/>
            <label class={timeLabel}><h4>وقت النهاية</h4></label>
        </div>
        <div class={timeContainer}>
            <input value={beginTime()}
                onChange={e => setBeginTime(e.currentTarget.value)}
                class={timeInput}
                type="time"
                min={(shiftBorders() || ["", ""]).at(0)}
                max={endTime()}
                required/>
            <label class={timeLabel}><h4>وقت البداية</h4></label>
        </div>
        <SearchBar
                 subject="Machine"
                 updates={updates}
                 chosens={machines}
                 setChosens={setMachines}
                 isMulti={false}
                 mtMessage="لا يوجد ماكينة بهذا الاسم"
                 defaultPlaceholder="ابحث عن الماكينة التي تمت عليها الصيانة"
                 resultPlaceholder="الماكينة"
                 selection_fetcher={(name : () => string | null) =>
                     fetcher("machines_selection",
                                 name(),
                                 () => machines.map(m => m.name))}
                 nyMessage={null}/>
        <SearchBar
                 subject="Employee"
                 updates={updates}
                 chosens={employees}
                 setChosens={setEmployees}
                 isMulti={false}
                 mtMessage="لا يوجد موظف بهذا الاسم"
                 defaultPlaceholder="ابحث عن الموظف الذي قام بالصيانة"
                 resultPlaceholder="الموظف"
                 selection_fetcher={(name : () => string | null) =>
                     fetcher("employees_selection",
                                 name(),
                                 () => employees.map(e => e.id))
                 }
                 nyMessage={null}/>
        <SearchBar
                 subject="Problem"
                 updates={updates}
                 chosens={problems}
                 setChosens={setProblems}
                 isMulti={true}
                 mtMessage="لا يوجد مشكلة بهذا الاسم"
                 defaultPlaceholder="ابحث عن مشكلة او مشاكل"
                 resultPlaceholder="عدد المشاكل"
                 selection_fetcher={(name : () => string | null) =>
                     department_fetcher("problems_selection",
                                        departmentId, name(),
                                        () => problems.map(p => p.name))}
                 nyMessage={"لم يتم اختيار اي مشكلة حتي الان <اجباري> ا"}/>
        <SearchBar
                 subject="SparePart"
                 updates={updates}
                 chosens={spareParts}
                 setChosens={setSpareParts}
                 isMulti={true}
                 mtMessage="لا توجد قطعة غيار بهذا الاسم"
                 defaultPlaceholder="ابحث عن قطع الغيار المستخدمة في الصيانة"
                 resultPlaceholder="عدد قطع الغيار المستخدمة"
                 selection_fetcher={(name : () => string | null) =>
                     fetcher("spare_parts_selection",
                                 name(),
                                () => spareParts.map(s => s.name))}
                 nyMessage={"لم يتم تسجيل اي قطع غيار <اختياري> ا"}/>
        <ExtraNote note={() => note()} setNote={setNote} />
        <SubmitButton/>
      </form>
    </Show>
  </div>
  )
}

function ExtraNote({note,setNote} : {note : () => string,setNote : Setter<string>}){
  const [displayNote    ,setDisplayNote  ] = createSignal(false)

  const toggleNote   = () => {
      if(displayNote()){
          setDisplayNote(false)
      } else {
          setDisplayNote(true)
      }
  }

  return (
    <section>
      <NoteButton
          length={() => note().length}
          toggleNote={toggleNote}/>
      <Show when={displayNote()}>
        <NoteText
            note={() => note()}
            setNote={setNote}/>
      </Show>
    </section>
  )
}

function NoteText({note,setNote} : {note : () => string,setNote : Setter<string>}){
  const style = css({
    fontSize: "x-large",
    width: "90%",
    backgroundColor: "blanchedalmond",
  })

  return (
      <textarea value={note()}
        onInput={e => setNote(e.currentTarget.value)}
        class={style}
        cols={30} rows={4}
        maxLength={499}
        placeholder="اكتب ما لا يتجاوز 500 حرف"></textarea>
  )
}

function NoteButton({length,toggleNote} : {length : () => number,toggleNote : Function}){
  const [hover,setHover] = createSignal(false)

  const style = () => css({
   display: "block",
   width: "15%",
   borderRadius: hover() ? "5px" : "20px",
   fontSize: hover() ? "22px" : "16px",
   border: "solid 1px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
      <button
          type="button"
          onClick={() => toggleNote()}
          class={style()}
          onMouseOver={() => setHover(true)}
          onMouseLeave={() => setHover(false)}
      >اضافة ملحوظة  { length() }</button>
  )
}

function SubmitButton(){
  const [hover,setHover] = createSignal(false)

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
        class={style()}
        onMouseOver={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        type="submit">حفظ</button>
  )
}
