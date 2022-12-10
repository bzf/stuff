import { useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";
import NewTaskForm from "../components/NewTaskForm";
import TaskItem from "../components/TaskItem";
import TaskForm from "../components/TaskForm";
import useEditTask from "../hooks/useEditTask";
import dayjs from "dayjs";

export default function Today() {
  const tasks = useTasks();
  const { save, cancel, editTask, setEditTask } = useEditTask();

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks
    .filter((task) => !task.projectId && !task.areaId)
    .filter((task) => task.deferDate)
    .filter((task) => dayjs(task.deferDate).isBefore(new Date(), "day"));

  return (
    <>
      <div>
        <div className="flex flex-col pb-4 px-3">
          <PageTitle title="Today" />

          {visibleTasks.map((task) =>
            task.id === editTask?.id ? (
              <TaskForm
                key={task.id}
                initialNote={editTask.description}
                initialTitle={editTask.title}
                initialDeferDate={editTask.deferDate}
                onUpdate={save}
                onCancel={cancel}
              />
            ) : (
              <TaskItem
                task={task}
                key={task.id}
                onDoubleClick={() => setEditTask(task)}
              />
            )
          )}
        </div>

        <NewTaskForm projectId={undefined} />
      </div>
    </>
  );
}
