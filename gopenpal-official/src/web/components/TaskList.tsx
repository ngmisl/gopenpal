import { useState, useEffect } from "react";
import { useTasksStore } from "../store/tasks";

export function TaskList({ compact = false }: { compact?: boolean }) {
  const { tasks, setTasks, updateTask } = useTasksStore();
  const [newTaskTitle, setNewTaskTitle] = useState("");

  // Mock data for demo
  useEffect(() => {
    setTasks([
      {
        id: "1",
        title: "Review project proposal",
        priority: "high",
        status: "pending",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        due_date: null,
        completed_at: null,
      },
      {
        id: "2",
        title: "Call with team",
        priority: "medium",
        status: "in_progress",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        due_date: null,
        completed_at: null,
      },
    ]);
  }, [setTasks]);

  const pendingTasks = tasks.filter((t) => t.status !== "completed");
  const completedTasks = tasks.filter((t) => t.status === "completed");

  if (compact) {
    return (
      <div className="bg-white rounded-lg shadow-md p-4">
        <h3 className="font-semibold mb-2 flex items-center">
          <span className="text-xl mr-2">📋</span>
          Tasks
        </h3>
        <div className="space-y-2">
          {pendingTasks.slice(0, 3).map((task) => (
            <div key={task.id} className="flex items-start space-x-2 text-sm">
              <input
                type="checkbox"
                checked={task.status === "completed"}
                onChange={() =>
                  updateTask(task.id, {
                    status: "completed",
                    completed_at: new Date().toISOString(),
                  })
                }
                className="mt-1"
              />
              <span className={task.status === "completed" ? "line-through text-gray-500" : ""}>
                {task.title}
              </span>
            </div>
          ))}
        </div>
        <div className="mt-3 text-xs text-gray-500">
          {pendingTasks.length} pending • {completedTasks.length} completed
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Add Task */}
      <div className="flex space-x-2">
        <input
          type="text"
          value={newTaskTitle}
          onChange={(e) => setNewTaskTitle(e.target.value)}
          onKeyPress={(e) => {
            if (e.key === "Enter" && newTaskTitle.trim()) {
              setTasks([
                ...tasks,
                {
                  id: crypto.randomUUID(),
                  title: newTaskTitle,
                  priority: "medium",
                  status: "pending",
                  created_at: new Date().toISOString(),
                  updated_at: new Date().toISOString(),
                  due_date: null,
                  completed_at: null,
                },
              ]);
              setNewTaskTitle("");
            }
          }}
          placeholder="Add a new task..."
          className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
        />
        <button
          onClick={() => {
            if (newTaskTitle.trim()) {
              setTasks([
                ...tasks,
                {
                  id: crypto.randomUUID(),
                  title: newTaskTitle,
                  priority: "medium",
                  status: "pending",
                  created_at: new Date().toISOString(),
                  updated_at: new Date().toISOString(),
                  due_date: null,
                  completed_at: null,
                },
              ]);
              setNewTaskTitle("");
            }
          }}
          className="px-6 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition-colors"
        >
          Add
        </button>
      </div>

      {/* Task List */}
      <div className="space-y-3">
        <h3 className="font-semibold">Pending Tasks ({pendingTasks.length})</h3>
        {pendingTasks.map((task) => (
          <div key={task.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div className="flex items-center space-x-3">
              <input
                type="checkbox"
                checked={false}
                onChange={() =>
                  updateTask(task.id, {
                    status: "completed",
                    completed_at: new Date().toISOString(),
                  })
                }
              />
              <div>
                <div className="font-medium">{task.title}</div>
                <div className="text-xs text-gray-500">
                  Priority: {task.priority} • Status: {task.status}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      {completedTasks.length > 0 && (
        <div className="space-y-3">
          <h3 className="font-semibold text-gray-600">Completed Tasks ({completedTasks.length})</h3>
          {completedTasks.map((task) => (
            <div key={task.id} className="flex items-center space-x-3 p-3 bg-green-50 rounded-lg">
              <input type="checkbox" checked={true} readOnly />
              <div className="line-through text-gray-500">{task.title}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
