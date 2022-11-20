import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useProjects, useTasks } from "../stuff";

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
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <div>
          <input
            id="greet-input"
            value={title}
            onChange={(e) => setTitle(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => addTask()}>
            Add
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
