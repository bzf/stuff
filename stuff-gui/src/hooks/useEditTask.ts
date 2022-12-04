import dayjs from "dayjs";
import { useState } from "react";
import {
  ITask,
  updateTaskDeferDate,
  updateTaskDescription,
  updateTaskTitle,
} from "../stuff";

interface UseEditTaskHook {
  save: (title: string, description: string, deferDate?: Date) => void;
  cancel: () => void;
  editTask?: ITask;
  setEditTask: (task: ITask) => void;
}

export default function useEditTask(): UseEditTaskHook {
  const [editTask, setEditTask] = useState(null);

  return {
    editTask,
    setEditTask,
    async save(title: string, description: string, deferDate?: Date) {
      const promises = [];

      if (title !== editTask.title) {
        promises.push(updateTaskTitle(editTask.id, title));
      }

      if (description !== editTask.description) {
        promises.push(updateTaskDescription(editTask.id, description));
      }

      if (!dayjs(deferDate).isSame(editTask.deferDate)) {
        if (deferDate !== null) {
          promises.push(
            updateTaskDeferDate(
              editTask.id,
              dayjs(deferDate).format("YYYY-MM-DD")
            )
          );
        } else {
          promises.push(updateTaskDeferDate(editTask.id, null));
        }
      }

      await Promise.allSettled(promises);
      setEditTask(null);
    },
    cancel() {
      setEditTask(null);
    },
  };
}
