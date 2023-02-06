import { invoke } from "@tauri-apps/api"
import { BaseSyntheticEvent, useState } from "react"
import { Name, ProblemDeps, ShiftProblemMini } from "./main"
import { SearchBar } from "./SearchBar"
import { shiftProblemFromMinimal } from "./Wall"

export default function ProblemForm({
    toggle,
    add,
    deps,
    writerId,
    shiftId,
    departmentId
} : {
    toggle          : Function,
    add             : Function,
    deps            : ProblemDeps,
    writerId        : string,
    shiftId         : string,
    departmentId    : string
}){
    const { employees, machines,spareParts, problems,shiftBegin,shiftEnd} = deps

    const [displayNote,setDisplayNote] = useState(false)

    const [beginTime       ,setBeginTime        ] = useState(shiftBegin)
    const [endTime         ,setEndTime          ] = useState(shiftEnd)
    const [chosenEmployees ,setChosenEmployees  ] = useState<Name[]>([])
    const [chosenMachines  ,setChosenMachines   ] = useState<Name[]>([])
    const [chosenSpareParts,setChosenSpareParts ] = useState<Name[]>([])
    const [chosenProblems  ,setChosenProblems   ] = useState<Name[]>([])
    const [writtenNote     ,setWrittenNote      ] = useState('')

    const handleSubmit = async (e: BaseSyntheticEvent) => {
      e.preventDefault()
      if (!chosenMachines[0]){
          alert("يجب تحديد الالة التي تمت عليها الصيانة")
          return;
      }
      if (!chosenEmployees[0]){
          alert("يجب تحديد الموظف الذي قام بالصيانة")
          return;
      }
      if (!chosenProblems.length){
          alert("يجب تحديد مشكلة واحدة علي الاقل")
          return;
      }
      toggle()
      try{
        const shift_problem = await invoke("save_problem_detail",{problemDetail : {
            shift_id             : shiftId,
            writer_id            : writerId,
            maintainer_id        : chosenEmployees[0].id,
            machine_id           : chosenMachines[0].id,
            begin_time           : beginTime.length === 8 ? beginTime : beginTime + ":00",
            end_time             : endTime.length   === 8 ? endTime   : endTime   + ":00",
            problems_ids         : chosenProblems.map(problem => problem.id),
            spare_parts_ids      : chosenSpareParts.length ? chosenSpareParts.map(part => part.id) : null,
            note                 : writtenNote ? writtenNote : null
        }, departmentId : departmentId})
        const shiftProblem = await shiftProblemFromMinimal(shift_problem as ShiftProblemMini)
        add(shiftProblem)
      }catch(err){
        alert(err)
      }
    }

    const toggleNote   = () => {
        if(displayNote){
            setDisplayNote(false)
        } else {
            setDisplayNote(true)
        }
    }

    const noteArea = <textarea value={writtenNote}
                onChange={e => setWrittenNote(e.currentTarget.value)}
                className={"problemFormText"}
                cols={30} rows={4}
                maxLength={499}
                placeholder="اكتب ما لا يتجاوز 500 حرف"></textarea>
    return (
    <div className={"problemFormContainer"}>
    <form onSubmit={handleSubmit}>
      <div className={"problemFormTimeBlock"}>
        <input value={endTime}
               onChange={e => setEndTime(e.currentTarget.value)}
               className={"problemFormTimeInput"}
               type="time"
               min={beginTime}
               max={shiftEnd}
               required/>
        <label className="problemFormTimeLabel"><h4>وقت النهاية</h4></label>
      </div>
      <div className={"problemFormTimeBlock"}>
        <input value={beginTime}
               onChange={e => setBeginTime(e.currentTarget.value)}
               className={"problemFormTimeInput"}
               type="time"
               min={shiftBegin}
               max={endTime}
               required/>
        <label className={"problemFormTimeLabel"}><h4>وقت البداية</h4></label>
      </div>
        <SearchBar dispatch={[chosenMachines, setChosenMachines]}
                 isMulti={false}
                 mtMessage="لا يوجد ماكينة بهذا الاسم"
                 defaultPlaceholder="ابحث عن الماكينة التي تمت عليها الصيانة"
                 resultPlaceholder="الماكينة"
                 optionsList={machines}
                 nyMessage={null}/>
        <SearchBar dispatch={[chosenEmployees, setChosenEmployees]}
                 isMulti={false}
                 mtMessage="لا يوجد موظف بهذا الاسم"
                 defaultPlaceholder="ابحث عن الموظف الذي قام بالصيانة"
                 resultPlaceholder="الموظف"
                 optionsList={employees}
                 nyMessage={null}/>
        <SearchBar dispatch={[chosenProblems, setChosenProblems]}
                 isMulti={true}
                 mtMessage="لا يوجد مشكلة بهذا الاسم"
                 defaultPlaceholder="ابحث عن مشكلة او مشاكل"
                 resultPlaceholder="عدد المشاكل"
                 optionsList={problems}
                 nyMessage={"لم يتم اختيار اي مشكلة حتي الان <اجباري> ا"}/>
        <SearchBar dispatch={[chosenSpareParts, setChosenSpareParts]}
                 isMulti={true}
                 mtMessage="لا توجد قطعة غيار بهذا الاسم"
                 defaultPlaceholder="ابحث عن قطع الغيار المستخدمة في الصيانة"
                 resultPlaceholder="عدد قطع الغيار المستخدمة"
                 optionsList={spareParts}
                 nyMessage={"لم يتم تسجيل اي قطع غيار <اختياري> ا"}/>
        <button type="button" onClick={toggleNote} className={"problemFormButton"}>اضافة ملحوظة  { writtenNote.length }</button>
        {displayNote ? noteArea : <></>}
        <button type="submit">حفظ</button>
    </form>
  </div>
  )
}
