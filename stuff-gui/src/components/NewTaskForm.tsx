import { useState } from "react";
import { addTask } from "../stuff";

export default function NewTaskForm() {
  const [title, setTitle] = useState("");

  function handleAdd() {
    if (title === "") return;

    addTask(title);
    setTitle("");
  }

  return (
    <div className="row">
      <div>
        <input
          id="greet-input"
          value={title}
          onChange={(e) => setTitle(e.currentTarget.value)}
          placeholder="Enter a name..."
          onSubmit={handleAdd}
        />

        <button onClick={handleAdd} type="button">
          Add
        </button>
      </div>
    </div>
  );
}
