import { useAgentsStore, agentMetadata } from "../store/agents";

export function AgentSelector() {
  const { selectedAgent, setSelectedAgent } = useAgentsStore();

  const agents = Object.values(agentMetadata);

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-lg font-semibold mb-4">Choose Your Agent</h2>
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {agents.map((agent) => {
          const isSelected = selectedAgent === agent.name;
          const colorClass =
            agent.name === "hydrix"
              ? "border-hydrix bg-hydrix/5"
              : agent.name === "serhant"
              ? "border-serhant bg-serhant/5"
              : agent.name === "mio"
              ? "border-mio bg-mio/5"
              : "border-karen bg-karen/5";

          return (
            <button
              key={agent.name}
              onClick={() => setSelectedAgent(isSelected ? null : agent.name)}
              className={`p-4 rounded-lg border-2 transition-all hover:scale-105 ${
                isSelected ? colorClass : "border-gray-200 hover:border-gray-300"
              }`}
            >
              <div className="text-3xl mb-2">{agent.icon}</div>
              <div className="font-semibold text-sm">{agent.display_name}</div>
              <div className="text-xs text-gray-500 mt-1">{agent.role}</div>
            </button>
          );
        })}
      </div>
      {selectedAgent && (
        <div className="mt-4 p-4 bg-gray-50 rounded-lg">
          <p className="text-sm text-gray-600">{agentMetadata[selectedAgent].description}</p>
        </div>
      )}
    </div>
  );
}
