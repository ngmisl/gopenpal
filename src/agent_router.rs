//! Agent routing system for context-aware agent selection.
//!
//! This module analyzes message content to automatically route requests to the
//! most appropriate agent, or allows explicit agent selection via @mentions.

use serde::{Deserialize, Serialize};

/// Result of agent routing analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingDecision {
    /// The selected agent name
    pub agent: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f32,
    /// Whether this was an explicit mention vs auto-routing
    pub explicit: bool,
    /// The message with agent mention removed (if applicable)
    pub cleaned_message: String,
}

/// Agent router for analyzing message content and selecting appropriate agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRouter {
    /// Keywords associated with each agent
    agent_keywords: AgentKeywords,
}

/// Keywords and patterns for each agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentKeywords {
    hydrix: Vec<String>,
    serhant: Vec<String>,
    karen: Vec<String>,
    mio: Vec<String>,
}

impl Default for AgentRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRouter {
    /// Create a new agent router with default keyword mappings.
    pub fn new() -> Self {
        Self {
            agent_keywords: AgentKeywords {
                hydrix: vec![
                    // Direct water/hydration terms
                    "water".to_string(),
                    "hydration".to_string(),
                    "hydrate".to_string(),
                    "drink".to_string(),
                    "thirsty".to_string(),
                    "dehydrated".to_string(),
                    // Health-related
                    "health".to_string(),
                    "energy".to_string(),
                    "tired".to_string(),
                    "fatigue".to_string(),
                    "headache".to_string(),
                    // Tracking/stats
                    "intake".to_string(),
                    "drank".to_string(),
                    "drinking".to_string(),
                    "ml".to_string(),
                    "oz".to_string(),
                    "glasses".to_string(),
                    "bottle".to_string(),
                ],
                serhant: vec![
                    // Work/business terms
                    "work".to_string(),
                    "sales".to_string(),
                    "client".to_string(),
                    "deal".to_string(),
                    "meeting".to_string(),
                    "presentation".to_string(),
                    "pitch".to_string(),
                    "negotiate".to_string(),
                    "close".to_string(),
                    "closing".to_string(),
                    // Motivation/mindset
                    "motivation".to_string(),
                    "motivated".to_string(),
                    "energy".to_string(),
                    "bme".to_string(),
                    "big money energy".to_string(),
                    "crush".to_string(),
                    "framework".to_string(),
                    // Productivity
                    "productive".to_string(),
                    "productivity".to_string(),
                    "goals".to_string(),
                    "target".to_string(),
                    "achievement".to_string(),
                ],
                karen: vec![
                    // Task management
                    "task".to_string(),
                    "tasks".to_string(),
                    "todo".to_string(),
                    "to-do".to_string(),
                    "reminder".to_string(),
                    "remind".to_string(),
                    "remember".to_string(),
                    "note".to_string(),
                    "notes".to_string(),
                    // Organization
                    "organize".to_string(),
                    "schedule".to_string(),
                    "calendar".to_string(),
                    "plan".to_string(),
                    "planning".to_string(),
                    "prioritize".to_string(),
                    "priority".to_string(),
                    // Context/memory
                    "context".to_string(),
                    "forgot".to_string(),
                    "what was".to_string(),
                    "working on".to_string(),
                    "track".to_string(),
                    "tracking".to_string(),
                ],
                mio: vec![
                    // Coordination terms
                    "coordinate".to_string(),
                    "help".to_string(),
                    "assist".to_string(),
                    "support".to_string(),
                    // General queries
                    "how".to_string(),
                    "what".to_string(),
                    "status".to_string(),
                    "overview".to_string(),
                    "summary".to_string(),
                    // Multi-domain
                    "everything".to_string(),
                    "overall".to_string(),
                    "together".to_string(),
                ],
            },
        }
    }

    /// Route a message to the appropriate agent.
    ///
    /// This method first checks for explicit @mentions, then falls back to
    /// content-based routing if no mention is found.
    pub fn route(&self, message: &str) -> RoutingDecision {
        // First, check for explicit agent mentions
        if let Some(decision) = self.parse_mention(message) {
            return decision;
        }

        // Fall back to content-based routing
        self.analyze_content(message)
    }

    /// Parse explicit agent mentions like @Hydrix, @Mio, @Serhant, @Karen.
    fn parse_mention(&self, message: &str) -> Option<RoutingDecision> {
        let message_lower = message.to_lowercase();

        // Check for each agent mention
        let agents = [
            ("@hydrix", "Hydrix"),
            ("@mio", "Mio"),
            ("@serhant", "Serhant"),
            ("@karen", "Karen"),
        ];

        for (mention, agent_name) in &agents {
            if message_lower.contains(mention) {
                // Remove the mention from the message
                let cleaned = message
                    .replace(&format!("@{}", agent_name.to_lowercase()), "")
                    .replace(&format!("@{}", agent_name), "")
                    .trim()
                    .to_string();

                return Some(RoutingDecision {
                    agent: agent_name.to_string(),
                    confidence: 1.0,
                    explicit: true,
                    cleaned_message: cleaned,
                });
            }
        }

        None
    }

    /// Analyze message content to determine the best agent.
    fn analyze_content(&self, message: &str) -> RoutingDecision {
        let message_lower = message.to_lowercase();

        // Calculate keyword match scores for each agent
        let hydrix_score = self.calculate_score(&message_lower, &self.agent_keywords.hydrix);
        let serhant_score = self.calculate_score(&message_lower, &self.agent_keywords.serhant);
        let karen_score = self.calculate_score(&message_lower, &self.agent_keywords.karen);
        let mio_score = self.calculate_score(&message_lower, &self.agent_keywords.mio);

        // Find the highest scoring agent
        let scores = [
            ("Hydrix", hydrix_score),
            ("Serhant", serhant_score),
            ("Karen", karen_score),
            ("Mio", mio_score),
        ];

        let (agent, score) = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        // If no clear winner or low confidence, default to Mio (coordinator)
        let (final_agent, confidence) = if *score < 1 {
            ("Mio", 0.5) // Low confidence, Mio coordinates
        } else {
            (agent.as_ref(), (*score as f32 / 10.0).min(1.0))
        };

        RoutingDecision {
            agent: final_agent.to_string(),
            confidence,
            explicit: false,
            cleaned_message: message.to_string(),
        }
    }

    /// Calculate keyword match score for a message.
    fn calculate_score(&self, message: &str, keywords: &[String]) -> usize {
        keywords
            .iter()
            .filter(|keyword| message.contains(keyword.as_str()))
            .count()
    }

    /// Add custom keywords for an agent.
    pub fn add_keywords(&mut self, agent: &str, keywords: Vec<String>) {
        match agent {
            "Hydrix" => self.agent_keywords.hydrix.extend(keywords),
            "Serhant" => self.agent_keywords.serhant.extend(keywords),
            "Karen" => self.agent_keywords.karen.extend(keywords),
            "Mio" => self.agent_keywords.mio.extend(keywords),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_mention_hydrix() {
        let router = AgentRouter::new();
        let decision = router.route("@Hydrix how much water should I drink?");

        assert_eq!(decision.agent, "Hydrix");
        assert_eq!(decision.confidence, 1.0);
        assert!(decision.explicit);
        assert_eq!(decision.cleaned_message, "how much water should I drink?");
    }

    #[test]
    fn test_explicit_mention_serhant() {
        let router = AgentRouter::new();
        let decision = router.route("@Serhant give me some motivation!");

        assert_eq!(decision.agent, "Serhant");
        assert_eq!(decision.confidence, 1.0);
        assert!(decision.explicit);
    }

    #[test]
    fn test_content_routing_water() {
        let router = AgentRouter::new();
        let decision = router.route("I need to drink more water today");

        assert_eq!(decision.agent, "Hydrix");
        assert!(!decision.explicit);
        assert!(decision.confidence > 0.0);
    }

    #[test]
    fn test_content_routing_tasks() {
        let router = AgentRouter::new();
        let decision = router.route("Can you remind me to finish my tasks?");

        assert_eq!(decision.agent, "Karen");
        assert!(!decision.explicit);
    }

    #[test]
    fn test_content_routing_sales() {
        let router = AgentRouter::new();
        let decision = router.route("How do I close this deal with the client?");

        assert_eq!(decision.agent, "Serhant");
        assert!(!decision.explicit);
    }

    #[test]
    fn test_default_to_mio() {
        let router = AgentRouter::new();
        let decision = router.route("Hello! How are you?");

        assert_eq!(decision.agent, "Mio");
        assert!(!decision.explicit);
    }

    #[test]
    fn test_add_custom_keywords() {
        let mut router = AgentRouter::new();
        router.add_keywords("Hydrix", vec!["h2o".to_string()]);

        let decision = router.route("I need more h2o");
        assert_eq!(decision.agent, "Hydrix");
    }
}
