# GopenPal - Future Tasks & Features

This document tracks planned features, improvements, and technical tasks for GopenPal.

## Recently Completed ✅

### Agent System (2026-01-03)
- [x] **Karen Agent** - Executive assistant for reminders, memory, and task management
  - Full personality system with 5 moods
  - 20+ message library entries
  - TASK_TOOL integration with 7 commands
  - Integrated into startup scripts and TUI
- [x] **Date/Time Awareness** - All agents now receive current date/time in prompts
- [x] **TUI Agents Tab** - Visual dashboard showing all 4 agents with moods, relationships, last interaction
- [x] **Achievement Auto-Unlock** - Automatic achievement checking with desktop notifications
- [x] **Work Session Tracking Schema** - Database tables for FKD methodology, tasks, follow-ups, networking
- [x] **Multi-Agent Daemon** - All 4 agents (Hydrix, Serhant, Mio, Karen) running via start-day.sh
- [x] **Comprehensive Startup Scripts** - start-day.sh and stop-day.sh for full ecosystem management

## Recently Completed ✅

### Security & Organization (2026-01-03)
- [x] **World Folder Structure** - Organize all data files in dedicated world directory
  - Default database path: world/gopenpal.db
  - Updated CLI default values
  - All data contained within workspace
- [x] **Path Sandboxing Security** - Restrict agent file access to workspace only
  - Full security module with path validation
  - Canonicalization to prevent directory traversal
  - JSON-based settings configuration (configs/security.json)
  - Integrated into main application with validation
  - Comprehensive testing and documentation
  - Blocks absolute paths outside sandbox
  - Blocks parent directory escapes (../)
  - Resolves and validates symlinks

## High Priority

### Agent System Enhancements

- [ ] **Implement AGENT_DELEGATE_TOOL** (Mio coordination)
  - Add backend support for agent-to-agent delegation
  - Implement `[DELEGATE:HYDRIX:{request}]` command processing
  - Implement `[DELEGATE:SERHANT:{request}]` command processing
  - Implement `[DELEGATE:BOTH:{request}]` for multi-agent coordination
  - Add context passing between agents

- [ ] **Agent Voice/Personality Consistency**
  - Ensure each agent maintains their unique voice across all interactions
  - Add personality validation in message generation
  - Create agent-specific prompt templates

- [x] **Agent Daemon Enhancements**
  - Add support for multiple agents running simultaneously
  - Implement agent-specific intervals and probabilities
  - Add time-of-day awareness (e.g., Hydrix more active during work hours, Serhant in morning) (pending)
  - Add notification preferences (sound, urgency levels) (pending)

### AI Chat Improvements

- [ ] **Context-Aware Agent Selection**
  - Auto-route messages to appropriate agent based on content
  - Default to Mio for coordination and route to specialists
  - Add explicit agent selection in CLI/TUI (e.g., `@Hydrix how am I doing?`)

- [ ] **Multi-Agent Conversations**
  - Allow multiple agents to participate in single conversation
  - Mio orchestrates responses from Hydrix and Serhant
  - Show agent avatars/names in TUI chat interface

- [ ] **Enhanced Tool Integration**
  - Add more STATS_TOOL commands (monthly, yearly, comparisons)
  - Implement CRON_TOOL preset suggestions based on user patterns
  - Add undo/rollback for tool actions

### Statistics & Analytics

- [ ] **Advanced Pattern Detection**
  - Detect correlations (e.g., work sessions → water intake)
  - Identify anomalies and notify user
  - Predictive analytics (forecast water needs based on calendar)

- [ ] **Visualization**
  - Water intake history charts (ASCII art in CLI, ratatui graphs in TUI)
  - Hourly heatmaps showing drinking patterns
  - Streak visualizations with celebration animations

- [ ] **Goals & Tracking**
  - Customizable daily water goals
  - Progress toward goals with percentage indicators
  - Goal adjustment suggestions from AI based on patterns

## Medium Priority

### Agent World & Lore

- [ ] **Dynamic Lore System**
  - Implement lore unlock triggers based on achievements
  - Add lore discovery notifications
  - Create lore viewer in TUI (dedicated tab or modal)

- [ ] **Relationship Progression**
  - Visual relationship level indicators
  - Unlock new message types at relationship milestones
  - Special events/interactions at max relationship

- [x] **Achievement System Integration**
  - Trigger achievements based on user behavior
  - Achievement unlock notifications with agent celebrations
  - Achievement viewer in TUI (CLI completed)

- [ ] **More Agents**
  - **Tempo**: Time management and productivity rhythm specialist
  - **Zenith**: Energy optimization and peak performance coach
  - **Nexus**: Connection and networking specialist (works with Serhant)
  - Each new agent should have unique personality, lore, and specialized skills

### Work Session Tracking (Serhant Integration)

- [x] **FKD Time-Blocking System** (Database schema completed)
  - Track FINDER/KEEPER/DOER time blocks
  - AI-suggested time block schedules (pending)
  - Integration with calendar APIs (pending)

- [ ] **Sales Pipeline Tracking**
  - Log deals, follow-ups, and client interactions
  - Serhant provides coaching based on pipeline health
  - Three F's tracking (Follow Up, Follow Through, Follow Back)

- [ ] **Network Tracking**
  - Log daily networking interactions (meet 3-5 people/day)
  - Network growth visualization
  - Serhant celebrates milestones

### TUI Enhancements

- [x] **Agent Dashboard Tab**
  - View all agents, their moods, relationship levels
  - Recent interactions from each agent
  - Quick trigger for agent messages

- [ ] **Lore & Achievements Tab**
  - Browse unlocked lore entries by category
  - View achievement progress
  - Visual relationship progress bars

- [ ] **Enhanced Chat Interface**
  - Syntax highlighting for tool commands
  - Agent message history with avatars
  - Typing indicators, better UX

- [ ] **Notification Center**
  - In-app notification history from all agents
  - Mark as read/unread
  - Filter by agent or type

### Configuration & Customization

- [ ] **Agent Customization**
  - Adjust agent personality intensity
  - Set preferred agents for different contexts
  - Custom agent activation schedules

- [ ] **Theme Support**
  - Color themes for TUI
  - Agent-specific color schemes

- [ ] **Internationalization**
  - Multi-language support for all agents
  - Locale-aware date/time formatting

## Low Priority / Future Ideas

### Integration & Export

- [ ] **Health API Integration**
  - Fitbit, Apple Health, Google Fit for water logging
  - Export water data to health platforms
  - Import activity data for smarter recommendations

- [ ] **Calendar Integration**
  - Google Calendar, Outlook for work session tracking
  - Smart reminder scheduling based on calendar
  - Meeting-aware hydration reminders

- [ ] **Export & Reporting**
  - PDF/CSV export of statistics
  - Weekly/monthly reports via email
  - Shareable progress images for social media

### Web Dashboard

- [ ] **Web Interface**
  - Browser-based dashboard for analytics
  - Mobile-responsive design
  - Real-time sync with CLI/TUI

- [ ] **Public API**
  - REST API for third-party integrations
  - Webhooks for agent notifications
  - OAuth for secure access

### Advanced AI Features

- [ ] **Voice Input**
  - Speech-to-text for logging water intake
  - Voice commands for agents
  - Text-to-speech for agent notifications

- [ ] **Vision AI**
  - Photo logging of drinks (identify volume)
  - QR code scanning for predefined amounts
  - OCR for importing data

- [ ] **Proactive AI**
  - Agents learn user preferences over time
  - Context-aware suggestions (weather → hydration needs)
  - Personalized coaching strategies

### Gamification

- [ ] **Challenges**
  - Weekly hydration challenges
  - Compete with friends (leaderboards)
  - Special event challenges (summer hydration, work sprints)

- [ ] **Rewards & Badges**
  - Unlock special agent messages
  - Custom notification sounds
  - Visual rewards (themes, avatars)

## Technical Debt & Maintenance

### Code Quality

- [ ] **Comprehensive Testing**
  - Integration tests for agent system
  - Mock OpenRouter API for testing
  - Property-based testing for statistics calculations

- [ ] **Error Handling**
  - Better error messages for users
  - Graceful degradation when APIs fail
  - Retry logic for network operations

- [ ] **Performance**
  - Database query optimization
  - Caching for statistics calculations
  - Async/parallel processing where applicable

### Documentation

- [ ] **Developer Documentation**
  - Architecture overview with diagrams
  - Agent system implementation guide
  - Contributing guidelines with examples

- [ ] **User Documentation**
  - Interactive tutorial on first run
  - Help command with examples
  - Video tutorials for TUI features

- [ ] **Agent Development Guide**
  - How to add new agents
  - Message library best practices
  - Lore writing guidelines

### Infrastructure

- [ ] **Cloud Backup**
  - Optional cloud backup of database
  - Cross-device sync
  - Encrypted backups

- [ ] **Distribution**
  - Publish to crates.io
  - Docker container for easy deployment
  - Package for major Linux distros (apt, pacman, etc.)
  - Homebrew formula for macOS

## Ideas for Exploration

- Habit tracking beyond water (sleep, exercise, meditation)
- AI-generated personalized health plans
- Integration with smart water bottles (Bluetooth)
- Browser extension for work session tracking
- Slack/Discord bot for team challenges
- Machine learning for optimal reminder timing
- Seasonal agent variants (winter Hydrix, summer Serhant)
- Agent crossover events (Hydrix + Serhant special interactions)

---

**Note:** This is a living document. Add new tasks as ideas emerge, and mark completed items with dates.

Last updated: 2026-01-03
