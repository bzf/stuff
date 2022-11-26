import { useRouter } from "next/router";
import { useState } from "react";
import NewTaskForm from "../../components/NewTaskForm";
import PageTitle from "../../components/PageTitle";
import TaskItem from "../../components/TaskItem";
import {
  addProjectHeading,
  useProject,
  useProjectHeadings,
  useTasks,
} from "../../stuff";

export default function Project() {
  const { projectId } = useRouter().query;
  const project = useProject(projectId);
  const headings = useProjectHeadings(projectId);
  const tasks = useTasks();

  console.log({ project, headings });

  if (tasks === undefined || project === undefined) {
    return <div>loading</div>;
  }

  const visibleTasks = tasks.filter((task) => task.projectId === project.id);

  return (
    <>
      <div>
        <div className="flex flex-col pb-4 px-3">
          <PageTitle title={project.name} />

          {visibleTasks.map((task) => (
            <TaskItem task={task} key={task.id} />
          ))}
        </div>

        <NewTaskForm projectId={project.id} />

        <div className="flex flex-col gap-6 px-3">
          {headings.map((heading) => (
            <ProjectHeading key={heading.id} heading={heading} tasks={tasks} />
          ))}

          <NewProjectHeading projectId={project.id} />
        </div>
      </div>
    </>
  );
}

function ProjectHeading({ heading, tasks }) {
  const headingTasks = tasks.filter(
    (task) => task.projectHeadingId === heading.id
  );

  return (
    <div>
      <div className="flex items-end pb-2 mb-2 border-b gap-3 pt-8">
        <h2 className="text-xl font-semibold tracking-tight text-gray-700">
          {heading.name}
        </h2>

        <span className="text-sm mb-px text-gray-500">
          {headingTasks.length}
        </span>
      </div>
    </div>
  );
}

function NewProjectHeading({ projectId }) {
  const [showForm, setShowForm] = useState(false);
  const [headingTitle, setHeadingTitle] = useState("");

  function handleCreate() {
    if (headingTitle === "") return;

    addProjectHeading(projectId, headingTitle);
    handleCancel();
  }

  function handleCancel() {
    setHeadingTitle("");
    setShowForm(false);
  }

  if (showForm) {
    return (
      <div className="flex justify-between bg-blue-100">
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
    return <button onClick={() => setShowForm(true)}>Add heading...</button>;
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
