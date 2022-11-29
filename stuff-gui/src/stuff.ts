import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export interface ITask {
  id: string;
  title: string;
  description: string;

  projectId?: string;
  areaId?: string;
  projectHeadingId?: string;
}

export interface IProject {
  id: string;
  name: string;
}

export interface IArea {
  id: string;
  name: string;
}

export function useProject(projectId: string) {
  const projects = useProjects();
  return projects.find((project) => project.id === projectId);
}

export function useArea(areaId: string) {
  const areas = useAreas();
  return areas.find((area) => area.id === areaId);
}

function useStuffState() {
  const [tasks, setTasks] = useState([]);
  const [projects, setProjects] = useState([]);
  const [projectHeadings, setProjectHeadings] = useState([]);
  const [areas, setAreas] = useState([]);

  useEffect(() => {
    listen("next-stuff-state", () => {
      invoke("tasks").then(setTasks);
      invoke("projects").then(setProjects);
      invoke("project_headings").then(setProjectHeadings);
      invoke("areas").then(setAreas);
    });
  }, [setTasks, setProjects, setProjectHeadings, setAreas]);

  useEffect(() => {
    invoke("tasks").then(setTasks);
    invoke("projects").then(setProjects);
    invoke("project_headings").then(setProjectHeadings);
    invoke("areas").then(setAreas);
  }, []);

  return { tasks, projects, projectHeadings, areas };
}

export function useProjects(): IProject[] {
  const { projects } = useStuffState();
  return projects;
}

export function useProjectHeadings(projectId: string) {
  const { projectHeadings } = useStuffState();
  return projectHeadings.filter((heading) => heading.projectId === projectId);
}

export function useTasks(): ITask[] {
  const { tasks } = useStuffState();
  return tasks;
}

export function useAreas() {
  const { areas } = useStuffState();
  return areas;
}

export async function addTask(
  title: string,
  description?: string,
  projectId?: string,
  projectHeadingId?: string,
  areaId?: string
) {
  await invoke("add_task", {
    title,
    description,
    projectId,
    projectHeadingId,
    areaId,
  });
}

export async function updateTaskTitle(taskId: string, title: string) {
  await invoke("update_task_title", { taskId, title });
}

export async function updateTaskDescription(
  taskId: string,
  description: string
) {
  await invoke("update_task_description", { taskId, description });
}

export async function createProject(name: string): Promise<IProject | null> {
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

export async function createArea(name: string): Promise<IArea | undefined> {
  return await invoke("create_area", { name });
}
