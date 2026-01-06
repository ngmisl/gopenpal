# GopenPal Web - Multi-Agent AI Assistant

A modern web-based multi-agent AI assistant for health tracking, productivity, and personal growth. Built with VoltAgent, OpenRouter, Zustand, and Zod.

## 🌟 Features

### 4 Unique AI Agents

- **Hydrix 🌊** - Health & Hydration Specialist
  - Ancient water spirit with 5000 years of wisdom
  - Tracks water intake, analyzes patterns
  - Sets goals and creates hydration reminders

- **Serhant ⚡** - Work & Productivity Coach
  - Big Money Energy methodology expert
  - Motivates and holds you accountable
  - Teaches productivity frameworks

- **Mio 🌸** - Coordination & Integration Specialist
  - Personal concierge orchestrating harmony
  - Connects insights across health, work, and tasks
  - Provides holistic support

- **Karen 📋** - Task Management Expert
  - Executive assistant for getting things done
  - Breaks down goals into actionable steps
  - Tracks progress and identifies blockers

### Core Capabilities

- 💬 **Real-time Chat** - Stream responses from AI agents
- 💧 **Water Tracking** - Log and monitor daily hydration
- ✅ **Task Management** - Create, track, and complete tasks
- 📊 **Analytics** - Detailed insights across all domains
- ⏰ **Smart Reminders** - Cron-based scheduling
- 🎨 **Modern UI** - Beautiful, responsive interface

## 🚀 Tech Stack

- **Frontend**: Vite + React + TypeScript + TailwindCSS
- **State Management**: Zustand
- **Validation**: Zod
- **AI Framework**: VoltAgent
- **LLM Provider**: OpenRouter (Claude 3.5 Sonnet)
- **Database**: LibSQL (SQLite)
- **Server**: Hono
- **Runtime**: Bun

## 📦 Installation

### Prerequisites

- [Bun](https://bun.sh/) installed
- OpenRouter API key ([get one here](https://openrouter.ai))

### Setup

1. **Clone and navigate to the project:**
   ```bash
   cd examples/gopenpal-web
   ```

2. **Install dependencies:**
   ```bash
   bun install
   ```

3. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env and add your OPENROUTER_API_KEY
   ```

4. **Run the development servers:**
   ```bash
   # Run both frontend and backend
   bun run dev:all

   # Or run separately:
   bun run dev      # Frontend (Vite) on port 3000
   bun run server   # Backend (VoltAgent) on port 3141
   ```

5. **Open your browser:**
   ```
   http://localhost:3000
   ```

## 🏗️ Project Structure

```
gopenpal-web/
├── src/
│   ├── agents/           # 4 AI agent definitions
│   │   ├── hydrix.ts
│   │   ├── serhant.ts
│   │   ├── mio.ts
│   │   └── karen.ts
│   ├── tools/            # Zod-validated tools
│   │   ├── water.ts
│   │   ├── tasks.ts
│   │   ├── stats.ts
│   │   └── cron.ts
│   ├── schemas/          # Zod schemas
│   │   ├── agent.ts
│   │   ├── water.ts
│   │   ├── task.ts
│   │   ├── stats.ts
│   │   └── cron.ts
│   ├── store/            # Zustand stores
│   │   ├── chat.ts
│   │   ├── water.ts
│   │   ├── tasks.ts
│   │   └── agents.ts
│   ├── components/       # React components
│   │   ├── AgentSelector.tsx
│   │   ├── ChatInterface.tsx
│   │   ├── WaterTracker.tsx
│   │   └── TaskList.tsx
│   ├── lib/              # Utilities
│   │   ├── db.ts         # Database layer
│   │   └── openrouter.ts # OpenRouter config
│   ├── server/           # VoltAgent server
│   │   └── index.ts
│   ├── App.tsx           # Main app component
│   ├── main.tsx          # App entry point
│   └── index.css         # Global styles
├── index.html            # HTML entry point
├── vite.config.ts        # Vite configuration
├── tailwind.config.ts    # Tailwind configuration
└── package.json
```

## 🛠️ Development

### Build for Production

```bash
bun run build
```

### Preview Production Build

```bash
bun run preview
```

### Type Check

```bash
bun run type-check
```

### Linting

```bash
bun run lint
bun run lint:fix
```

## 🔧 Configuration

### Adding New Tools

1. Create a tool in `src/tools/`:
   ```typescript
   import { createTool } from "@voltagent/core";
   import { z } from "zod";

   export const myTool = createTool({
     name: "my_tool",
     description: "What this tool does",
     parameters: z.object({
       param: z.string().describe("Parameter description"),
     }),
     execute: async ({ param }) => {
       // Tool logic
       return { result: "success" };
     },
   });
   ```

2. Add the tool to an agent in `src/agents/`:
   ```typescript
   import { myTool } from "../tools/myTool";

   export const myAgent = new Agent({
     name: "my-agent",
     instructions: "...",
     model: models["claude-3.5-sonnet"],
     tools: [myTool], // Add your tool here
   });
   ```

### Using Different LLM Models

Edit `src/lib/openrouter.ts` to change models:

```typescript
export const models = {
  "claude-3.5-sonnet": openrouter("anthropic/claude-3.5-sonnet"),
  "gpt-4-turbo": openrouter("openai/gpt-4-turbo"),
  "gpt-4o": openrouter("openai/gpt-4o"),
  // Add more models...
};
```

## 📊 Database

The app uses LibSQL (SQLite) for local storage:
- Database file: `.voltagent/gopenpal.db`
- Automatically initialized on first run
- Stores: water intake, tasks, agent states, cron jobs

### Database Schema

- `water_intake` - Water logging with timestamps
- `water_goals` - Daily water goals
- `tasks` - Task management
- `cron_jobs` - Scheduled reminders
- `agents` - Agent state and relationships

## 🔐 Environment Variables

```env
OPENROUTER_API_KEY=sk-or-v1-...
VOLTAGENT_API_URL=http://localhost:3141
DATABASE_URL=file:./.voltagent/gopenpal.db
```

## 🎯 Usage Examples

### Chat with an Agent

1. Select an agent (Hydrix, Serhant, Mio, or Karen)
2. Type your message in the chat interface
3. Agent will respond with personality and can use tools

### Log Water

- Click quick amount buttons (250ml, 500ml, etc.)
- Or enter custom amount and click "Log"
- View progress toward daily goal

### Manage Tasks

- Add tasks with title and priority
- Check off completed tasks
- View pending and completed lists

## 🚢 Deployment

### Vercel (Recommended for Frontend)

```bash
bun run build
# Deploy dist/ folder to Vercel
```

### Backend Deployment

The VoltAgent server can be deployed to:
- Railway
- Fly.io
- Any Node.js/Bun hosting platform

Update `VOLTAGENT_API_URL` in your frontend environment to point to your deployed backend.

## 🤝 Contributing

This is part of the VoltAgent examples collection. To contribute:

1. Fork the VoltAgent repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📝 License

MIT License - see the VoltAgent repository for details

## 🙏 Acknowledgments

- **VoltAgent** - Production-ready AI agent framework
- **OpenRouter** - Unified LLM API
- **Original GopenPal** - Rust CLI version that inspired this web port
- **Ryan Serhant** - Big Money Energy methodology

## 📞 Support

- [VoltAgent Documentation](https://voltagent.ai/docs)
- [VoltAgent GitHub](https://github.com/VoltAgent/voltagent)
- [OpenRouter Docs](https://openrouter.ai/docs)

---

**Built with ❤️ using VoltAgent, OpenRouter, Zustand, and Zod**
