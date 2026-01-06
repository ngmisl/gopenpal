# GopenPal Web

> **Multi-Agent AI Assistant for Health, Productivity & Personal Growth**

A modern web application powered by VoltAgent, featuring four specialized AI agents that help you stay hydrated, productive, and balanced. Built with TypeScript, React, and OpenRouter for enterprise-grade reliability with full code control.

[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](https://www.typescriptlang.org/)
[![VoltAgent](https://img.shields.io/badge/VoltAgent-2.0-purple)](https://voltagent.ai)
[![OpenRouter](https://img.shields.io/badge/OpenRouter-500%2B%20Models-green)](https://openrouter.ai)
[![React](https://img.shields.io/badge/React-18-cyan)](https://react.dev)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

![GopenPal Screenshot](https://via.placeholder.com/800x400/667eea/ffffff?text=GopenPal+Screenshot)

---

## ✨ Features

### 🤖 Four Specialized AI Agents

Each agent has a unique personality, specialized tools, and domain expertise:

| Agent | Role | Personality | Specialization |
|-------|------|-------------|----------------|
| **Hydrix 🌊** | Health & Hydration | Caring + Quirky | Water tracking, wellness analytics |
| **Serhant ⚡** | Work & Productivity | Confident + Motivator | BME coaching, performance optimization |
| **Mio 🌸** | Coordination | Warm + Coordinator | Multi-agent orchestration, holistic support |
| **Karen 📋** | Task Management | Efficient + Supportive | GTD methodology, execution support |

### 🎯 Core Capabilities

- **💬 Real-time AI Chat** - Stream responses from specialized agents
- **💧 Water Tracking** - Log intake, set goals, analyze patterns
- **✅ Task Management** - Create, prioritize, track, complete
- **📊 Analytics** - Detailed insights across health and productivity
- **⏰ Smart Reminders** - Cron-based scheduling for hydration and tasks
- **🎨 Modern UI** - Beautiful, responsive interface with TailwindCSS
- **🔧 Model Selector** - Choose from 500+ OpenRouter models or add custom
- **💾 Persistence** - LibSQL database with automatic synchronization

### 🚀 Tech Stack

**Frontend**
- [Vite](https://vite.dev) - Lightning-fast build tool
- [React 18](https://react.dev) - UI framework
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [TailwindCSS](https://tailwindcss.com/) - Utility-first styling
- [Zustand](https://zustand-demo.pmnd.rs/) - State management

**Backend**
- [VoltAgent](https://voltagent.ai) - AI agent framework
- [Hono](https://hono.dev/) - Lightweight web framework
- [OpenRouter](https://openrouter.ai) - Unified LLM API (500+ models)
- [LibSQL](https://turso.tech/libsql) - SQLite-based database
- [Zod](https://zod.dev) - Schema validation

**Runtime**
- [Bun](https://bun.sh) - Fast all-in-one JavaScript runtime

---

## 🚀 Quick Start

### Prerequisites

- [Bun](https://bun.sh/) v1.0+ installed
- [OpenRouter API key](https://openrouter.ai/keys) (free tier available)

### Installation

```bash
# Clone the repository
git clone https://github.com/ngmisl/gopenpal.git
cd gopenpal/voltagent-port

# Install dependencies
bun install

# Set up environment variables
cp .env.example .env
# Edit .env and add your OPENROUTER_API_KEY
```

### Running Locally

```bash
# Start both frontend and backend
bun run dev:all

# Or run separately:
bun run dev      # Frontend on http://localhost:3000
bun run server   # Backend on http://localhost:3141
```

### Building for Production

```bash
# Type-check and build
bun run build

# Preview production build
bun run preview
```

---

## 📚 Documentation

- **[AGENTS.md](AGENTS.md)** - Complete agent guide (personalities, tools, examples)
- **[MODEL_SELECTOR.md](MODEL_SELECTOR.md)** - Model selector feature documentation
- **[VoltAgent Docs](https://voltagent.ai/docs)** - Framework documentation
- **[OpenRouter Models](https://openrouter.ai/models)** - Available LLM models

---

## 🏗️ Project Structure

```
gopenpal-web/
├── src/
│   ├── agents/              # AI agent definitions
│   │   ├── hydrix.ts        # Health & Hydration specialist
│   │   ├── serhant.ts       # Work & Productivity coach
│   │   ├── mio.ts           # Coordination specialist
│   │   └── karen.ts         # Task management expert
│   │
│   ├── tools/               # Zod-validated agent tools
│   │   ├── water.ts         # Water tracking operations
│   │   ├── tasks.ts         # Task CRUD operations
│   │   ├── stats.ts         # Analytics and insights
│   │   └── cron.ts          # Reminder scheduling
│   │
│   ├── schemas/             # Zod validation schemas
│   │   ├── agent.ts         # Agent state and metadata
│   │   ├── water.ts         # Water intake schemas
│   │   ├── task.ts          # Task management schemas
│   │   ├── stats.ts         # Analytics schemas
│   │   └── cron.ts          # Cron job schemas
│   │
│   ├── store/               # Zustand state management
│   │   ├── chat.ts          # Chat messages and streaming
│   │   ├── water.ts         # Water tracking state
│   │   ├── tasks.ts         # Task management state
│   │   ├── agents.ts        # Agent selection and metadata
│   │   └── settings.ts      # Model selection and preferences
│   │
│   ├── components/          # React UI components
│   │   ├── AgentSelector.tsx    # Agent selection UI
│   │   ├── ModelSelector.tsx    # LLM model picker
│   │   ├── ChatInterface.tsx    # Chat UI with streaming
│   │   ├── WaterTracker.tsx     # Water logging interface
│   │   └── TaskList.tsx         # Task management UI
│   │
│   ├── lib/                 # Utilities and helpers
│   │   ├── db.ts            # LibSQL database layer
│   │   ├── openrouter.ts    # OpenRouter configuration
│   │   └── agentFactory.ts  # Dynamic agent creation
│   │
│   ├── server/              # VoltAgent server
│   │   └── index.ts         # Hono server with VoltAgent
│   │
│   ├── App.tsx              # Main application component
│   ├── main.tsx             # Application entry point
│   └── index.css            # Global styles
│
├── public/                  # Static assets
├── index.html              # HTML entry point
├── vite.config.ts          # Vite configuration
├── tailwind.config.ts      # Tailwind configuration
├── tsconfig.json           # TypeScript configuration
├── package.json            # Dependencies and scripts
├── .env.example            # Environment variable template
├── AGENTS.md               # Agent documentation
├── MODEL_SELECTOR.md       # Model selector guide
└── README.md               # This file
```

---

## 💡 Usage

### Chat with Agents

1. **Select an Agent** - Click on Hydrix, Serhant, Mio, or Karen
2. **Choose a Model** - Pick from Claude, GPT, Gemini, Llama, or add custom
3. **Start Chatting** - Type your message and get specialized assistance

### Log Water Intake

- Use quick buttons (250ml, 500ml, 750ml, 1000ml)
- Or enter a custom amount
- View progress toward daily goal
- Check analytics for patterns

### Manage Tasks

- Add tasks with title and priority
- Check off completed items
- Filter by status or priority
- Get productivity insights from Karen

### Schedule Reminders

- Ask agents to create reminders
- Set hydration check-ins with Hydrix
- Schedule accountability sessions with Serhant
- Coordinate multi-domain reminders with Mio

---

## 🔧 Configuration

### Environment Variables

```bash
# Required
OPENROUTER_API_KEY=sk-or-v1-...

# Optional
VOLTAGENT_API_URL=http://localhost:3141
DATABASE_URL=file:./.voltagent/gopenpal.db
```

### Model Selection

The app supports any OpenRouter model. Popular choices:

- **Claude 3.5 Sonnet** - Best for complex reasoning (default)
- **GPT-4o** - Fast and capable
- **Gemini Pro 1.5** - Large context window (1M tokens)
- **Llama 3.1 405B** - Open-source powerhouse

Add custom models via the UI or edit `src/store/settings.ts`.

### Database

Default: `.voltagent/gopenpal.db` (LibSQL/SQLite)

Tables:
- `water_intake` - Water logging with timestamps
- `water_goals` - Daily hydration goals
- `tasks` - Task management with priorities
- `cron_jobs` - Scheduled reminders
- `agents` - Agent state and relationships

---

## 🛠️ Development

### Adding a New Agent

1. Create agent file in `src/agents/myagent.ts`
2. Define instructions with personality
3. Add to agent factory in `src/lib/agentFactory.ts`
4. Update metadata in `src/store/agents.ts`
5. Register in server `src/server/index.ts`

See [AGENTS.md](AGENTS.md) for detailed guide.

### Creating Custom Tools

```typescript
// src/tools/mytool.ts
import { createTool } from "@voltagent/core";
import { z } from "zod";

export const myTool = createTool({
  name: "my_tool",
  description: "Description for the LLM",
  parameters: z.object({
    param: z.string().describe("Parameter description"),
  }),
  execute: async ({ param }) => {
    // Your logic here
    return { result: "success" };
  },
});
```

### Type Checking

```bash
bun run type-check
```

### Linting

```bash
bun run lint       # Check
bun run lint:fix   # Fix issues
```

---

## 🚢 Deployment

### Frontend (Vercel Recommended)

```bash
# Build
bun run build

# Deploy dist/ to Vercel
vercel deploy
```

Set environment variables in Vercel dashboard.

### Backend (Railway / Fly.io)

Deploy `src/server/index.ts` to any Node.js/Bun hosting:

```bash
# Railway
railway up

# Fly.io
fly deploy
```

Update `VOLTAGENT_API_URL` in frontend to point to deployed backend.

### Full Stack (Docker)

```dockerfile
FROM oven/bun:1

WORKDIR /app
COPY package.json bun.lockb ./
RUN bun install

COPY . .
RUN bun run build

CMD ["bun", "run", "server"]
```

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow existing code style (Biome)
- Add Zod schemas for all data structures
- Write comprehensive agent instructions
- Test agents thoroughly
- Update documentation

---

## 📖 Learn More

### VoltAgent Framework

VoltAgent is an open-source TypeScript framework for building production-ready AI agents:

- **Full Code Control** - No black boxes, complete transparency
- **Type Safety** - TypeScript-first with Zod validation
- **Observability** - Built-in traces, logs, and evals
- **Multi-Agent** - Coordinate multiple specialized agents
- **Enterprise Ready** - Production-grade with proper error handling

Learn more: https://voltagent.ai

### Inspiration

GopenPal was inspired by:
- **Original GopenPal** - Rust CLI version with TUI
- **Ryan Serhant** - Big Money Energy methodology
- **GTD** - Getting Things Done by David Allen
- **Atomic Habits** - James Clear's habit-forming principles

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **[VoltAgent](https://voltagent.ai)** - AI agent framework
- **[OpenRouter](https://openrouter.ai)** - Unified LLM API
- **[Bun](https://bun.sh)** - Fast JavaScript runtime
- **[Anthropic](https://anthropic.com)** - Claude models
- **[Vercel](https://vercel.com)** - AI SDK and hosting

---

## 📞 Support

- **Documentation**: See [AGENTS.md](AGENTS.md) and [MODEL_SELECTOR.md](MODEL_SELECTOR.md)
- **Issues**: https://github.com/ngmisl/gopenpal/issues
- **VoltAgent Docs**: https://voltagent.ai/docs
- **OpenRouter Help**: https://openrouter.ai/docs

---

## 🎯 Roadmap

- [ ] Voice interaction with ElevenLabs
- [ ] RAG for personalized insights
- [ ] Mobile app (React Native)
- [ ] Browser extension
- [ ] Slack/Discord integration
- [ ] Team collaboration features
- [ ] Advanced analytics dashboard
- [ ] Custom agent marketplace

---

**Built with ❤️ using VoltAgent, OpenRouter, Zustand, and Zod**

*Ship enterprise-grade multi-agent systems end-to-end with full code control.*

---

## 🚀 Get Started Now

```bash
git clone https://github.com/ngmisl/gopenpal.git
cd gopenpal/voltagent-port
bun install
cp .env.example .env
# Add your OPENROUTER_API_KEY
bun run dev:all
```

Open http://localhost:3000 and start chatting with your AI assistants! 🎉
