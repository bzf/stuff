import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useProjects, useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";

async function loadData() {
  const projects = await invoke("projects");
  const tasks = await invoke("tasks");

  return {
    projects,
    tasks,
  };
}

function App() {
  const [title, setTitle] = useState("");

  const projects = useProjects();
  const tasks = useTasks();

  return (
    <>
      <PageTitle title="Inbox" />

      <div className="row">
        <div>
          <input
            id="greet-input"
            value={title}
            onChange={(e) => setTitle(e.currentTarget.value)}
            placeholder="Enter a name..."
          />

          <button type="button">Add</button>
        </div>
      </div>
    </>
  );
}

export default App;
