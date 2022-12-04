import { useState } from "react";
import dayjs from "dayjs";

interface TaskFormArgs {
  initialTitle: string;
  initialNote: string;
  initialDeferDate?: Date;
  onUpdate: (title: string, description: string, deferDate?: string) => void;
  onCancel: () => void;
}

export default function TaskForm({
  initialTitle,
  initialNote,
  initialDeferDate,
  onUpdate,
  onCancel,
}: TaskFormArgs) {
  const [title, setTitle] = useState(initialTitle);
  const [notes, setNotes] = useState(initialNote);
  const [deferDate, setDeferDate] = useState(
    initialDeferDate && dayjs(initialDeferDate).format("YYYY-MM-DD")
  );

  const save = () => onUpdate(title, notes, deferDate);

  return (
    <div className="flex flex-col gap-2">
      <div className="rounded-lg px-3 py-2 border flex gap-3">
        <input type="checkbox" disabled className="mt-[7px]" />

        <div className="flex-1 flex flex-col gap-1">
          <InlineNewTaskForm
            title={title}
            setTitle={setTitle}
            onSubmit={save}
            onCancel={onCancel}
          />

          <textarea
            placeholder="Notes"
            className="resize-none text-gray-700 w-full outline-none text-sm"
            onChange={(event) => setNotes(event.target.value)}
            value={notes}
          />
        </div>

        <div>
          <label>
            <input
              value={deferDate}
              type="date"
              onChange={(event) => setDeferDate(event.target.value)}
            />
          </label>
        </div>
      </div>

      <div className="flex justify-end items-center gap-4">
        <button onClick={onCancel}>Cancel</button>
        <button onClick={save}>Update</button>
      </div>
    </div>
  );
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
          onSubmit={() => onSubmit()}
        />
      </div>
    </div>
  );
}
