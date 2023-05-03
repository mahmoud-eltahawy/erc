import { invoke } from "@tauri-apps/api";
import { createResource } from "solid-js";

export default function Namer({
  id,
  command,
}: {
  id: () => string;
  command: string;
}) {
  const fetcher = async () => {
    return (await invoke(command, { id: id() })
      .catch((err) => console.log(err))) as string;
  };

  let [name] = createResource(fetcher);

  return <p>{name()}</p>;
}
