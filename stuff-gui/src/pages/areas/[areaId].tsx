import { useRouter } from "next/router";
import _ from "lodash";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import TaskItem from "../../components/TaskItem";
import {
  moveTaskToPosition,
  useProjects,
  useArea,
  useTasks,
  renameArea,
} from "../../stuff";
import TaskForm from "../../components/TaskForm";
import useEditTask from "../../hooks/useEditTask";
import { ReactSortable } from "react-sortablejs";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHeart } from "@fortawesome/free-regular-svg-icons";

export default function Area() {
  const { areaId } = useRouter().query;
  const area = useArea(areaId);
  const projects = useProjects().filter((project) => project.areaId === areaId);
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
    <div className="flex flex-col gap-12">
      <div>
        <div className="pb-2">
          <div className="flex flex-col pb-4 px-3">
            <input
              value={area.name}
              onChange={(event) => renameArea(areaId, event.target.value)}
              placeholder="New area"
              className="text-4xl font-bold tracking-tight text-gray-900 pt-8 pb-4"
            />

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

      {projects.length > 0 ? (
        <div>
          {projects.map((project) => (
            <a
              key={project.id}
              href={`/projects/${project.id}`}
              className="flex items-center gap-2"
            >
              <FontAwesomeIcon icon={faHeart} />
              {project.name}
            </a>
          ))}
        </div>
      ) : null}

      <Link href={`/projects/new?areaId=${areaId}`} legacyBehavior>
        <a className="text-gray-700">
          <FontAwesomeIcon fixedWidth size="xs" icon={faPlus} />
          New project in <i>{area.name || "New area"}</i>
        </a>
      </Link>
    </div>
  );
}
