import { updateTaskDescription, updateTaskTitle, useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";
import NewTaskForm from "../components/NewTaskForm";
import TaskItem from "../components/TaskItem";
import { useState } from "react";
import TaskForm from "../components/TaskForm";

function App() {
  const tasks = useTasks();
  const [editingTaskId, setEditingTaskId] = useState(null);

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => !task.projectId && !task.areaId);

  async function handleUpdate(title: string, description: string) {
    const editingTask = tasks.find((task) => task.id === editingTaskId);
    const promises = [];

    if (title !== editingTask.title) {
      promises.push(updateTaskTitle(editingTaskId, title));
    }

    if (description !== editingTask.description) {
      promises.push(updateTaskDescription(editingTaskId, description));
    }

    await Promise.allSettled(promises);
    setEditingTaskId(null);
  }

  return (
    <>
      <div>
        <div className="flex flex-col pb-4 px-3">
          <PageTitle title="Inbox" />

          {visibleTasks.map((task) =>
            task.id === editingTaskId ? (
              <TaskForm
                key={task.id}
                initialNote={task.description}
                initialTitle={task.title}
                onUpdate={handleUpdate}
                onCancel={() => setEditingTaskId(null)}
              />
            ) : (
              <TaskItem
                task={task}
                key={task.id}
                onDoubleClick={() => setEditingTaskId(task.id)}
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
