import { useEffect, useState } from "react";
import { ToggleButton } from "../atoms/toggleButton";

export function ButtonsOrElement({
  returnButtonText  ,
  buttonElementPairs,
  num,
  fun
  } : {
  returnButtonText   : string,
  buttonElementPairs : [string,JSX.Element][],
  num : number,
  fun : Function}){
  const [emptyPlayGround, setEmptyPlayGround] = useState(true)
  const [toggleButtons, setToggleButtons] = useState(Array(buttonElementPairs.length).fill(false))

  const toggle = (id : number) => {
    setToggleButtons(buttons => buttons.map((cond,index) => {
        if (index !== id) {
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

  useEffect(() => {
    if(num !== -1){
        toggle(num)
        fun()
    }
  },[num])

  return (
    <>
      {buttonElementPairs.map((idc, index) => <Compositor key={index}
          e1={<ToggleButton
                      pGround={emptyPlayGround}
                      tButton={toggleButtons[index]}
                      defaultCont={returnButtonText}
                      cont={idc[0]}
              toggle={() => toggle(index)}/>}
           e2={toggleButtons[index] ? idc[1] : null}
          />
        )}
    </>
  )
}

function Compositor({e1,e2} : {e1 : JSX.Element,e2 : JSX.Element | null}){
    return (
        <>{e1}{e2}</>
    )
}
