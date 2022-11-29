import { useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";
import NewTaskForm from "../components/NewTaskForm";
import TaskItem from "../components/TaskItem";
import TaskForm from "../components/TaskForm";
import useEditTask from "../hooks/useEditTask";

function App() {
  const tasks = useTasks();
  const { save, cancel, editTask, setEditTask } = useEditTask();

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => !task.projectId && !task.areaId);

  return (
    <>
      <div>
        <div className="flex flex-col pb-4 px-3">
          <PageTitle title="Inbox" />

          {visibleTasks.map((task) =>
            task.id === editTask?.id ? (
              <TaskForm
                key={task.id}
                initialNote={editTask.description}
                initialTitle={editTask.title}
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

export default App;
