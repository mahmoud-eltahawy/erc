import { useEffect, useState } from "react"

export function ToggleButton({
    toggle,
    cont,
    defaultCont,
    pGround,
    tButton
    } : {
    toggle          : Function,
    cont            : string,
    defaultCont     : string,
    pGround         : boolean,
    tButton         : boolean
}){
    const [content, setContent] = useState(cont)
    const [display,setDisplay] = useState(pGround || tButton)

    useEffect(()=>{
      setDisplay(pGround || tButton)
      setContent(tButton ? defaultCont : cont )
    },[pGround,tButton])

    return (
    <>
      {display ? <button onClick={() => toggle()}>{content}</button>: null}
    </>
    )
}
