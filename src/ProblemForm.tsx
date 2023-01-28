import { invoke } from "@tauri-apps/api"
import { BaseSyntheticEvent, useEffect, useState } from "react"
import { Name, ProblemDeps } from "./main"
import { SearchBar } from "./SearchBar"

export default function ProblemForm({deps} : {deps : ProblemDeps}){
    const { employees, machines,spareParts, problems,shiftBegin,shiftEnd} = deps
    const [writtenNote,setWrittenNote] = useState('')
    const [displayNote,setDisplayNote] = useState(false)
    const [beginTime,setBeginTime]     = useState('')
    const [endTime,setEndTime]         = useState('')

    const handleSubmit = (e: BaseSyntheticEvent) => { e.preventDefault() }

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
               min={shiftBegin}
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
               max={shiftEnd}
               required/>
        <label className={"problemFormTimeLabel"}><h4>وقت البداية</h4></label>
      </div>
      <SearchBar isMulti={false}
                 mtMessage="لا يوجد ماكينة بهذا الاسم"
                 defaultPlaceholder="ابحث عن الماكينة التي تمت عليها الصيانة"
                 resultPlaceholder="الماكينة"
                 optionsList={machines}
                 nyMessage={null}/>
      <SearchBar isMulti={false}
                 mtMessage="لا يوجد موظف بهذا الاسم"
                 defaultPlaceholder="ابحث عن الموظف الذي قام بالصيانة"
                 resultPlaceholder="الموظف"
                 optionsList={employees}
                 nyMessage={null}/>
      <SearchBar isMulti={true}
                 mtMessage="لا يوجد مشكلة بهذا الاسم"
                 defaultPlaceholder="ابحث عن مشكلة او مشاكل"
                 resultPlaceholder="عدد المشاكل"
                 optionsList={problems}
                 nyMessage={"لم يتم اختيار اي مشكلة حتي الان <اجباري> ا"}/>
      <SearchBar isMulti={true}
                 mtMessage="لا توجد قطعة غيار بهذا الاسم"
                 defaultPlaceholder="ابحث عن قطع الغيار المستخدمة في الصيانة"
                 resultPlaceholder="عدد قطع الغيار المستخدمة"
                 optionsList={spareParts}
                 nyMessage={"لم يتم تسجيل اي قطع غيار <اختياري> ا"}/>
      <button onClick={toggleNote} className={"problemFormButton"}>اضافة ملحوظة  { writtenNote.length }</button>
      {displayNote ? noteArea : <></>}
      <button type="submit">حفظ</button>
    </form>
  </div>
  )
}
