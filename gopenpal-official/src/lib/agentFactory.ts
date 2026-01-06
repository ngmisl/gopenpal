import { Agent } from "@voltagent/core";
import { getModel } from "./openrouter";
import {
  logWaterTool,
  getWaterStatsTool,
  getWaterHistoryTool,
  setWaterGoalTool,
} from "../tools/water";
import { createTaskTool, listTasksTool, updateTaskTool, completeTaskTool } from "../tools/tasks";
import { getWaterAnalyticsTool, getTaskAnalyticsTool } from "../tools/stats";
import { createCronJobTool, listCronJobsTool, updateCronJobTool } from "../tools/cron";

// Agent instructions are kept in separate constants
import { HYDRIX_INSTRUCTIONS } from "../agents/hydrix";
import { SERHANT_INSTRUCTIONS } from "../agents/serhant";
import { MIO_INSTRUCTIONS } from "../agents/mio";
import { KAREN_INSTRUCTIONS } from "../agents/karen";

export function createHydrixAgent(modelId: string) {
  return new Agent({
    name: "hydrix",
    instructions: HYDRIX_INSTRUCTIONS,
    model: getModel(modelId),
    tools: [
      logWaterTool,
      getWaterStatsTool,
      getWaterHistoryTool,
      setWaterGoalTool,
      getWaterAnalyticsTool,
      createCronJobTool,
      listCronJobsTool,
      updateCronJobTool,
    ],
  });
}

export function createSerhantAgent(modelId: string) {
  return new Agent({
    name: "serhant",
    instructions: SERHANT_INSTRUCTIONS,
    model: getModel(modelId),
    tools: [getWaterAnalyticsTool, getTaskAnalyticsTool, createCronJobTool, listCronJobsTool, updateCronJobTool],
  });
}

export function createMioAgent(modelId: string) {
  return new Agent({
    name: "mio",
    instructions: MIO_INSTRUCTIONS,
    model: getModel(modelId),
    tools: [getWaterAnalyticsTool, getTaskAnalyticsTool, createCronJobTool, listCronJobsTool],
  });
}

export function createKarenAgent(modelId: string) {
  return new Agent({
    name: "karen",
    instructions: KAREN_INSTRUCTIONS,
    model: getModel(modelId),
    tools: [createTaskTool, listTasksTool, updateTaskTool, completeTaskTool, getTaskAnalyticsTool],
  });
}

// Create agent by name with specified model
export function createAgent(agentName: string, modelId: string) {
  switch (agentName) {
    case "hydrix":
      return createHydrixAgent(modelId);
    case "serhant":
      return createSerhantAgent(modelId);
    case "mio":
      return createMioAgent(modelId);
    case "karen":
      return createKarenAgent(modelId);
    default:
      throw new Error(`Unknown agent: ${agentName}`);
  }
}
