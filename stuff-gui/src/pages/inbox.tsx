import { useTasks } from "../stuff";
import PageTitle from "../components/PageTitle";
import NewTaskForm from "../components/NewTaskForm";
import TaskItem from "../components/TaskItem";

function App() {
  const tasks = useTasks();

  if (tasks === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => !task.projectId);

  return (
    <>
      <PageTitle title="Inbox" />

      <div>
        <div className="flex flex-col pb-4">
          {visibleTasks.map((task) => (
            <TaskItem task={task} key={task.id} />
          ))}
        </div>
        <NewTaskForm projectId={undefined} />
      </div>
    </>
  );
}

export default App;
