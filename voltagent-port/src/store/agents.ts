import { create } from "zustand";
import type { AgentState, AgentMetadata } from "../schemas/agent";

export const agentMetadata: Record<string, AgentMetadata> = {
  hydrix: {
    name: "hydrix",
    display_name: "Hydrix",
    personality: "caring_quirky",
    icon: "🌊",
    role: "Health & Hydration Specialist",
    description: "Ancient water spirit helping you optimize hydration and wellness",
    available_moods: ["joyful", "concerned", "proud", "nostalgic", "playful", "contemplative"],
  },
  serhant: {
    name: "serhant",
    display_name: "Serhant",
    personality: "confident_motivator",
    icon: "⚡",
    role: "Work & Productivity Coach",
    description: "Big Money Energy coach driving peak performance",
    available_moods: ["energized", "focused", "fired_up", "coaching", "closing"],
  },
  mio: {
    name: "mio",
    display_name: "Mio",
    personality: "warm_coordinator",
    icon: "🌸",
    role: "Coordination Specialist",
    description: "Personal concierge orchestrating harmony across all agents",
    available_moods: ["attentive", "coordinating", "nurturing", "proud", "strategic"],
  },
  karen: {
    name: "karen",
    display_name: "Karen",
    personality: "efficient_supportive",
    icon: "📋",
    role: "Task Management Expert",
    description: "Executive assistant turning chaos into clarity",
    available_moods: ["ready", "focused", "urgent", "satisfied", "strategic"],
  },
};

interface AgentsState {
  agentStates: Record<string, AgentState>;
  selectedAgent: string | null;
  isLoading: boolean;

  // Actions
  setAgentState: (name: string, state: AgentState) => void;
  setSelectedAgent: (name: string | null) => void;
  setIsLoading: (isLoading: boolean) => void;
  updateRelationship: (name: string, level: number) => void;
}

export const useAgentsStore = create<AgentsState>((set) => ({
  agentStates: {
    hydrix: {
      name: "hydrix",
      current_mood: "joyful",
      relationship_level: 0,
      last_interaction: null,
    },
    serhant: {
      name: "serhant",
      current_mood: "energized",
      relationship_level: 0,
      last_interaction: null,
    },
    mio: {
      name: "mio",
      current_mood: "attentive",
      relationship_level: 0,
      last_interaction: null,
    },
    karen: {
      name: "karen",
      current_mood: "ready",
      relationship_level: 0,
      last_interaction: null,
    },
  },
  selectedAgent: null,
  isLoading: false,

  setAgentState: (name, state) =>
    set((prev) => ({
      agentStates: {
        ...prev.agentStates,
        [name]: state,
      },
    })),

  setSelectedAgent: (name) => set({ selectedAgent: name }),

  setIsLoading: (isLoading) => set({ isLoading }),

  updateRelationship: (name, level) =>
    set((prev) => ({
      agentStates: {
        ...prev.agentStates,
        [name]: {
          ...prev.agentStates[name],
          relationship_level: level,
        },
      },
    })),
}));
