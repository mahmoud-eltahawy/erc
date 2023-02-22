import { invoke } from "@tauri-apps/api"
import { createResource, createSignal} from "solid-js"
import { createStore, SetStoreFunction} from "solid-js/store"
import { Name} from "../../index"
import { SearchBar } from "../molecules/SearchBar"

const borders_fetcher = async () => {
    return (await invoke("current_shift_borders")) as [string,string]
}
const fetcher = async (selection : string) => {
    return (await invoke(selection)) as Name[]
}
const department_fetcher = async (selection : string,departmentId : string) => {
    return (await invoke(selection,{departmentId})) as Name[]
}
export default function ProblemForm({
    toggle,
    writerId,
    shiftId,
    departmentId,
    problemsNumber,
    setShiftProblems
} : {
    toggle          : Function,
    writerId        : string,
    shiftId         : string,
    departmentId    : string,
    problemsNumber  : number[],
    setShiftProblems: SetStoreFunction<number[]>
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
      setShiftProblems(sp => [sp[0] + 1])
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
                 chosens={machines}
                 setChosens={setMachines}
                 elementsNumber={[0]}
                 isMulti={false}
                 mtMessage="لا يوجد ماكينة بهذا الاسم"
                 defaultPlaceholder="ابحث عن الماكينة التي تمت عليها الصيانة"
                 resultPlaceholder="الماكينة"
                 selection_fetcher={() => fetcher("machines_selection")}
                 nyMessage={null}/>
        <SearchBar
                 chosens={employees}
                 setChosens={setEmployees}
                 elementsNumber={[0]}
                 isMulti={false}
                 mtMessage="لا يوجد موظف بهذا الاسم"
                 defaultPlaceholder="ابحث عن الموظف الذي قام بالصيانة"
                 resultPlaceholder="الموظف"
                 selection_fetcher={() => fetcher("employees_selection")}
                 nyMessage={null}/>
        <SearchBar
                 chosens={problems}
                 setChosens={setProblems}
                 elementsNumber={problemsNumber}
                 isMulti={true}
                 mtMessage="لا يوجد مشكلة بهذا الاسم"
                 defaultPlaceholder="ابحث عن مشكلة او مشاكل"
                 resultPlaceholder="عدد المشاكل"
                 selection_fetcher={() => department_fetcher("problems_selection",departmentId)}
                 nyMessage={"لم يتم اختيار اي مشكلة حتي الان <اجباري> ا"}/>
        <SearchBar
                 chosens={spareParts}
                 setChosens={setSpareParts}
                 elementsNumber={[0]}
                 isMulti={true}
                 mtMessage="لا توجد قطعة غيار بهذا الاسم"
                 defaultPlaceholder="ابحث عن قطع الغيار المستخدمة في الصيانة"
                 resultPlaceholder="عدد قطع الغيار المستخدمة"
                 selection_fetcher={() => fetcher("spare_parts_selection")}
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
