import { useRouter } from "next/router";
import { useState } from "react";
import { ReactSortable } from "react-sortablejs";
import _ from "lodash";
import NewTaskForm from "../../components/NewTaskForm";
import TaskItem from "../../components/TaskItem";
import {
  addProjectHeading,
  clearTaskProjectHeading,
  moveTaskToPosition,
  moveTaskToProjectHeading,
  renameProject,
  useProject,
  useProjectHeadings,
  useTasks,
} from "../../stuff";
import useEditTask from "../../hooks/useEditTask";
import TaskForm from "../../components/TaskForm";

export default function Project() {
  const { projectId } = useRouter().query;
  const project = useProject(projectId);
  const headings = useProjectHeadings(projectId);
  const tasks = useTasks();
  const projectTasks = tasks.filter((task) => task.projectId === projectId);
  const { save, cancel, editTask, setEditTask } = useEditTask();

  if (tasks === undefined || project === undefined) {
    return <div>loading</div>;
  }

  return (
    <>
      <div>
        <div className="pb-2">
          <div className="flex flex-col pb-4 px-3">
            <input
              value={project.name}
              onChange={(event) => renameProject(projectId, event.target.value)}
              placeholder="New project"
              className="text-4xl font-bold tracking-tight text-gray-900 pt-8 pb-4"
            />

            <ReactSortable
              group="groupName"
              animation={200}
              delay={2}
              list={projectTasks}
              setList={() => null}
              onEnd={handleTaskMove}
            >
              {projectTasks
                .filter((t) => _.isEmpty(t.projectHeadingId))
                .map((task) =>
                  task.id === editTask?.id ? (
                    <TaskForm
                      key={task.id}
                      initialNote={editTask.description}
                      initialTitle={editTask.title}
                      initialDeferDate={editTask.deferDate}
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

          <NewTaskForm projectId={project.id} />
        </div>

        <div className="flex flex-col gap-6">
          <NewProjectHeading projectId={project.id} index={0} />

          {headings.map((heading, index) => (
            <div className="flex flex-col gap-4" key={heading.id}>
              <ProjectHeading
                key={heading.id}
                save={save}
                cancel={cancel}
                setEditTask={setEditTask}
                editTask={editTask}
                heading={heading}
                tasks={projectTasks.filter(
                  (t) => t.projectHeadingId === heading.id
                )}
              />

              <NewProjectHeading
                key={`${heading.id}/heading`}
                index={index + 1}
                projectId={project.id}
              />
            </div>
          ))}
        </div>
      </div>
    </>
  );
}

function ProjectHeading({
  heading,
  tasks,
  editTask,
  setEditTask,
  save,
  cancel,
}) {
  const headingTasks = tasks.filter(
    (task) => task.projectHeadingId === heading.id
  );

  return (
    <div>
      <div className="flex items-end pb-2 mb-2 border-b gap-3 px-3">
        <h2 className="text-xl font-semibold tracking-tight text-gray-700">
          {heading.name}
        </h2>

        <span className="text-sm mb-px text-gray-500">
          {headingTasks.length}
        </span>
      </div>

      <div className="px-3 pb-2">
        <ReactSortable
          group="groupName"
          animation={200}
          delay={2}
          list={tasks}
          setList={() => null}
          onEnd={handleTaskMove}
          id={heading.id}
        >
          {tasks.map((task) =>
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

      <NewTaskForm
        projectId={heading.projectId}
        projectHeadingId={heading.id}
      />
    </div>
  );
}

function NewProjectHeading({ projectId, index }) {
  const [showForm, setShowForm] = useState(false);
  const [headingTitle, setHeadingTitle] = useState("");

  function handleCreate() {
    if (headingTitle === "") return;

    addProjectHeading(projectId, headingTitle, index);
    handleCancel();
  }

  function handleCancel() {
    setHeadingTitle("");
    setShowForm(false);
  }

  if (showForm) {
    return (
      <div className="flex justify-between">
        <InlineNewTaskForm
          title={headingTitle}
          setTitle={setHeadingTitle}
          onSubmit={handleCreate}
          onCancel={handleCancel}
        />

        <button onClick={handleCancel}>Add</button>
      </div>
    );
  } else {
    return (
      <div className="opacity-0 hover:opacity-100 transition flex items-center justify-center">
        <div className="flex-1 h-px bg-gray-200" />

        <button className="px-4 font-medium" onClick={() => setShowForm(true)}>
          Add heading...
        </button>

        <div className="flex-1 h-px bg-gray-200" />
      </div>
    );
  }
}

interface InlineNewTaskFormArgs {
  title: string;
  setTitle: (title: string) => void;
  onSubmit: () => void;
  onCancel: () => void;
}

function InlineNewTaskForm({
  title,
  setTitle,
  onSubmit,
  onCancel,
}: InlineNewTaskFormArgs) {
  function handleKeyDown(event: any) {
    switch (event.key) {
      case "Enter":
        onSubmit();
        break;

      case "Escape":
        onCancel();
        break;
    }
  }

  return (
    <div className="row">
      <div>
        <input
          id="greet-input"
          value={title}
          autoFocus={true}
          onKeyDown={handleKeyDown}
          onChange={(e) => setTitle(e.currentTarget.value)}
          placeholder="Enter a name..."
          onSubmit={onSubmit}
        />
      </div>
    </div>
  );
}

function handleTaskMove(event) {
  const { newIndex, item, to } = event;
  const headingId = _.isEmpty(to.id) ? undefined : to.id;
  const { taskId, headingId: taskHeadingId } = item.dataset;

  moveTaskToPosition(taskId, newIndex);

  if (headingId !== taskHeadingId) {
    if (_.isEmpty(headingId)) {
      clearTaskProjectHeading(taskId);
    } else {
      moveTaskToProjectHeading(taskId, headingId);
    }
  }
}
