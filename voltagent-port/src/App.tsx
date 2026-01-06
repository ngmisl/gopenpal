import { useState } from "react";
import { AgentSelector } from "./components/AgentSelector";
import { ChatInterface } from "./components/ChatInterface";
import { WaterTracker } from "./components/WaterTracker";
import { TaskList } from "./components/TaskList";

function App() {
  const [activeTab, setActiveTab] = useState<"chat" | "water" | "tasks">("chat");

  return (
    <main className="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="mb-8 text-center">
          <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent">
            GopenPal
          </h1>
          <p className="mt-2 text-gray-600">Your AI-powered health & productivity companion</p>
        </div>

        {/* Agent Selector */}
        <div className="mb-6">
          <AgentSelector />
        </div>

        {/* Main Content */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Left Sidebar - Quick Stats */}
          <div className="lg:col-span-1 space-y-4">
            <WaterTracker compact />
            <TaskList compact />
          </div>

          {/* Main Area - Chat Interface */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-lg shadow-lg p-6">
              {/* Tabs */}
              <div className="flex space-x-4 mb-6 border-b">
                <button
                  onClick={() => setActiveTab("chat")}
                  className={`pb-2 px-4 font-medium transition-colors ${
                    activeTab === "chat"
                      ? "border-b-2 border-blue-500 text-blue-600"
                      : "text-gray-500 hover:text-gray-700"
                  }`}
                >
                  Chat
                </button>
                <button
                  onClick={() => setActiveTab("water")}
                  className={`pb-2 px-4 font-medium transition-colors ${
                    activeTab === "water"
                      ? "border-b-2 border-blue-500 text-blue-600"
                      : "text-gray-500 hover:text-gray-700"
                  }`}
                >
                  Water Tracking
                </button>
                <button
                  onClick={() => setActiveTab("tasks")}
                  className={`pb-2 px-4 font-medium transition-colors ${
                    activeTab === "tasks"
                      ? "border-b-2 border-blue-500 text-blue-600"
                      : "text-gray-500 hover:text-gray-700"
                  }`}
                >
                  Tasks
                </button>
              </div>

              {/* Tab Content */}
              {activeTab === "chat" && <ChatInterface />}
              {activeTab === "water" && <WaterTracker />}
              {activeTab === "tasks" && <TaskList />}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-8 text-center text-sm text-gray-500">
          Powered by{" "}
          <a
            href="https://voltagent.ai"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:underline"
          >
            VoltAgent
          </a>{" "}
          &{" "}
          <a
            href="https://openrouter.ai"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:underline"
          >
            OpenRouter
          </a>
        </div>
      </div>
    </main>
  );
}

export default App;
