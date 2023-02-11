import { ButtonsOrElement } from "../molecules/buttonsOrElement"

export default function HistoryShow(){

  const buttonsOrElement = <ButtonsOrElement returnButtonText="العودة لصفحة البحث"
                                  buttonElementPairs={[
                                    ["ابحث عن يوم"    ,<h1>يوم</h1>],
                                    ["ابحث عن قطعة غيار",<h1>قطعة غيار</h1>],
                                    ["ابحث عن مشكلة"  ,<h1>مشكلة</h1>],
                                    ["ابحث عن ماكينة"   ,<h1>ماكينة</h1>]
                                  ]}
                                  num={-1} fun={() => console.log("later")}
  />
  return (
      <section className="LoginFormContainer">
      {buttonsOrElement}
      </section>
  )
}
