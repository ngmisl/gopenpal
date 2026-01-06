import { useState } from "react";
import { useSettingsStore, type ModelConfig } from "../store/settings";

export function ModelSelector() {
  const { selectedModel, setSelectedModel, getAllModels, addCustomModel, removeCustomModel } =
    useSettingsStore();
  const [showCustomModal, setShowCustomModal] = useState(false);
  const [customModelForm, setCustomModelForm] = useState({
    id: "",
    name: "",
    provider: "",
    contextWindow: 128000,
  });

  const allModels = getAllModels();
  const selectedModelInfo = allModels.find((m) => m.id === selectedModel);

  const handleAddCustomModel = () => {
    if (customModelForm.id && customModelForm.name && customModelForm.provider) {
      addCustomModel({
        ...customModelForm,
        custom: true,
      });
      setCustomModelForm({ id: "", name: "", provider: "", contextWindow: 128000 });
      setShowCustomModal(false);
    }
  };

  // Group models by provider
  const modelsByProvider = allModels.reduce((acc, model) => {
    const provider = model.provider;
    if (!acc[provider]) {
      acc[provider] = [];
    }
    acc[provider].push(model);
    return acc;
  }, {} as Record<string, ModelConfig[]>);

  return (
    <div className="bg-white rounded-lg shadow-md p-4">
      <div className="flex items-center justify-between mb-3">
        <h3 className="font-semibold flex items-center">
          <span className="text-xl mr-2">🤖</span>
          AI Model
        </h3>
        <button
          onClick={() => setShowCustomModal(true)}
          className="text-xs px-2 py-1 bg-blue-50 hover:bg-blue-100 text-blue-600 rounded transition-colors"
        >
          + Add Custom
        </button>
      </div>

      {/* Model Selector */}
      <select
        value={selectedModel}
        onChange={(e) => setSelectedModel(e.target.value)}
        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
      >
        {Object.entries(modelsByProvider).map(([provider, models]) => (
          <optgroup key={provider} label={provider}>
            {models.map((model) => (
              <option key={model.id} value={model.id}>
                {model.name}
                {model.custom && " (Custom)"}
              </option>
            ))}
          </optgroup>
        ))}
      </select>

      {/* Model Info */}
      {selectedModelInfo && (
        <div className="mt-2 text-xs text-gray-600 space-y-1">
          <div className="flex justify-between">
            <span>Provider:</span>
            <span className="font-medium">{selectedModelInfo.provider}</span>
          </div>
          {selectedModelInfo.contextWindow && (
            <div className="flex justify-between">
              <span>Context:</span>
              <span className="font-medium">
                {(selectedModelInfo.contextWindow / 1000).toFixed(0)}K tokens
              </span>
            </div>
          )}
          {selectedModelInfo.custom && (
            <button
              onClick={() => removeCustomModel(selectedModelInfo.id)}
              className="text-red-500 hover:text-red-600 text-xs mt-1"
            >
              Remove Custom Model
            </button>
          )}
        </div>
      )}

      {/* Custom Model Modal */}
      {showCustomModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h2 className="text-xl font-bold mb-4">Add Custom Model</h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">Model ID</label>
                <input
                  type="text"
                  value={customModelForm.id}
                  onChange={(e) => setCustomModelForm({ ...customModelForm, id: e.target.value })}
                  placeholder="e.g., anthropic/claude-3-opus"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <p className="text-xs text-gray-500 mt-1">
                  Find model IDs at{" "}
                  <a
                    href="https://openrouter.ai/models"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-blue-600 hover:underline"
                  >
                    openrouter.ai/models
                  </a>
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">Display Name</label>
                <input
                  type="text"
                  value={customModelForm.name}
                  onChange={(e) => setCustomModelForm({ ...customModelForm, name: e.target.value })}
                  placeholder="e.g., Claude 3 Opus"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">Provider</label>
                <input
                  type="text"
                  value={customModelForm.provider}
                  onChange={(e) =>
                    setCustomModelForm({ ...customModelForm, provider: e.target.value })
                  }
                  placeholder="e.g., Anthropic"
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">
                  Context Window (tokens)
                </label>
                <input
                  type="number"
                  value={customModelForm.contextWindow}
                  onChange={(e) =>
                    setCustomModelForm({
                      ...customModelForm,
                      contextWindow: parseInt(e.target.value),
                    })
                  }
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>

            <div className="flex space-x-3 mt-6">
              <button
                onClick={handleAddCustomModel}
                disabled={!customModelForm.id || !customModelForm.name || !customModelForm.provider}
                className="flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
              >
                Add Model
              </button>
              <button
                onClick={() => {
                  setShowCustomModal(false);
                  setCustomModelForm({ id: "", name: "", provider: "", contextWindow: 128000 });
                }}
                className="flex-1 px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
