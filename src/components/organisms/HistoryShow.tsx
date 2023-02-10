import { useState } from "react"
import { ToggleButtons } from "../molecules/toggleButtons"

const bId = {
  day       : '0',
  sparePart : '1',
  problem   : '2',
  machine   : '3'
}

export default function HistoryShow(){
  const [emptyPlayGround, setEmptyPlayGround] = useState(true)
  const [toggleButtons, setToggleButtons] = useState<Array<boolean>>(Array(4).fill(false))

  const toggle = (id : string) => {
    setToggleButtons(buttons => buttons.map((cond,condId) => {
        if (condId !== +id) {
          return false
        }
        if(cond){
          setEmptyPlayGround(true)
          return false
        }
        setEmptyPlayGround(false)
        return true
      })
    )
  }

  const theButtons = <ToggleButtons defaultContent="العودة لصفحة البحث"
                                    idToggle={toggle}
                                    isEmptyGround={emptyPlayGround}
                                    tbuttons={toggleButtons}
                                    idContent={[
                                      {id : bId.day      , content : "ابحث عن يوم"},
                                      {id : bId.sparePart, content : "ابحث عن قطعة غيار"},
                                      {id : bId.problem  , content : "ابحث عن مشكلة"},
                                      {id : bId.machine  , content : "ابحث عن ماكينة"},
                                    ]}
  />
  return (
      <section className="LoginFormContainer">
      {theButtons}
      </section>
  )
}
