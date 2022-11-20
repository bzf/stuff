import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

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
  const [data, setData] = useState(null);

  async function addTask() {
    await invoke("add_task", { title });
    setTitle("");

    loadData().then(setData);
  }

  useEffect(() => {
    loadData().then(setData);
  }, []);

  if (data === null) {
    return <div>loading</div>;
  }

  console.log({ data });

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
