import { ToggleButton } from "../atoms/toggleButton"

export function ToggleButtons({
  defaultContent ,
  isEmptyGround  ,
  idToggle       ,
  idContent      ,
  tbuttons
  } : {
  defaultContent : string,
  isEmptyGround  : boolean,
  idToggle       : Function,
  idContent      : {id : string,content : string}[],
  tbuttons       : boolean[]
}) {
  return (
    <div>{idContent.map(idc => <ToggleButton pGround={isEmptyGround}
                                      tButton={tbuttons[+idc.id]}
                                      defaultCont={defaultContent}
                                      cont={idc.content}
                                      toggle={() => idToggle(idc.id)}/>)}
    </div>
  )
}
