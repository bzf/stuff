import { useState } from "react";
import { markTaskAsComplete, markTaskAsIncomplete, useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";

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
  const [title, setTitle] = useState("");

  const tasks = useTasks();

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => task.projectId === undefined);
  console.log({ visibleTasks });

  return (
    <>
      <PageTitle title="Inbox" />

      <div className="row">
        <div>
          <input
            id="greet-input"
            value={title}
            onChange={(e) => setTitle(e.currentTarget.value)}
            placeholder="Enter a name..."
          />

          <button type="button">Add</button>
        </div>
      </div>

      <div className="flex flex-col pb-4">
        {visibleTasks.map((task) => (
          <TaskItem task={task} key={task.id} />
        ))}
      </div>
    </>
  );
}

export default App;
