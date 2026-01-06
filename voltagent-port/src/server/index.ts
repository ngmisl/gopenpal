import { VoltAgent } from "@voltagent/core";
import { honoServer } from "@voltagent/server-hono";
import { createLogger } from "@voltagent/logger";
import { hydrixAgent } from "../agents/hydrix";
import { serhantAgent } from "../agents/serhant";
import { mioAgent } from "../agents/mio";
import { karenAgent } from "../agents/karen";
import { db } from "../lib/db";

// Initialize database
await db.initialize();

// Create logger
const logger = createLogger({
  level: "info",
  name: "gopenpal-server",
});

// Create VoltAgent server instance
const voltAgent = new VoltAgent({
  agents: {
    hydrix: hydrixAgent,
    serhant: serhantAgent,
    mio: mioAgent,
    karen: karenAgent,
  },
  server: honoServer({
    port: 3141,
    cors: {
      origin: ["http://localhost:3000", "http://localhost:3141"],
      credentials: true,
    },
  }),
  logger,
});

console.log("🌊 Hydrix - Health & Hydration Specialist");
console.log("⚡ Serhant - Work & Productivity Coach");
console.log("🌸 Mio - Coordination & Integration Specialist");
console.log("📋 Karen - Task Management & Execution Expert");
console.log("\n✨ GopenPal VoltAgent Server running on http://localhost:3141");
console.log("📊 VoltOps Dashboard: https://voltagent.ai/dashboard");

// Graceful shutdown
process.on("SIGTERM", () => {
  console.log("\n👋 Shutting down gracefully...");
  process.exit(0);
});
