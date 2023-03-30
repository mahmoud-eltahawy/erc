import { css } from "solid-styled-components"
import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import ControllAdmins from "../molecules/controllAdmins"
import ControllDepartments from "../molecules/controllDepartments"
import ControllEmployees from "../molecules/controllEmployee"

export default function Controlling(){

  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
      <section class={container}>
          {<ButtonsOrElement
               returnButtonText="العودة لصفحة التحكم"
               buttonElementPairs={() => [
                   ["تمكين موظف", () => <ControllAdmins/>],
                   ["اعدادات الاقسام", () => <ControllDepartments/>],
                   ["صلاحيات الموظفين", () => <ControllEmployees/>],
                 ]}
               num={[-1]}
               fun={() => console.log("later")}/>}
      </section>
  )
}
