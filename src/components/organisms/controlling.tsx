import { css } from "solid-styled-components"
import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import ControllAdmins from "../molecules/controllAdmins"
import ControllDepartments from "../molecules/controllDepartments"
import ControllEmployees from "../molecules/controllEmployee"
import { Show } from "solid-js"

export default function Controlling({isAllowed} : {isAllowed : boolean}){

  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
    <section class={container}>
      <Show
	when={isAllowed}
        fallback={<div>
          <p>ليس لديك الصلاحيات لفتح صفحة التحكم</p>
	  <p>يجب ان تكون مشرف او يكون لديك الحساب الرئيسي</p>
        </div>}>
          <ButtonsOrElement
               returnButtonText="العودة لصفحة التحكم"
               buttonElementPairs={() => [
                   ["تعيين مشرف", () => <ControllAdmins/>],
                   ["اعدادات الاقسام", () => <ControllDepartments/>],
                   ["صلاحيات الموظفين", () => <ControllEmployees/>],
                 ]}
               num={[-1]}
               fun={() => console.log("later")}/>
      </Show>
      </section>
  )
}
