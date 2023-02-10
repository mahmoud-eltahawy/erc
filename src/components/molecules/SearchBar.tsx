import { useMemo, useState } from "react"
import { Name } from "../../main"

export function SearchBar({
    defaultPlaceholder,
    resultPlaceholder,
    mtMessage,
    nyMessage = null,
    isMulti,
    optionsList,
    dispatch
} : {
    defaultPlaceholder   : string,
    resultPlaceholder    : string,
    mtMessage            : string,
    nyMessage            : string | null,
    isMulti              : boolean,
    optionsList          : Name[],
    dispatch : [Name[],React.Dispatch<React.SetStateAction<Name[]>>]}){

  const wildChar = ' '
  const [target, setTarget]                             = useState('')
  const [list,setList]                                  = useState(optionsList)
  const [chosens,setChosens]                            = dispatch
  const filtered = useMemo<Name[]>(() => {
      return list.filter(membr => membr.name.includes(target) || target === wildChar)
    }, [target,list])

  const showSelectView = target.length > 0 || target === wildChar

  const getChosenOne = () => {
    if(chosens[0]){
        return resultPlaceholder + " : " + chosens[0].name
    } else {
        return defaultPlaceholder
    }
  }

  const disabledOption = (message : string) => {
    return <option disabled>{message}</option>
  }

  const headInput = <input
    placeholder={isMulti ? `${resultPlaceholder} : ${chosens.length}` :  getChosenOne()}
    className={"insertField"}
    type="text"
    value={target}
    onChange={e => setTarget(e.currentTarget.value)} />


  const choiceOptionHandler = (member : Name) => {
                setChosens(chosens => {
                  if(isMulti){
                    if (!chosens.includes(member)){
                      chosens.unshift(member)
                    }
                    return chosens
                  }
                  return [member]
                })
                if(isMulti){
                  setList(list => list.filter(m => m !== member))
                } else {
                  setTarget('')
                }
              }
  const resultOptionHandler = (chosen : Name) => {
                  setChosens(chosens => chosens.filter(c => c.id !== chosen.id))
                  setList(list => [chosen,...list])
                }
  const choiceSelect = <select
          multiple className="searchBarViewMember">
          {
            filtered
              .map(member => <option
                    onClick={() => choiceOptionHandler(member)}
                    key={member.id} >{member.name}
              </option>)
          }
          {!filtered.length? disabledOption(mtMessage): <></>}
        </select>

  const resultSelect = <select multiple className="searchBarViewMember">
          {
            chosens.map(chosen =>
                <option key={chosen.id}
                    onClick={() => resultOptionHandler(chosen)}
                >{chosen.name}</option>)
          }
        {!chosens.length? disabledOption(nyMessage!) : <></>}
        </select>

  const searchView = <section className="searchBarView">
      {isMulti  ? resultSelect : <></>} {choiceSelect} </section>

  return (
    <div className={"searchBarContainer"} >
      {headInput}
      {showSelectView ? searchView : <></>}
    </div>
  )
}
