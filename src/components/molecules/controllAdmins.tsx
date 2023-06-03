import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { createResource, createSignal, For, Show } from "solid-js";
import { css } from "solid-styled-components";
import Namer from "../atoms/Namer";

const non_admins_fetcher = async (props: { name: () => string | null }) => {
  return (await invoke("search_non_admins", {
    name: props.name() !== " " ? props.name() : null,
  })) as string[];
};

const admins_fetcher = async () => {
  return (await invoke("search_admins")) as string[];
};

const [target, setTarget] = createSignal<string | null>(null);

const [admins, a] = createResource(admins_fetcher);
const [nonAdmins, na] = createResource(
  { name: () => target() },
  non_admins_fetcher,
);

export default function ControllAdmins() {
  const container = css({
    display: "block",
    padding: ".1em",
    margin: "10px auto",
  });

  const viewContainer = css({
    display: "flex",
    padding: ".1em",
  });

  const inputStyle = css({
    display: "block",
    backgroundColor: "transparent",
    fontSize: "24px",
    width: "70%",
    padding: ".1em",
    margin: ".1em auto",
  });

  return (
    <div class={container}>
      <input
        placeholder={"ابحث عن موظف للتمكين"}
        class={inputStyle}
        type="text"
        value={target()!}
        onInput={(e) => {
          setTarget(e.currentTarget.value);
          na.refetch();
        }}
      />
      <section class={viewContainer}>
        <AdminsSection />
        <NonAdminSection />
      </section>
    </div>
  );
}

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
  borderBottomLeftRadius: "20px",
  borderBottomRightRadius: "20px",
});

function AdminsSection() {
  const handler = async (employeeId: string) => {
    await invoke("unadmin_employee", { employeeId });
  };

  const down = (id : string) => {
      a.mutate(xs => xs?.filter(x => x != id))
      na.refetch()
  }

  listen("update_employee_down",(e) => down(e.payload as string))

  return (
    <select multiple size={9} class={viewMember}>
      {
        <For each={admins()}>
          {(id) => (
            <option onClick={() => handler(id)}><Namer command="employee_name" id={() => id}/></option>
          )}
        </For>
      }
      <Show when={!(admins() || []).length}>
        <option disabled>لا يوجد موظفين ممكنين</option>
      </Show>
    </select>
  );
}

function NonAdminSection() {
  const handler = async (employeeId: string) => {
    await invoke("admin_employee", { employeeId });
  };

  const up = (id : string) => {
      a.mutate(xs => [id,...(xs || [])])
      na.refetch()
  }

  listen("update_employee_up",(e) => up(e.payload as string))

  return (
    <select multiple size={9} class={viewMember}>
      {
        <For each={nonAdmins()}>
          {(id) => (
            <option onClick={() => handler(id)}><Namer command="employee_name" id={() => id}/></option>
          )}
        </For>
      }
      <Show when={!(nonAdmins() || []).length}>
        <option disabled>لا يوجد موظفين</option>
      </Show>
    </select>
  );
}
