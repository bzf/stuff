import { useState } from "react";
import { addTask } from "../stuff";

export default function NewTaskForm() {
  const [title, setTitle] = useState("");
  const [showInputForm, setShowInputForm] = useState(false);

  function handleAdd() {
    if (title === "") return;

    addTask(title);
    setTitle("");
  }

  if (showInputForm) {
    return (
      <div className="flex flex-col gap-2">
        <div className="rounded-lg px-3 py-2 border">
          <InlineNewTaskForm
            title={title}
            setTitle={setTitle}
            onSubmit={handleAdd}
            onCancel={() => setShowInputForm(false)}
          />
        </div>

        <div className="flex justify-end items-center gap-4">
          <button onClick={() => setShowInputForm(false)}>Cancel</button>
          <button onClick={handleAdd}>Add task</button>
        </div>
      </div>
    );
  } else {
    return (
      <div>
        <button onClick={() => setShowInputForm(true)} className="block">
          Add task
        </button>
      </div>
    );
  }
}

interface InlineNewTaskFormArgs {
  title: string;
  setTitle: (title: string) => void;
  onSubmit: () => void;
  onCancel: () => void;
}

function InlineNewTaskForm({
  title,
  setTitle,
  onSubmit,
  onCancel,
}: InlineNewTaskFormArgs) {
  function handleKeyDown(event: any) {
    switch (event.key) {
      case "Enter":
        onSubmit();
        break;

      case "Escape":
        onCancel();
        break;
    }
  }

  return (
    <div className="row">
      <div>
        <input
          id="greet-input"
          value={title}
          autoFocus={true}
          onKeyDown={handleKeyDown}
          onChange={(e) => setTitle(e.currentTarget.value)}
          placeholder="Enter a name..."
          onSubmit={onSubmit}
        />
      </div>
    </div>
  );
}
