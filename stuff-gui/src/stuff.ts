import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export function useProject(projectId: string) {
  const projects = useProjects();
  return projects.find((project) => project.id === projectId);
}

function useStuffState() {
  const [tasks, setTasks] = useState([]);
  const [projects, setProjects] = useState([]);

  useEffect(() => {
    listen("next-stuff-state", () => {
      invoke("tasks").then(setTasks);
      invoke("projects").then(setProjects);
    });
  }, [setTasks, setProjects]);

  useEffect(() => {
    invoke("tasks").then(setTasks);
    invoke("projects").then(setProjects);
  }, []);

  return { tasks, projects };
}

export function useProjects() {
  const { projects } = useStuffState();
  return projects;
}

export function useTasks() {
  const { tasks } = useStuffState();
  return tasks;
}

export async function addTask(title: string) {
  await invoke("add_task", { title });
}

export async function createProject(name: string) {
  await invoke("create_project", { name });
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
