import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, createSignal, For, Setter, Show} from "solid-js"
import { createStore, SetStoreFunction } from "solid-js/store"
import { Name, Note, ShiftProblem } from "../../index"
import { listen } from '@tauri-apps/api/event'
import { css } from "solid-styled-components"
import SubmitButton from "../atoms/submitButton"
import { employee, permissions, shiftId } from "../../App"
import {ToggleButton} from "../atoms/toggleButton"

export function ProblemUpdateForm({
  toggle,
  beginValues
} : {
  toggle          : Function,
  beginValues : [ShiftProblem,Name[],Name[] | undefined,Note | undefined]
}){
  const [{id,machine,maintainer,begin_time,end_time},problems,spareParts,note] = beginValues

  const [beginTimeI, setBeginTime] = createSignal(begin_time)
  const [endTimeI, setEndTime] = createSignal(end_time)
  const [employeesI      ,setEmployees    ] = createStore<Name[]>([maintainer])
  const [machinesI       ,setMachines     ] = createStore<Name[]>([machine])
  const [problemsI       ,setProblems     ] = createStore<Name[]>([...problems])
  const [sparePartsI     ,setSpareParts   ] = createStore<Name[]>([...(spareParts || [])])
  const [noteI           ,setNote         ] = createSignal(note?.content)

  listen_selections_updates()

  const handleSubmit = async (e : Event) => {
    e.preventDefault()
    if (!machinesI.at(0)){
        alert("يجب تحديد الالة التي تمت عليها الصيانة")
        return;
    }
    if (!employeesI.at(0)){
        alert("يجب تحديد الموظف الذي قام بالصيانة")
        return;
    }
    if (!problemsI.length){
        alert("يجب تحديد مشكلة واحدة علي الاقل")
        return;
    }
    toggle()
    try{
      await invoke("update_problem_detail", {
          shiftProblemId : id,
          core : [
              [maintainer.id,employeesI[0].id],
              [machine.id,machinesI[0].id],
              [begin_time,beginTimeI().length === 8 ? beginTimeI() : beginTimeI() + ":00"],
              [end_time,endTimeI().length   === 8 ? endTimeI()   : endTimeI()   + ":00"],
          ],
          problems   : [problems.map(p => p.id)          , problemsI.map(problem => problem.id)],
          spareParts : [spareParts?.map(s => s.id) || [] , sparePartsI?.map(part => part.id) || []],
          note       : [note?.content                   , noteI()?.trim()],
      })
    }catch(err){
      alert(err)
    }
  }

  return (
    <div class={container}>
      <Show
          when={permissions()?.write_department_problem && shiftBorders()}
          fallback={<h1>ليس لديك صلاحية تسجيل عطل</h1>}>
        <form onSubmit={handleSubmit}>
          <TimeConstraint
            beginTime={() => beginTimeI()}
            endTime={() => endTimeI()}
            setBeginTime={setBeginTime}
            setEndTime={setEndTime}/>
          <SearchBars
              employees={employeesI}
              setEmployees={setEmployees}
              problems={problemsI}
              setProblems={setProblems}
              spareParts={sparePartsI}
              setSpareParts={setSpareParts}
              machines={machinesI}
              setMachines={setMachines}/>
          <ExtraNote note={() => noteI() || ""} setNote={setNote} />
          <SubmitButton length={undefined} />
          <ToggleButton
              toggle={toggle}
              cont=""
              defaultCont="الغاء"
              tButton={() => true}/>
        </form>
      </Show>
    </div>
  )
}

export function ProblemSaveForm({
    toggle,
} : {
    toggle          : Function,
}){
  const [beginTime      ,setBeginTime    ] = createSignal("")
  const [endTime        ,setEndTime      ] = createSignal("")
  const [employees      ,setEmployees    ] = createStore<Name[]>([])
  const [machines       ,setMachines     ] = createStore<Name[]>([])
  const [spareParts     ,setSpareParts   ] = createStore<Name[]>([])
  const [problems       ,setProblems     ] = createStore<Name[]>([])
  const [note           ,setNote         ] = createSignal("")

  listen_selections_updates()

  const restore = () => {
      setBeginTime("")
      setEndTime("")
      setEmployees([])
      setMachines([])
      setSpareParts([])
      setProblems([])
      setNote("")
  }

  const handleSubmit = async (e : Event) => {
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
          shift_id             : shiftId(),
          writer_id            : employee()?.id,
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

  return (
    <div class={container}>
      <Show
          when={permissions()?.write_department_problem && shiftBorders()}
          fallback={<h1>ليس لديك صلاحية تسجيل عطل</h1>}>
        <form onSubmit={handleSubmit}>
          <TimeConstraint
            beginTime={() => beginTime()}
            endTime={() => endTime()}
            setBeginTime={setBeginTime}
            setEndTime={setEndTime}/>
          <SearchBars
              employees={employees}
              setEmployees={setEmployees}
              problems={problems}
              setProblems={setProblems}
              spareParts={spareParts}
              setSpareParts={setSpareParts}
              machines={machines}
              setMachines={setMachines}/>
          <ExtraNote note={() => note()} setNote={setNote} />
          <SubmitButton length={undefined} />
        </form>
      </Show>
    </div>
  )
}

const borders_fetcher = async () => {
  return (await invoke("current_shift_borders")) as [string,string]
}
const [shiftBorders,{refetch}] = createResource(borders_fetcher)

listen("shift_ended",() => refetch())

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
                name : string | null,canceled : () => string[]) => {
    let the_name = null;
    if ( name ) {
        if ( name !== ' ' ){
          the_name = name
        }
    }
    return (await invoke(selection,{departmentId : employee()!.department_id,name : the_name,canceled :canceled()})) as Name[]
}

const listen_selections_updates = () => {
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
}

export type Updates = ["Problem"] | ["SparePart"] | ["Machine"] | ["Employee"] | ["None"]

const [updates , setUpdates] = createStore<Updates>(["None"])

const container = css({
  display: "block",
  fontSize: "x-large",
  borderTop: "solid 2px",
  borderBottom: "solid 9px",
  margin: "1% auto",
  padding: "1%",
})

function TimeConstraint({
    endTime,
    setEndTime,
    beginTime,
    setBeginTime,
  } : {
    endTime : () => string,
    setEndTime : Function,
    beginTime : () => string,
    setBeginTime : Function,
}){
  const timeLabel = css({
    display: "inline-block",
    width: "35%",
    padding: ".1em",
    margin: ".1em auto",
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
  return (
    <section>
      <div class={timeContainer}>
          <input value={endTime()}
              onChange={e => setEndTime(e.currentTarget.value)}
              class={timeInput}
              type="time"
              min={beginTime()}
              max={shiftBorders()!.at(1)!.slice(0,5)}
              required/>
          <label class={timeLabel}><h4>وقت النهاية</h4></label>
      </div>
      <div class={timeContainer}>
          <input value={beginTime()}
              onChange={e => setBeginTime(e.currentTarget.value)}
              class={timeInput}
              type="time"
              min={shiftBorders()!.at(0)!.slice(0,5)}
              max={endTime()}
              required/>
          <label class={timeLabel}><h4>وقت البداية</h4></label>
      </div>
    </section>
  )
}

function ExtraNote({note,setNote} : {note : () => string | null,setNote : Setter<string>}){
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
          length={() => note()?.length || 0}
          toggleNote={toggleNote}/>
      <Show when={displayNote()}>
        <NoteText
            note={() => note() || ""}
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

function SearchBars({
    machines,
    setMachines,
    employees,
    setEmployees,
    problems,
    setProblems,
    spareParts,
    setSpareParts,
  } : {
    machines : Name[],
    setMachines : SetStoreFunction<Name[]>,
    employees : Name[],
    setEmployees : SetStoreFunction<Name[]>,
    problems : Name[],
    setProblems : SetStoreFunction<Name[]>,
    spareParts : Name[],
    setSpareParts : SetStoreFunction<Name[]>,
}){
    return (
      <section>
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
                                      name(),
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
      </section>
    )
}

function SearchBar({
    defaultPlaceholder,
    resultPlaceholder,
    mtMessage,
    nyMessage = null,
    isMulti,
    chosens,
    setChosens,
    selection_fetcher,
    subject,
    updates
} : {
    subject              : string,
    updates              : [string]
    defaultPlaceholder   : string,
    resultPlaceholder    : string,
    mtMessage            : string,
    nyMessage            : string | null,
    isMulti              : boolean,
    selection_fetcher    : (name : () => string | null) => Promise<Name[]>,
    chosens              : Name[],
    setChosens           : SetStoreFunction<Name[]>,
}){

  const [target, setTarget]     = createSignal<string | null>(null)
  const [optionsList,{refetch}] = createResource(() => target,selection_fetcher)

  createEffect(() => {
      if(updates[0] === subject || target()){
          refetch()
      }
  })

  const getChosenOne = () => {
    if (chosens.at(0)){
        return resultPlaceholder + " : " + chosens.at(0)!.name
    } else {
        return defaultPlaceholder
    }
  }

  const choiceOptionHandler = (member : Name) => {
    setChosens(prev => {
      if(isMulti){
        if (!prev.includes(member)){
          return [member,...prev]
        }
        return prev
      }
      return [member]
    })
    if(!isMulti){
      setTarget('')
    }
    refetch()
  }

  const resultOptionHandler = (chosen : Name) => {
      setChosens(prev => prev.filter(c => c.id !== chosen.id))
      refetch()
  }

  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  })

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
  })

  const viewMember = css({
    display: "inline-block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
    borderLeft: "solid 5px",
    borderRight: "solid 5px",
    borderBottom: "solid 5px",
    borderTop: "none",
    borderBottomLeftRadius : "20px",
    borderBottomRightRadius : "20px",
  })

  const inputStyle = css({
    display: "block",
    backgroundColor: "transparent",
    fontSize: "24px",
    width: "70%",
    padding: ".1em",
    margin: ".1em auto",
  })

  return (
    <div class={container} >
        <input
          placeholder={isMulti ? `${resultPlaceholder} : ${chosens.length}` :  getChosenOne()}
          class={inputStyle}
          type="text"
          value={target()!}
          onInput={e => {
            setTarget(e.currentTarget.value)
            refetch()
          }} />
        <Show when={(target() || '').length > 0}>
          <section class={viewContainer}>
            <Show when={isMulti}>
              <select multiple size={chosens.length} class={viewMember}>
                {
                    <For each={chosens}>
                        {
                            (item) => (
                              <option onClick={() => resultOptionHandler(item)}>{item.name}</option>
                            )
                        }
                    </For>
                }
                <Show when={!chosens.length}><option disabled>{nyMessage}</option></Show>
              </select>
            </Show>
            <select multiple class={viewMember}>
              {
                  <For each={optionsList()}>
                      {
                          (item) => (
                            <option onClick={() => choiceOptionHandler(item)}>{item.name}</option>
                          )
                      }
                  </For>
              }
              <Show when={!(optionsList() || []).length}><option disabled>{mtMessage}</option></Show>
            </select>
          </section>
      </Show>
    </div>
  )
}
