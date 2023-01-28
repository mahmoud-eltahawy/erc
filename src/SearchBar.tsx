import { useMemo, useState } from "react"
import { Name } from "./main"

export function SearchBar(props : {defaultPlaceholder   : string,
                                   resultPlaceholder    : string,
                                   mtMessage            : string,
                                   nyMessage            : string,
                                   isMulti              : boolean,
                                   optionsList          : Name[]}){
  const {defaultPlaceholder,resultPlaceholder,mtMessage,nyMessage,isMulti,optionsList} = props

  const theInterval = 450
  const [selectViewDisplay,setSelectViewDisplay]        = useState(false)
  const [isSearching,setIsSearching]                    = useState(false)
  const [target, setTarget]                             = useState('')
  const [list,setList]                                  = useState(optionsList)
  const [chosens,setChosens]                            = useState<Name[]>([])
  const filtered = useMemo<Name[]>(() => {
      return list.filter(membr => membr.name.includes(target))
    }, [target,list])

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
    onMouseOver={() => setIsSearching(true)}
    onInput={() => setIsSearching(true)}
    onFocus={() => setIsSearching(true)}
    onMouseLeave={() => {
        if(!selectViewDisplay){
          setInterval(() => {
            setIsSearching(false)
          },theInterval)
        }
      }
    }
    placeholder={isMulti ? `${resultPlaceholder} : ${chosens.length}` :  getChosenOne()}
    className={"insertField"}
    type="text"
    value={target}
    onChange={e => setTarget(e.currentTarget.value)} />

    const choiceSelect = <select
          multiple className="searchBarViewMember">
          {
            filtered
              .map(member => <option onClick={() => {
                setChosens(chosens => {
                    setTarget('')
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
                  }
                }} key={member.id} >{member.name}</option>)
          }
          {!filtered.length? disabledOption(mtMessage): <></>}
        </select>

  const resultSelect = <select multiple className="searchBarViewMember">
          {
            chosens
              .map(chosen =>
                <option key={chosen.id} onClick={() => {
                  setChosens(chosens => chosens.filter(c => c.id !== chosen.id))
                  setList(list => {
                    if (!list.includes(chosen)){
                      list.unshift(chosen)
                    }
                    return list
                  })
                }
              } >{chosen.name}</option>
            )
          }
        {!chosens.length? disabledOption(nyMessage) : <></>}
        </select>

  const searchView = <section
        onMouseOver={()  => setSelectViewDisplay(true)}
        onFocus={()      => setSelectViewDisplay(true)}
        onMouseLeave={() => {
            if(!isSearching){
              setInterval(() => {
                  setSelectViewDisplay(false)
              },theInterval)
            }
          }
        }
        className="searchBarView">
      {isMulti  ? resultSelect : <></>}
      {choiceSelect}
    </section>

  return (
    <div className={"searchBarContainer"} >
      {headInput}
      {selectViewDisplay || isSearching || target.length > 0 ? searchView : <></>}
    </div>
  )
}
