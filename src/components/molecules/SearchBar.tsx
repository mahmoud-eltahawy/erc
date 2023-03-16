import { createSignal,createResource, Show,For, createEffect } from "solid-js"
import { SetStoreFunction } from "solid-js/store"
import { css } from "solid-styled-components"
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

  const getChosenOne = () => {
    if (chosens.at(0)){
        return resultPlaceholder + " : " + chosens.at(0)!.name
    } else {
        return defaultPlaceholder
    }
  }

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

  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  })

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
  })

  const viewMember = css({
    display: "inline-block",
    fontSize: "20px",
    margin: "20px auto",
    width: "40%",
    backgroundColor: "inherit",
    borderLeft: "solid 5px",
    borderRight: "solid 5px",
    borderBottom: "solid 5px",
    borderTop: "none",
    borderBottomLeftRadius : "20px",
    borderBottomRightRadius : "20px",
  })

  const inputStyle = css({
    display: "block",
    backgroundColor: "transparent",
    fontSize: "24px",
    width: "70%",
    padding: ".1em",
    margin: ".1em auto",
  })

  return (
    <div class={container} >
        <input
          placeholder={isMulti ? `${resultPlaceholder} : ${chosens.length}` :  getChosenOne()}
          class={inputStyle}
          type="text"
          value={target()!}
          onInput={e => {
            setTarget(e.currentTarget.value)
            refetch()
          }} />
        <Show when={(target() || '').length > 0}>
          <section class={viewContainer}>
            <Show when={isMulti}>
              <select multiple class={viewMember}>
                {
                    <For each={chosens}>
                        {
                            (item) => (
                              <option onClick={() => resultOptionHandler(item)}>{item.name}</option>
                            )
                        }
                    </For>
                }
                <Show when={!chosens.length}><option disabled>{nyMessage}</option></Show>
              </select>
            </Show>
            <select multiple class={viewMember}>
              {
                  <For each={optionsList()}>
                      {
                          (item) => (
                            <option onClick={() => choiceOptionHandler(item)}>{item.name}</option>
                          )
                      }
                  </For>
              }
              <Show when={!(optionsList() || []).length}><option disabled>{mtMessage}</option></Show>
            </select>
          </section>
      </Show>
    </div>
  )
}
