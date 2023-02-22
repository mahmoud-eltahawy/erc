import { createSignal,createResource, Show,For, createEffect } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { Name } from "../../index"

export function SearchBar({
    defaultPlaceholder,
    resultPlaceholder,
    mtMessage,
    nyMessage = null,
    isMulti,
    chosens,
    setChosens,
    selection_fetcher,
    elementsNumber
} : {
    defaultPlaceholder   : string,
    resultPlaceholder    : string,
    mtMessage            : string,
    nyMessage            : string | null,
    isMulti              : boolean,
    selection_fetcher    : () => Promise<Name[]>,
    chosens              : Name[],
    setChosens           : SetStoreFunction<Name[]>,
    elementsNumber       : number[]
}){

  const wildChar = ' '
  const [optionsList,{refetch}] = createResource(selection_fetcher)

  const [target, setTarget]                             = createSignal('')
  const [list,setList]                                  = createSignal<Name[]>([])
  const filter = () => {
    setList((optionsList() || []).filter(membr => membr.name.includes(target()) || target() === wildChar))
  }

  const showSelectView = () => target().length > 0 || target() === wildChar

  createEffect(() => {
      if (elementsNumber[0] != 0) {
          refetch()
      }
  })
  const getChosenOne = () => {
    if (chosens.at(0)){
        return resultPlaceholder + " : " + chosens.at(0)!.name
    } else {
        return defaultPlaceholder
    }
  }

  const disabledOption = (message : string) => {
    return <option disabled>{message}</option>
  }


  const headInput = <input
    placeholder={isMulti ? `${resultPlaceholder} : ${chosens.length}` :  getChosenOne()}
    class={"insertField"}
    type="text"
    value={target()}
    onInput={e => {
      filter()
      setTarget(e.currentTarget.value)
    }} />


  const choiceOptionHandler = (member : Name) => {
                setChosens(prev => {
                  if(isMulti){
                    if (!prev.includes(member)){
                      return [member,...prev]
                    }
                    return prev
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
                  setChosens(prev => prev.filter(c => c.id !== chosen.id))
                  setList(list => [chosen,...list])
                }
  const choiceSelect = <select multiple class="searchBarViewMember">
          {
              <For each={list()}>
                  {
                      (item) => (
                        <option onClick={() => choiceOptionHandler(item)}>{item.name}</option>
                      )
                  }
              </For>
          }
          {!list().length? disabledOption(mtMessage): <></>}
        </select>

  const resultSelect = <select multiple class="searchBarViewMember">
          {
              <For each={chosens}>
                  {
                      (item) => (
                        <option onClick={() => resultOptionHandler(item)}>{item.name}</option>
                      )
                  }
              </For>
          }
        {!chosens.length? disabledOption(nyMessage!) : <></>}
        </select>

  const searchView = <section class="searchBarView">
      {isMulti  ? resultSelect : <></>} {choiceSelect} </section>

  return (
    <div class={"searchBarContainer"} >
      {headInput}
      <Show when={showSelectView()}>{searchView}</Show>
    </div>
  )
}
