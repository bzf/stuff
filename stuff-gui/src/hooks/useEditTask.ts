import { useState } from "react";
import { ITask, updateTaskDescription, updateTaskTitle } from "../stuff";

interface UseEditTaskHook {
  save: (title: string, description: string) => void;
  cancel: () => void;
  editTask?: ITask;
  setEditTask: (task: ITask) => void;
}

export default function useEditTask(): UseEditTaskHook {
  const [editTask, setEditTask] = useState(null);

  return {
    editTask,
    setEditTask,
    async save(title: string, description: string) {
      const promises = [];

      if (title !== editTask.title) {
        promises.push(updateTaskTitle(editTask.id, title));
      }

      if (description !== editTask.description) {
        promises.push(updateTaskDescription(editTask.id, description));
      }

      await Promise.allSettled(promises);
      setEditTask(null);
    },
    cancel() {
      setEditTask(null);
    },
  };
}
