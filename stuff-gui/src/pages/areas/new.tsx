import { useState } from "react";
import _ from "lodash";
import { createArea } from "../../stuff";
import { useRouter } from "next/router";

export default function AreaNewPage() {
  const router = useRouter();
  const [title, setTitle] = useState("");

  async function handleCreate() {
    if (_.isEmpty(title)) return;

    const area = await createArea(title);
    router.replace(`/areas/${area.id}`);
  }

  function handleKeydown() {
    if (event.key === "Enter") {
      handleCreate();
    }
  }

  return (
    <div>
      <input
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        onKeyDown={handleKeydown}
        className="text-4xl outline-none font-bold tracking-tight text-gray-900 pt-8 pb-4"
        placeholder="Area name"
        autoFocus
      />
    </div>
  );
}
