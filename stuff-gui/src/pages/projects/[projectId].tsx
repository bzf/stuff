import { useRouter } from "next/router";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import TaskItem from "../../components/TaskItem";
import { useProject, useTasks } from "../../stuff";

export default function Project() {
  const { projectId } = useRouter().query;
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
