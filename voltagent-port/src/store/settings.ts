import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface ModelConfig {
  id: string;
  name: string;
  provider: string;
  contextWindow?: number;
  custom?: boolean;
}

export const defaultModels: ModelConfig[] = [
  {
    id: "anthropic/claude-3.5-sonnet",
    name: "Claude 3.5 Sonnet",
    provider: "Anthropic",
    contextWindow: 200000,
  },
  {
    id: "anthropic/claude-3-opus",
    name: "Claude 3 Opus",
    provider: "Anthropic",
    contextWindow: 200000,
  },
  {
    id: "anthropic/claude-3-haiku",
    name: "Claude 3 Haiku",
    provider: "Anthropic",
    contextWindow: 200000,
  },
  {
    id: "openai/gpt-4o",
    name: "GPT-4o",
    provider: "OpenAI",
    contextWindow: 128000,
  },
  {
    id: "openai/gpt-4-turbo",
    name: "GPT-4 Turbo",
    provider: "OpenAI",
    contextWindow: 128000,
  },
  {
    id: "openai/gpt-3.5-turbo",
    name: "GPT-3.5 Turbo",
    provider: "OpenAI",
    contextWindow: 16385,
  },
  {
    id: "google/gemini-pro-1.5",
    name: "Gemini Pro 1.5",
    provider: "Google",
    contextWindow: 1000000,
  },
  {
    id: "google/gemini-flash-1.5",
    name: "Gemini Flash 1.5",
    provider: "Google",
    contextWindow: 1000000,
  },
  {
    id: "meta-llama/llama-3.1-70b-instruct",
    name: "Llama 3.1 70B",
    provider: "Meta",
    contextWindow: 128000,
  },
  {
    id: "meta-llama/llama-3.1-405b-instruct",
    name: "Llama 3.1 405B",
    provider: "Meta",
    contextWindow: 128000,
  },
  {
    id: "mistralai/mistral-large",
    name: "Mistral Large",
    provider: "Mistral AI",
    contextWindow: 128000,
  },
  {
    id: "anthropic/claude-3.5-sonnet:beta",
    name: "Claude 3.5 Sonnet (Beta)",
    provider: "Anthropic",
    contextWindow: 200000,
  },
];

interface SettingsState {
  selectedModel: string;
  customModels: ModelConfig[];
  availableModels: ModelConfig[];

  // Actions
  setSelectedModel: (modelId: string) => void;
  addCustomModel: (model: ModelConfig) => void;
  removeCustomModel: (modelId: string) => void;
  getAllModels: () => ModelConfig[];
}

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set, get) => ({
      selectedModel: "anthropic/claude-3.5-sonnet",
      customModels: [],
      availableModels: defaultModels,

      setSelectedModel: (modelId) => set({ selectedModel: modelId }),

      addCustomModel: (model) =>
        set((state) => ({
          customModels: [...state.customModels, { ...model, custom: true }],
        })),

      removeCustomModel: (modelId) =>
        set((state) => ({
          customModels: state.customModels.filter((m) => m.id !== modelId),
          selectedModel: state.selectedModel === modelId ? defaultModels[0].id : state.selectedModel,
        })),

      getAllModels: () => {
        const state = get();
        return [...state.availableModels, ...state.customModels];
      },
    }),
    {
      name: "gopenpal-settings",
    }
  )
);
