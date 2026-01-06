import { create } from "zustand";
import { z } from "zod";

// Message schema
const messageSchema = z.object({
  id: z.string(),
  role: z.enum(["user", "assistant"]),
  content: z.string(),
  timestamp: z.string().datetime(),
  agent: z.enum(["hydrix", "serhant", "mio", "karen"]).optional(),
});

export type Message = z.infer<typeof messageSchema>;

interface ChatState {
  messages: Message[];
  currentAgent: "hydrix" | "serhant" | "mio" | "karen" | null;
  isStreaming: boolean;
  streamingMessage: string;

  // Actions
  addMessage: (message: Omit<Message, "id" | "timestamp">) => void;
  setCurrentAgent: (agent: "hydrix" | "serhant" | "mio" | "karen" | null) => void;
  setIsStreaming: (isStreaming: boolean) => void;
  setStreamingMessage: (message: string) => void;
  clearStreamingMessage: () => void;
  clearMessages: () => void;
}

export const useChatStore = create<ChatState>((set) => ({
  messages: [],
  currentAgent: null,
  isStreaming: false,
  streamingMessage: "",

  addMessage: (message) =>
    set((state) => ({
      messages: [
        ...state.messages,
        {
          ...message,
          id: crypto.randomUUID(),
          timestamp: new Date().toISOString(),
        },
      ],
    })),

  setCurrentAgent: (agent) => set({ currentAgent: agent }),

  setIsStreaming: (isStreaming) => set({ isStreaming }),

  setStreamingMessage: (message) => set({ streamingMessage: message }),

  clearStreamingMessage: () => set({ streamingMessage: "" }),

  clearMessages: () => set({ messages: [] }),
}));
