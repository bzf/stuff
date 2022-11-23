import { markTaskAsComplete, markTaskAsIncomplete, useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";
import NewTaskForm from "../components/NewTaskForm";

function TaskItem({ task }) {
  function handleToggle() {
    if (!!task.completedAt) {
      markTaskAsIncomplete(task.id);
    } else {
      markTaskAsComplete(task.id);
    }
  }

  return (
    <div className="border-b py-3 flex gap-3 justify-start items-start">
      <span>
        <input
          type="checkbox"
          onChange={handleToggle}
          checked={!!task.completedAt}
        />
      </span>

      <div>
        <span className="font-medium text-md text-gray-700">{task.title}</span>
      </div>
    </div>
  );
}

function App() {
  const tasks = useTasks();

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => task.projectId === undefined);

  return (
    <>
      <PageTitle title="Inbox" />

      <div>
        <div className="flex flex-col pb-4">
          {visibleTasks.map((task) => (
            <TaskItem task={task} key={task.id} />
          ))}
        </div>
        <NewTaskForm />
      </div>
    </>
  );
}

export default App;
