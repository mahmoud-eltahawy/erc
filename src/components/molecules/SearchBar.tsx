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
    subject,
    updates
} : {
    subject              : string,
    updates              : [string]
    defaultPlaceholder   : string,
    resultPlaceholder    : string,
    mtMessage            : string,
    nyMessage            : string | null,
    isMulti              : boolean,
    selection_fetcher    : (name : () => string | null) => Promise<Name[]>,
    chosens              : Name[],
    setChosens           : SetStoreFunction<Name[]>,
}){

  const [target, setTarget]     = createSignal<string | null>(null)
  const [optionsList,{refetch}] = createResource(() => target,selection_fetcher)

  createEffect(() => {
      if(updates[0] === subject || target()){
          refetch()
      }
  })

  const showSelectView = () => (target() || '').length > 0

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
    value={target()!}
    onInput={e => {
      refetch()
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
                if(!isMulti){
                  setTarget('')
                }
                refetch()
              }
  const resultOptionHandler = (chosen : Name) => {
      setChosens(prev => prev.filter(c => c.id !== chosen.id))
      refetch()
  }
  const choiceSelect = <select multiple class="searchBarViewMember">
          {
              <For each={optionsList()}>
                  {
                      (item) => (
                        <option onClick={() => choiceOptionHandler(item)}>{item.name}</option>
                      )
                  }
              </For>
          }
        {!(optionsList() || []).length? disabledOption(mtMessage): <></>}
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
