import { create } from "zustand";
import type { Task, TaskStatus, TaskPriority } from "../schemas/task";

interface TasksState {
  tasks: Task[];
  filter: {
    status?: TaskStatus;
    priority?: TaskPriority;
  };
  isLoading: boolean;

  // Actions
  setTasks: (tasks: Task[]) => void;
  addTask: (task: Task) => void;
  updateTask: (id: string, updates: Partial<Task>) => void;
  setFilter: (filter: { status?: TaskStatus; priority?: TaskPriority }) => void;
  setIsLoading: (isLoading: boolean) => void;
}

export const useTasksStore = create<TasksState>((set) => ({
  tasks: [],
  filter: {},
  isLoading: false,

  setTasks: (tasks) => set({ tasks }),

  addTask: (task) =>
    set((state) => ({
      tasks: [task, ...state.tasks],
    })),

  updateTask: (id, updates) =>
    set((state) => ({
      tasks: state.tasks.map((task) => (task.id === id ? { ...task, ...updates } : task)),
    })),

  setFilter: (filter) => set({ filter }),

  setIsLoading: (isLoading) => set({ isLoading }),
}));
