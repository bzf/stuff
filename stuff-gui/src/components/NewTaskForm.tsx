import { useState } from "react";
import _ from "lodash";
import { addTask } from "../stuff";

export default function NewTaskForm({ projectId }) {
  const [title, setTitle] = useState("");
  const [notes, setNotes] = useState("");

  const [showInputForm, setShowInputForm] = useState(false);

  function handleAdd() {
    if (title === "") return;

    addTask(title, presence(notes), projectId);
    clearInput();
  }

  function handleCancel() {
    clearInput();
    setShowInputForm(false);
  }

  function clearInput() {
    setTitle("");
    setNotes("");
  }

  if (showInputForm) {
    return (
      <div className="flex flex-col gap-2">
        <div className="rounded-lg px-3 py-2 border flex gap-3">
          <input type="checkbox" disabled className="mt-[7px]" />

          <div className="flex-1 flex flex-col gap-1">
            <InlineNewTaskForm
              title={title}
              setTitle={setTitle}
              onSubmit={handleAdd}
              onCancel={handleCancel}
            />

            <textarea
              placeholder="Notes"
              className="resize-none text-gray-700 w-full outline-none text-sm"
              onChange={(event) => setNotes(event.target.value)}
              value={notes}
            />
          </div>
        </div>

        <div className="flex justify-end items-center gap-4">
          <button onClick={handleCancel}>Cancel</button>
          <button onClick={handleAdd}>Add task</button>
        </div>
      </div>
    );
  } else {
    return (
      <div className="px-3">
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
          className="w-full outline-none"
          value={title}
          autoFocus={true}
          onKeyDown={handleKeyDown}
          onChange={(e) => setTitle(e.currentTarget.value)}
          placeholder="New task"
          onSubmit={onSubmit}
        />
      </div>
    </div>
  );
}

function presence(input: string) {
  if (_.isEmpty(input)) {
    return null;
  } else {
    return input;
  }
}
