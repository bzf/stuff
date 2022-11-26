import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export interface IProject {
  id: string;
  name: string;
}

export function useProject(projectId: string) {
  const projects = useProjects();
  return projects.find((project) => project.id === projectId);
}

function useStuffState() {
  const [tasks, setTasks] = useState([]);
  const [projects, setProjects] = useState([]);
  const [projectHeadings, setProjectHeadings] = useState([]);

  useEffect(() => {
    listen("next-stuff-state", () => {
      invoke("tasks").then(setTasks);
      invoke("projects").then(setProjects);
      invoke("project_headings").then(setProjectHeadings);
    });
  }, [setTasks, setProjects]);

  useEffect(() => {
    invoke("tasks").then(setTasks);
    invoke("projects").then(setProjects);
    invoke("project_headings").then(setProjectHeadings);
  }, []);

  return { tasks, projects, projectHeadings };
}

export function useProjects(): IProject[] {
  const { projects } = useStuffState();
  return projects;
}

export function useProjectHeadings(projectId: string) {
  const { projectHeadings } = useStuffState();
  return projectHeadings.filter((heading) => heading.projectId === projectId);
}

export function useTasks() {
  const { tasks } = useStuffState();
  return tasks;
}

export async function addTask(
  title: string,
  description?: string,
  projectId?: string,
  projectHeadingId?: string
) {
  await invoke("add_task", { title, description, projectId, projectHeadingId });
}

export async function createProject(name: string): Option<IProject> {
  return await invoke("create_project", { name });
}

export async function addProjectHeading(
  projectId: string,
  name: string,
  index: number
) {
  await invoke("add_project_heading", { projectId, name, index });
}

export async function markTaskAsComplete(taskId: string) {
  await invoke("mark_task_as_complete", { taskId });
}

export async function markTaskAsIncomplete(taskId: string) {
  await invoke("mark_task_as_incomplete", { taskId });
}

export async function moveTaskToInbox(taskId: string) {
  await invoke("move_task_to_inbox", { taskId });
}

export async function moveTaskToProject(taskId: string, projectId: string) {
  await invoke("move_task_to_project", { taskId, projectId });
}
