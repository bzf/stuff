import { useRouter } from "next/router";
import _ from "lodash";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import TaskItem from "../../components/TaskItem";
import { moveTaskToPosition, useArea, useTasks } from "../../stuff";
import TaskForm from "../../components/TaskForm";
import useEditTask from "../../hooks/useEditTask";
import { ReactSortable } from "react-sortablejs";

export default function Area() {
  const { areaId } = useRouter().query;
  const area = useArea(areaId);
  const tasks = useTasks();
  const areaTasks = tasks.filter((task) => task.areaId === areaId);
  const { save, cancel, editTask, setEditTask } = useEditTask();

  if (tasks === undefined || area === undefined) {
    return <div>loading</div>;
  }

  function handleTaskMove(event) {
    const { newIndex, item } = event;
    const { taskId } = item.dataset;

    moveTaskToPosition(taskId, newIndex);
  }

  return (
    <>
      <div>
        <div className="pb-2">
          <div className="flex flex-col pb-4 px-3">
            <PageTitle title={area.name} />

            <ReactSortable
              group="groupName"
              animation={200}
              delay={2}
              list={areaTasks}
              setList={() => null}
              onEnd={handleTaskMove}
            >
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
            </ReactSortable>
          </div>

          <NewTaskForm areaId={area.id} />
        </div>
      </div>
    </>
  );
}
