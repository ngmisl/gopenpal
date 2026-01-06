# Model Selector Feature

## Overview
The frontend now includes a complete model selector that allows users to:
- Choose from 12+ pre-configured OpenRouter models
- Add custom models with any OpenRouter model ID
- Persist model selection across sessions
- See model details (provider, context window)

## Components Added

### 1. Settings Store (`src/store/settings.ts`)
- **Zustand store with persistence**
- Default models: Claude, GPT, Gemini, Llama, Mistral
- Custom model management (add/remove)
- Selected model state

```typescript
const { selectedModel, setSelectedModel, addCustomModel, getAllModels } = useSettingsStore();
```

### 2. Model Selector Component (`src/components/ModelSelector.tsx`)
- **Dropdown grouped by provider**
- **Add Custom Model modal** with form:
  - Model ID (from openrouter.ai/models)
  - Display Name
  - Provider
  - Context Window
- **Model info display**
- **Remove custom models**

### 3. Updated Components
- `App.tsx` - Added ModelSelector to sidebar
- `ChatInterface.tsx` - Sends `model` parameter in API requests
- `lib/openrouter.ts` - Dynamic model loading with `getModel(modelId)`

## Backend Integration (TODO)

The frontend sends the model ID to the server. The server needs to:

1. **Accept model parameter** in chat endpoint:
```typescript
POST /api/agents/:agentId/chat
Body: {
  messages: [...],
  model: "anthropic/claude-3.5-sonnet" // <-- Add this
}
```

2. **Create agents dynamically** with selected model:
```typescript
// Use agentFactory to create agents on-demand
import { createAgent } from "./lib/agentFactory";

app.post("/agents/:agentId/chat", async (c) => {
  const { messages, model } = await c.req.json();
  const agent = createAgent(agentId, model || defaultModel);
  const response = await agent.generateText({ messages });
  return c.json({ response: response.text });
});
```

3. **Agent Factory** (`src/lib/agentFactory.ts`) - Already created
   - Exports `createAgent(agentName, modelId)`
   - Creates agents with any OpenRouter model
   - Reuses agent instructions with different models

## Usage

### For Users:
1. **Select Model**: Click dropdown in sidebar, choose any model
2. **Add Custom**: Click "+ Add Custom", enter OpenRouter model ID
3. **Chat**: Selected model is automatically used for all conversations

### Adding New Default Models:
Edit `src/store/settings.ts`:
```typescript
export const defaultModels: ModelConfig[] = [
  // Add your model here
  {
    id: "provider/model-name",
    name: "Display Name",
    provider: "Provider Name",
    contextWindow: 128000,
  },
];
```

## Testing

```bash
# 1. Start the server
bun run server

# 2. Start the frontend
bun run dev

# 3. Test model selector:
- Select different models
- Add a custom model (e.g., "anthropic/claude-3-haiku")
- Send a message and verify it uses the selected model
```

## Dependencies

Added to package.json:
- `zustand` - Already included
- No additional dependencies needed (persist is part of zustand)

## Notes

- Model selection persists in localStorage via Zustand persist middleware
- Custom models are also persisted
- All OpenRouter models supported (500+ models)
- Model costs vary - check openrouter.ai/models for pricing

##Future Enhancements
- Display model pricing in selector
- Show current token usage
- Model performance stats
- Per-agent model overrides
- Streaming indicator for model responses
