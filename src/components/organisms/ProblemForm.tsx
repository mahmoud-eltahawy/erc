import { invoke } from "@tauri-apps/api"
import { createResource, createSignal} from "solid-js"
import { createStore } from "solid-js/store"
import { Name} from "../../index"
import { SearchBar } from "../molecules/SearchBar"
import { listen } from '@tauri-apps/api/event'

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
  const [displayNote    ,setDisplayNote  ] = createSignal(false)

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
      setDisplayNote(false)
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
          note                 : note() ? note() : null
      }
      await invoke("save_problem_detail",{problemDetail})
      restore()
    }catch(err){
      alert(err)
    }
  }

  const toggleNote   = () => {
      if(displayNote()){
          setDisplayNote(false)
      } else {
          setDisplayNote(true)
      }
  }

  const noteArea = <textarea value={note()}
              onInput={e => setNote(e.currentTarget.value)}
              class={"problemFormText"}
              cols={30} rows={4}
              maxLength={499}
              placeholder="اكتب ما لا يتجاوز 500 حرف"></textarea>
  return (
    <div class={"problemFormContainer"}>
    <form onSubmit={handleSubmit}>
      <div class={"problemFormTimeBlock"}>
        <input value={endTime()}
               onChange={e => setEndTime(e.currentTarget.value)}
               class={"problemFormTimeInput"}
               type="time"
               min={beginTime()}
               max={(shiftBorders() || ["", ""]).at(1)}
               required/>
        <label class="problemFormTimeLabel"><h4>وقت النهاية</h4></label>
      </div>
      <div class={"problemFormTimeBlock"}>
        <input value={beginTime()}
               onChange={e => setBeginTime(e.currentTarget.value)}
               class={"problemFormTimeInput"}
               type="time"
               min={(shiftBorders() || ["", ""]).at(0)}
               max={endTime()}
               required/>
        <label class={"problemFormTimeLabel"}><h4>وقت البداية</h4></label>
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
        <button
            type="button"
            onClick={toggleNote}
            class={"problemFormButton"}
        >اضافة ملحوظة  { note().length }</button>
        {displayNote() ? noteArea : <></>}
        <button type="submit">حفظ</button>
    </form>
  </div>
  )
}
