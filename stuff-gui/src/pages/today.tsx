import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useProjects, useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";

function App() {
  const [title, setTitle] = useState("");

  return (
    <>
      <PageTitle title="Today" />

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
