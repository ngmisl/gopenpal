# GopenPal - Official VoltAgent Setup

This is the **official VoltAgent project structure** created following VoltAgent best practices.

## ✅ What Changed

### Before (Manual Structure)
- Custom directory layout in `voltagent-port/`
- Mixed concerns (agents, web UI, server all in src/)
- Non-standard configuration

### After (Official VoltAgent Structure)
- Proper separation: `src/server/`, `src/web/`, `src/agents/`, `src/tools/`
- Official VoltAgent conventions
- Clean TypeScript configuration
- Standard build tooling

## 📁 Official Project Structure

```
gopenpal-official/
├── src/
│   ├── server/          # VoltAgent server (Hono)
│   │   └── index.ts     # Server entry point
│   │
│   ├── web/             # React frontend
│   │   ├── components/  # UI components
│   │   ├── store/       # Zustand state
│   │   ├── App.tsx      # Main app
│   │   ├── main.tsx     # Entry point
│   │   └── index.css    # Styles
│   │
│   ├── agents/          # AI agent definitions
│   │   ├── hydrix.ts    # Health specialist
│   │   ├── serhant.ts   # Work coach
│   │   ├── mio.ts       # Coordinator
│   │   └── karen.ts     # Task expert
│   │
│   ├── tools/           # Zod-validated tools
│   │   ├── water.ts
│   │   ├── tasks.ts
│   │   ├── stats.ts
│   │   └── cron.ts
│   │
│   ├── schemas/         # Zod schemas
│   │   ├── agent.ts
│   │   ├── water.ts
│   │   ├── task.ts
│   │   ├── stats.ts
│   │   └── cron.ts
│   │
│   └── lib/             # Utilities
│       ├── db.ts
│       ├── openrouter.ts
│       └── agentFactory.ts
│
├── package.json         # Official VoltAgent deps
├── tsconfig.json        # TypeScript config
├── vite.config.ts       # Vite config
├── tailwind.config.ts   # Tailwind config
├── index.html           # HTML entry
├── .env.example         # Environment template
├── README.md            # Documentation
├── AGENTS.md            # Agent guide
└── MODEL_SELECTOR.md    # Model selector docs
```

## 🚀 Installation & Setup

### Prerequisites
- Node.js 20+ or Bun 1.0+
- OpenRouter API key

### Option 1: With Proper npm Registry

```bash
# Install dependencies
bun install  # or npm install

# Set up environment
cp .env.example .env
# Add your OPENROUTER_API_KEY

# Run development servers
bun run dev          # Both frontend & backend
# OR separately:
bun run dev:server   # Backend on :3141
bun run dev:web      # Frontend on :3000

# Build for production
bun run build
```

### Option 2: With VoltAgent Monorepo

If you have npm registry issues, install from the VoltAgent monorepo:

```bash
# Copy to VoltAgent examples
cp -r gopenpal-official /path/to/voltagent/examples/gopenpal

# Install from monorepo root
cd /path/to/voltagent
pnpm install

# Run from monorepo
cd examples/gopenpal
pnpm run dev
```

## 📋 Scripts

```json
{
  "dev": "Run both frontend and backend",
  "dev:server": "Run VoltAgent server only",
  "dev:web": "Run Vite frontend only",
  "build": "Build both server and frontend",
  "build:server": "Build server only",
  "preview": "Preview production build",
  "start": "Start production server",
  "volt": "VoltAgent CLI commands",
  "type-check": "TypeScript validation",
  "lint": "Biome linting",
  "lint:fix": "Auto-fix lint issues"
}
```

## 🔧 Key Differences from Manual Setup

1. **Separation of Concerns**
   - Server code in `src/server/`
   - Web code in `src/web/`
   - Shared code in `src/agents/`, `src/tools/`, `src/lib/`

2. **Official Dependencies**
   - Uses `@voltagent/cli` for tooling
   - Proper VoltAgent package versions
   - Official server setup with `honoServer()`

3. **Standard Build Process**
   - TypeScript compilation for server
   - Vite bundling for frontend
   - Proper production builds

4. **Development Workflow**
   - `tsx watch` for server (auto-reload)
   - `vite` for frontend (HMR)
   - Concurrent development servers

## 🎯 Benefits of Official Structure

✅ **Maintainability** - Standard VoltAgent conventions
✅ **Scalability** - Clear separation of concerns
✅ **Type Safety** - Proper TypeScript configuration
✅ **Developer Experience** - Official tooling and CLI
✅ **Documentation** - Follows VoltAgent patterns
✅ **Community** - Easier to get help and contribute

## 📚 Next Steps

1. **Install Dependencies** - Run `bun install`
2. **Configure Environment** - Add your API keys to `.env`
3. **Start Development** - Run `bun run dev`
4. **Read Documentation** - See README.md and AGENTS.md
5. **Customize Agents** - Modify agents in `src/agents/`
6. **Add Features** - Create new tools in `src/tools/`

## 🔄 Migration from Old Structure

If you have the old `voltagent-port/` structure:

```bash
# The new structure already has all the code
# Just need to install and run

cd gopenpal-official
bun install
cp .env.example .env
# Add OPENROUTER_API_KEY
bun run dev
```

All agents, tools, and features are already ported!

## 💡 Tips

- **Use `volt` CLI** - Explore VoltAgent commands
- **Check Examples** - See `/path/to/voltagent/examples/`
- **Read VoltAgent Docs** - https://voltagent.ai/docs
- **Join Community** - Get help and share feedback

---

**This is now a proper VoltAgent project! 🎉**

Built with official VoltAgent structure and best practices.
