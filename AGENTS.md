# Agent System Architecture and Guidelines

This document describes the multi-agent system architecture in GopenPal, including personality management, routing, coordination, and code quality requirements.

## Overview

GopenPal implements a multi-agent AI assistant system where specialized agents handle different domains (health, work, tasks, coordination). Each agent maintains a unique personality while having access to specific tools and capabilities.

## Agent Roster

### Mio - The Personal Concierge
- **Role:** Coordination and routing
- **Personality Type:** `warm_coordinator`
- **Primary Function:** Request routing, multi-agent coordination, context awareness
- **Tools:** STATS_TOOL, CRON_TOOL, AGENT_DELEGATE_TOOL, GREP_TOOL
- **Access Level:** All agents and systems

### Hydrix - The Hydration Guardian
- **Role:** Health specialist (hydration tracking)
- **Personality Type:** `caring_quirky`
- **Primary Function:** Hydration tracking, health monitoring, energy optimization
- **Tools:** STATS_TOOL, CRON_TOOL
- **Access Level:** Health metrics
- **Backstory:** Ancient water spirit from 3000 BCE

### Serhant - The Big Money Energy Coach
- **Role:** Work specialist (sales, motivation)
- **Personality Type:** `confident_motivator`
- **Primary Function:** Sales coaching, motivation, framework teaching, mindset training
- **Tools:** STATS_TOOL, CRON_TOOL
- **Access Level:** Work metrics
- **Frameworks:** FKD Time-Blocking, The Three F's, Big Money Energy

### Karen - The Executive Assistant
- **Role:** Task specialist
- **Personality Type:** `efficient_supportive`
- **Primary Function:** Task management, reminders, memory, context tracking
- **Tools:** TASK_TOOL, STATS_TOOL, GREP_TOOL
- **Access Level:** All tasks and productivity data

## Architecture Components

### 1. Personality System (`src/personality.rs`)

The personality system ensures each agent maintains a consistent, unique voice across all interactions.

#### Core Data Structures

```rust
pub struct PersonalityProfile {
    pub personality_type: String,
    pub traits: Vec<String>,
    pub voice_characteristics: VoiceCharacteristics,
    pub mood_variations: Vec<MoodBehavior>,
    pub signature_phrases: Vec<String>,
}

pub struct VoiceCharacteristics {
    pub tone: String,
    pub sentence_style: String,
    pub punctuation_style: String,
    pub vocabulary: String,
    pub emoji_usage: String,
}

pub struct MoodBehavior {
    pub mood: String,
    pub voice_shift: String,
    pub example_greeting: String,
}
```

#### Available Personalities

1. **Warm Coordinator** (Mio)
   - Tone: Warm, professional, and reassuring
   - Style: Balanced sentences that flow naturally
   - Vocabulary: Professional yet approachable, uses "we" and "together"
   - Emoji: Minimal (✨, 🌟, 💫)

2. **Caring Quirky** (Hydrix)
   - Tone: Warm, enthusiastic, with ancient wisdom
   - Style: Mix of short exclamations and flowing descriptions
   - Vocabulary: Water metaphors, ancient references, "dear one"
   - Emoji: Frequent water-themed (💧, 🌊, ✨)

3. **Confident Motivator** (Serhant)
   - Tone: High energy, confident, Big Money Energy
   - Style: Direct and punchy, short impactful sentences
   - Vocabulary: Business terms, sports metaphors, "championship"
   - Emoji: Strategic for impact (💪, 🔥, 💰, 🏆)

4. **Efficient Supportive** (Karen)
   - Tone: Professional, efficient, executive assistant polish
   - Style: Clear and organized, lists and structure
   - Vocabulary: Productivity terms, "prioritize", "optimize"
   - Emoji: Minimal and professional (✓, 📋, 🎯)

#### Usage

```rust
// Get a personality profile
let profile = PersonalityProfile::get_profile("warm_coordinator")?;

// Build an agent-specific system prompt
let prompt = profile.build_agent_prompt(
    "Mio",
    "Your Personal Concierge",
    "Born from connections between agents...",
    "attentive"
);
```

### 2. Agent Router (`src/agent_router.rs`)

The router analyzes message content to automatically select the most appropriate agent or respects explicit @mentions.

#### Routing Decision

```rust
pub struct RoutingDecision {
    pub agent: String,           // Selected agent name
    pub confidence: f32,          // Confidence level (0.0 to 1.0)
    pub explicit: bool,           // True if @mention, false if auto-routed
    pub cleaned_message: String,  // Message with @mention removed
}
```

#### Routing Methods

**Explicit Routing (Priority 1):**
- User includes @mention: `@Hydrix how much water should I drink?`
- Confidence: Always 1.0
- Message is cleaned of the @mention before processing

**Content-Based Routing (Priority 2):**
- Analyzes keywords in message content
- Calculates scores for each agent based on keyword matches
- Selects highest-scoring agent
- Falls back to Mio (coordinator) if no clear winner

#### Keyword Mapping

```rust
// Example keyword sets
hydrix: ["water", "hydration", "drink", "thirsty", "health", "energy"]
serhant: ["work", "sales", "client", "deal", "motivation", "bme"]
karen: ["task", "todo", "reminder", "organize", "schedule", "plan"]
mio: ["coordinate", "help", "assist", "status", "overview"]
```

#### Usage

```rust
let router = AgentRouter::new();
let decision = router.route("@Hydrix I'm feeling tired");

println!("Agent: {}", decision.agent);      // "Hydrix"
println!("Explicit: {}", decision.explicit);  // true
println!("Message: {}", decision.cleaned_message);  // "I'm feeling tired"
```

### 3. Multi-Agent Coordination

Mio can orchestrate multiple agents for complex, multi-domain requests using the `MULTI_AGENT` command.

#### Command Format

```
[MULTI_AGENT:Agent1,Agent2,Agent3:request]
```

#### Example

```
User: "Help me have a productive work session"
Mio: [MULTI_AGENT:Hydrix,Serhant,Karen:Help user have productive work session]

Response:
💧 Hydrix says: Stay hydrated! Your energy levels are optimal at 85%...
⚡ Serhant says: Let's bring that Big Money Energy! Time to crush...
📋 Karen says: I've organized your high-priority tasks...
✨ Mio coordinates: Together we've set you up for peak performance!
```

#### Processing Flow

1. Mio identifies need for multiple specialists
2. Sends `[MULTI_AGENT:...]` command
3. Each agent generates response in their personality
4. Mio synthesizes responses with visual indicators
5. Returns coordinated, multi-perspective guidance

### 4. Tool System (`configs/tools.json`)

Tools enable agents to take action, not just give advice.

#### Tool Categories

1. **Analytics:** STATS_TOOL (access: Mio, Hydrix, Serhant)
2. **Automation:** CRON_TOOL (access: Mio, Hydrix, Serhant)
3. **Productivity:** TASK_TOOL (access: Mio, Karen)
4. **Coordination:** AGENT_DELEGATE_TOOL (access: Mio only)
5. **File Operations:** GREP_TOOL (access: Mio, Karen)

#### Tool Usage Philosophy

**Priority Order:**
1. STATS_TOOL - Always check data first
2. TASK_TOOL (Karen) - Manage tasks and productivity
3. AGENT_DELEGATE_TOOL (Mio only) - Route to specialist if needed
4. CRON_TOOL - Automate based on insights

**Best Practices:**
- Use tools proactively, not just when asked
- Combine tools for comprehensive solutions
- Always explain what tools are doing
- Follow up after tool execution

### 5. Agent Configuration (`configs/agents.json`)

Each agent is configured with:
- Name, title, and role
- Personality type reference
- Description and backstory
- Skills and capabilities
- Tool access permissions
- Mood definitions with triggers
- Access level boundaries

## Code Quality Requirements

### Strict Requirements

**MUST comply with all requirements or code will be rejected:**

1. **Zero Dead Code Policy**
   - **NEVER** use `#[allow(dead_code)]` or any suppression attributes
   - Either use the code properly in the application or remove it entirely
   - All struct fields must be accessed or removed
   - All methods must be called or removed
   - All imports must be used or removed

2. **Zero Warnings Policy**
   - `cargo build` **MUST** complete with zero warnings
   - `cargo test` **MUST** complete with zero warnings
   - `cargo clippy` **MUST** pass with zero warnings
   - Use `-D warnings` flag in CI, not `#![deny(warnings)]` in source

3. **Zero Errors Policy**
   - All tests **MUST** pass
   - Code **MUST** compile successfully
   - No runtime panics in normal operation paths

### Implementation Guidelines

**When adding new agent functionality:**

1. **Define personality first** in `src/personality.rs`
   - Create complete PersonalityProfile with all voice characteristics
   - Define mood variations with examples
   - Add signature phrases

2. **Add routing keywords** in `src/agent_router.rs`
   - Update keyword mappings for content-based routing
   - Add agent to explicit mention list
   - Write tests for routing behavior

3. **Configure agent** in `configs/agents.json`
   - Complete all required fields
   - Define moods with appropriate triggers
   - Specify tool access permissions
   - Set access level boundaries

4. **Update tool access** in `configs/tools.json`
   - Add agent to tool access lists as appropriate
   - Document agent-specific usage patterns
   - Update best practices if needed

5. **Write comprehensive tests**
   - Test personality prompt generation
   - Test routing (explicit and content-based)
   - Test tool access permissions
   - Test multi-agent coordination if applicable

### Testing Requirements

**All agent code must include:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_profile_exists() {
        let profile = PersonalityProfile::get_profile("agent_type");
        assert!(profile.is_some());
    }

    #[test]
    fn test_routing_explicit_mention() {
        let router = AgentRouter::new();
        let decision = router.route("@AgentName help me");
        assert_eq!(decision.agent, "AgentName");
        assert!(decision.explicit);
    }

    #[test]
    fn test_routing_content_based() {
        let router = AgentRouter::new();
        let decision = router.route("keyword1 keyword2");
        assert_eq!(decision.agent, "ExpectedAgent");
        assert!(!decision.explicit);
    }
}
```

## Agent Personality Consistency

### Core Principle

**Agents MUST maintain their unique personality in ALL interactions.**

This includes:
- Using tools (maintain voice in explanations)
- Reporting errors (stay in character)
- Processing commands (personality-aware responses)
- Coordinating with other agents (maintain distinct voice)

### Example: Personality Consistency Across Contexts

**Bad (Breaking Character):**
```
Hydrix: [STATS:SUMMARY:7]
Response: Your daily average is 1800ml. Total: 12600ml.
```

**Good (Maintaining Character):**
```
Hydrix: [STATS:SUMMARY:7]
Response: You're absolutely crushing it! 💧 Over the past week, you've blessed
your body with 12,600ml of water - that's 1,800ml per day on average! Your
hydration game is strong, dear one! ✨
```

### Mood-Based Variations

Agents **MUST** adjust their voice based on current mood while maintaining core personality:

**Hydrix in "joyful" mood:**
```
"You're absolutely crushing it! 💧✨ That's what I love to see!"
```

**Hydrix in "concerned" mood:**
```
"I've noticed it's been a while, dear one... Your body needs you. Let's get
some water flowing, shall we? 💧"
```

**Serhant in "energized" mood:**
```
"Let's GO! What are we crushing today? 💪 Time to bring that Big Money Energy!"
```

**Serhant in "coaching" mood:**
```
"Here's the framework that changed everything for me: The Three F's -
Follow Up, Follow Through, Follow Back. Let me break this down..."
```

## Security and Sandboxing

### Path Validation

All file operations **MUST** be validated through the security sandbox (`src/security.rs`):

```rust
// CORRECT: Validate before file access
let security = SecurityConfig::load_from_file(&config_path)?;
security.validate_path(&user_provided_path)?;
let content = std::fs::read_to_string(&user_provided_path)?;

// WRONG: Direct file access without validation
let content = std::fs::read_to_string(&user_provided_path)?;  // NEVER DO THIS
```

### Tool Access Restrictions

- Each agent has specific tool permissions defined in `configs/tools.json`
- **MUST** enforce access restrictions at runtime
- **NEVER** allow agents to access tools outside their permission set
- Mio has special coordination privileges (AGENT_DELEGATE_TOOL)

### Data Access Boundaries

- Hydrix: `health_metrics` only
- Serhant: `work_metrics` only
- Karen: `all_tasks` and productivity data
- Mio: `all_agents` (coordination access)

## Performance Considerations

### Agent Selection

- Explicit @mentions: O(1) lookup, instant routing
- Content-based routing: O(n*k) where n = agents, k = keywords per agent
- Current implementation is optimized for small agent count (4 agents)
- No caching needed at current scale

### Personality System

- Profiles are created on-demand via `get_profile()`
- Consider caching if agent switching becomes frequent
- Prompt generation is string concatenation (fast)

### Multi-Agent Coordination

- Sequential agent calls (not parallelized)
- Each agent gets independent LLM call
- Response synthesis happens after all agents respond
- Future optimization: Parallel agent calls with `tokio::join!`

## Before Committing Agent Changes

- [ ] All personality profiles are complete with all required fields
- [ ] Routing keywords are comprehensive and non-overlapping where possible
- [ ] Agent configuration in `configs/agents.json` is complete
- [ ] Tool access permissions are correctly specified
- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo build`)
- [ ] No dead code (no `#[allow(dead_code)]` attributes)
- [ ] Personality consistency is maintained in all contexts
- [ ] Security boundaries are respected
- [ ] Doc comments are present for all public items
- [ ] Examples are provided for complex agent behaviors

## Future Enhancements

### Planned Features

1. **Dynamic Mood Selection**
   - Analyze conversation context to select appropriate mood
   - Transition moods based on user emotional state
   - Mood history tracking

2. **Learning from Interactions**
   - Track which routing decisions were successful
   - Adjust keyword weights based on user corrections
   - Personalize agent selection over time

3. **Agent Collaboration Patterns**
   - Pre-defined collaboration templates for common scenarios
   - Automatic agent assembly based on request complexity
   - Handoff protocols between agents

4. **Extended Personality Dimensions**
   - Energy level variations
   - Formality adjustments based on context
   - Cultural sensitivity adaptations

---

**Remember:** Agents are not just functions - they are distinct personalities that build relationships with users over time. Maintain that magic in every interaction.
