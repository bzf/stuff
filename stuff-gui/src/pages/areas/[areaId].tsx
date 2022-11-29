import { useRouter } from "next/router";
import _ from "lodash";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import TaskItem from "../../components/TaskItem";
import { useArea, useTasks } from "../../stuff";
import TaskForm from "../../components/TaskForm";
import useEditTask from "../../hooks/useEditTask";

export default function Area() {
  const { areaId } = useRouter().query;
  const area = useArea(areaId);
  const tasks = useTasks();
  const areaTasks = tasks.filter((task) => task.areaId === areaId);
  const { save, cancel, editTask, setEditTask } = useEditTask();

  if (tasks === undefined || area === undefined) {
    return <div>loading</div>;
  }

  return (
    <>
      <div>
        <div className="pb-2">
          <div className="flex flex-col pb-4 px-3">
            <PageTitle title={area.name} />

            {areaTasks.map((task) =>
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

          <NewTaskForm areaId={area.id} />
        </div>
      </div>
    </>
  );
}
