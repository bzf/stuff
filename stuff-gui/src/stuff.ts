import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export function useProjects() {
  const [projects, setProjects] = useState([]);

  useEffect(() => {
    invoke("projects").then(setProjects);
  }, []);

  return projects;
}

export function useTasks() {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    invoke("tasks").then(setTasks);
  }, []);

  return tasks;
}
