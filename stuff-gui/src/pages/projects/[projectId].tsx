import { useRouter } from "next/router";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import { useProject, useTasks } from "../../stuff";

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

export default function Project() {
  const { projectId } = useRouter().query;
  console.log({ projectId });
  const project = useProject(projectId);
  const tasks = useTasks();

  if (tasks === undefined || project === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => task.projectId === project.id);

  return (
    <>
      <PageTitle title={project.name} />

      <div>
        <div className="flex flex-col pb-4">
          {visibleTasks.map((task) => (
            <TaskItem task={task} key={task.id} />
          ))}
        </div>

        <NewTaskForm projectId={project.id} />
      </div>
    </>
  );
}
