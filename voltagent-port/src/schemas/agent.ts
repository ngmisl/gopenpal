import { z } from "zod";

// Agent mood schemas
export const agentMoodSchema = z.enum([
  "joyful",
  "concerned",
  "proud",
  "nostalgic",
  "playful",
  "contemplative",
  "energized",
  "focused",
  "fired_up",
  "coaching",
  "closing",
  "attentive",
  "coordinating",
  "nurturing",
  "strategic",
  "ready",
  "urgent",
  "satisfied",
]);

export const agentNameSchema = z.enum(["hydrix", "serhant", "mio", "karen"]);

export const agentPersonalitySchema = z.enum([
  "caring_quirky",
  "confident_motivator",
  "warm_coordinator",
  "efficient_supportive",
]);

export const agentStateSchema = z.object({
  name: agentNameSchema,
  current_mood: agentMoodSchema,
  relationship_level: z.number().min(0).max(100),
  last_interaction: z.string().datetime().nullable(),
  backstory: z.string().optional(),
});

export const agentMetadataSchema = z.object({
  name: agentNameSchema,
  display_name: z.string(),
  personality: agentPersonalitySchema,
  icon: z.string(),
  role: z.string(),
  description: z.string(),
  available_moods: z.array(agentMoodSchema),
});

export type AgentMood = z.infer<typeof agentMoodSchema>;
export type AgentName = z.infer<typeof agentNameSchema>;
export type AgentPersonality = z.infer<typeof agentPersonalitySchema>;
export type AgentState = z.infer<typeof agentStateSchema>;
export type AgentMetadata = z.infer<typeof agentMetadataSchema>;
