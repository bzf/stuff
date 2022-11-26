import { useState } from "react";
import _ from "lodash";
import { createProject } from "../../stuff";
import { useRouter } from "next/router";

export default function ProjectNewPage() {
  const router = useRouter();
  const [title, setTitle] = useState("");

  async function handleCreate() {
    if (_.isEmpty(title)) return;

    const project = await createProject(title);
    router.replace(`/projects/${project.id}`);
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
        placeholder="Project name"
        autoFocus
      />
    </div>
  );
}
