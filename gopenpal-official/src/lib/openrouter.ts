import { createOpenAI } from "@ai-sdk/openai";

// OpenRouter configuration
export const openrouter = createOpenAI({
  apiKey: process.env.OPENROUTER_API_KEY || "",
  baseURL: "https://openrouter.ai/api/v1",
});

// Get model instance by ID (supports any OpenRouter model)
export function getModel(modelId: string) {
  return openrouter(modelId);
}

// Default model
export const defaultModel = "anthropic/claude-3.5-sonnet";
