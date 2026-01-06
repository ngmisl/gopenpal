import { createOpenAI } from "@ai-sdk/openai";

// OpenRouter configuration
export const openrouter = createOpenAI({
  apiKey: process.env.OPENROUTER_API_KEY || "",
  baseURL: "https://openrouter.ai/api/v1",
});

// Model configurations
export const models = {
  "claude-3.5-sonnet": openrouter("anthropic/claude-3.5-sonnet"),
  "claude-3-opus": openrouter("anthropic/claude-3-opus"),
  "gpt-4-turbo": openrouter("openai/gpt-4-turbo"),
  "gpt-4o": openrouter("openai/gpt-4o"),
  "gemini-pro": openrouter("google/gemini-pro"),
  "llama-3.1-70b": openrouter("meta-llama/llama-3.1-70b-instruct"),
};

export type ModelName = keyof typeof models;
