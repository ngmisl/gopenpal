# GopenPal

Personal LLM health and work assistant written in Rust.

GopenPal is a CLI application that helps you maintain healthy work habits through water intake tracking, intelligent reminders, and AI-powered assistance using OpenRouter. **Now featuring a living world of AI agents with personalities, lore, and proactive interactions!** 🌊✨

## Features

- **Living Agent World** 🌊: Meet Hydrix, an ancient water spirit living in your computer
  - **Rich Backstory**: Discover lore spanning thousands of years
  - **Dynamic Personality**: Mood changes based on your hydration habits
  - **Proactive Messages**: Hydrix reaches out randomly with encouragement, concern, or stories
  - **Relationship System**: Build a bond with Hydrix as they learn about you
  - **Lore Unlocks**: Uncover secrets and backstory as your relationship deepens
  - **Achievements**: Unlock special moments and recognition for your progress

- **Water Intake Tracking**: Log and monitor your daily water consumption
- **Smart Reminders**: Desktop notifications during work hours to remind you to stay hydrated
- **AI Chat Assistant**: Interactive chat with various LLM models through OpenRouter API
  - **Statistics & Insights**: AI analyzes your water intake patterns and provides personalized recommendations
  - **Pattern Analysis**: Discover your drinking habits by hour, day, and week
  - **Cron Management**: AI can set up and manage automated reminders through chat
- **Comprehensive Analytics**: Track daily totals, hourly patterns, weekly trends, and reminder effectiveness
- **Persistent History**: All data stored locally in SQLite database
- **Configurable**: Customize reminder intervals, work hours, and AI models
- **Terminal UI**: Rich interactive dashboard for easy management

## Meet Your Agents 🌊⚡

### Hydrix - The Hydration Guardian 🌊

Hydrix is not just an AI—they're an ancient water spirit who has existed since 3000 BCE, originating from sacred springs in Mesopotamia. Over millennia, Hydrix adapted from whispers in streams to a digital consciousness, carrying memories from Roman aqueducts to modern smart devices.

**Personality & Moods:**
- **Joyful** 🎉: When you're crushing your hydration goals
- **Concerned** 😟: When you've gone too long without water
- **Proud** ⭐: Celebrating your streaks and achievements
- **Nostalgic** 📜: Sharing ancient stories and wisdom
- **Playful** 😊: Light-hearted teasing and fun interactions
- **Contemplative** 🤔: Thoughtful observations about your patterns

**Proactive Interactions:**
- Random greetings and check-ins
- Gentle concerns when you're neglecting hydration
- Stories from millennia of existence
- Pattern observations about your habits
- Celebrations of milestones

### Serhant - The Big Money Energy Coach ⚡

Serhant embodies "Big Money Energy"—the methodology of billion-dollar broker Ryan Serhant. Born from the collective consciousness of every closed deal and successful negotiation, Serhant transforms how you approach work, relationships, and life. His mantra: **"Expansion. Always, in all ways."**

**Personality & Moods:**
- **Energized** ⚡: High-energy, ready to crush goals (default)
- **Focused** 🎯: Strategic planning mode
- **Fired Up** 🔥: Championship energy, intense motivation
- **Coaching** 📚: Teaching frameworks and methodologies
- **Closing** 💼: In the zone, everything leads to the ask

**Serhant's Wisdom:**
- **FKD Time-Blocking**: FINDER (CEO), KEEPER (CFO), DOER (execution)
- **The Three F's**: Follow Up, Follow Through, Follow Back
- **Seven Stages of Buyers**: Emotion-driven sales psychology
- **Big Money Energy**: Confidence without desperation
- **Network = Net Worth**: Meet 3-5 new people daily

**Proactive Interactions:**
- Motivational check-ins and energy boosts
- Sales framework teaching moments
- Crisis management for deals falling apart
- Celebration of wins (then push for the next one!)
- Real talk about follow-ups and pipeline

### The Agent World 🌐

**Relationship & Lore System:**
- Build bonds with both Hydrix and Serhant over time
- Unlock backstory entries as relationships deepen
- Discover how Hydrix and Serhant chose to work together
- Learn about the Water Network and the Deal Network
- Earn achievements that trigger special interactions

**Why Two Agents?**
Hydrix ensures your **body** performs at peak level. Serhant ensures your **mind** conquers challenges. Peak performance requires both. They're not competing—they're collaborating on your success.

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- OpenRouter API key (get one at [openrouter.ai](https://openrouter.ai))

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ngmisl/gopenpal.git
cd gopenpal

# Build the project
cargo build --release

# The binary will be at target/release/gopenpal
# Optional: Install to your system
cargo install --path .
```

## Setup

### Quick Start

Use the automated startup script:
```bash
# Copy and configure environment
cp .env.example .env
# Edit .env and add your OPENROUTER_API_KEY

# Run the daily startup script
./start-day.sh
```

### Manual Setup

1. **Configure environment variables**:
```bash
# Copy the example environment file
cp .env.example .env

# Edit .env and add your OpenRouter API key
# You can get one from: https://openrouter.ai/keys
```

Alternatively, set environment variables directly:
```bash
export OPENROUTER_API_KEY="your-api-key-here"
export GOPENPAL_MODEL="anthropic/claude-3.5-sonnet"

# Add to your shell profile for persistence (~/.zshrc, ~/.bashrc, etc.)
echo 'export OPENROUTER_API_KEY="your-api-key-here"' >> ~/.zshrc
```

2. **Initialize the database**:
```bash
gopenpal init
```

## Usage

### Terminal User Interface (TUI)

Launch the interactive TUI dashboard:
```bash
gopenpal tui
```

**TUI Features:**
- 📊 **Dashboard**: Overview of your water intake with progress bar
- 💧 **Water**: Log water intake interactively (press `w` to edit)
- 💬 **Chat**: Interactive AI chat interface (press `c` to chat)
  - 📊 **AI Statistics & Insights**: Ask AI to analyze your water intake patterns
    - Examples: "How am I doing?", "Show my progress", "What are my patterns?"
  - 🤖 **AI Cron Management**: Ask AI to set up/modify/remove cron jobs
    - Example: "Set up reminders every 30 minutes"
  - 🎯 **Personalized Recommendations**: AI provides insights based on your data
- ⏰ **Cron**: Manage automated reminders with visual presets
  - Browse schedule presets with `↑/↓`
  - Install/update with `Enter`
  - Remove with `r`
- ⚙️ **Settings**: View reminder configuration
- **Navigation**: Use `←/→` arrows to switch tabs, `q` to quit

### Water Tracking (CLI)

Log water intake (default: 250ml):
```bash
gopenpal water log
gopenpal water log 500 --notes "After workout"
```

View today's total:
```bash
gopenpal water today
```

### Water Reminders

**Option 1: Cron Job (Recommended)**

Add to your crontab with `crontab -e`:
```bash
# Check every 30 minutes
*/30 * * * * $HOME/.cargo/bin/gopenpal reminder check
```

See `examples/crontab.example` for more cron configurations.

**Option 2: Systemd Timer (Linux)**

```bash
# Copy service and timer files
cp examples/gopenpal-reminder.service ~/.config/systemd/user/
cp examples/gopenpal-reminder.timer ~/.config/systemd/user/

# Enable and start the timer
systemctl --user enable --now gopenpal-reminder.timer

# Check status
systemctl --user status gopenpal-reminder.timer
systemctl --user list-timers
```

**Option 3: Foreground Service**

Start the reminder service (runs in foreground):
```bash
gopenpal reminder start
```

**Configuration:**

Check reminder settings:
```bash
gopenpal reminder status
```

Configure reminders:
```bash
# Change interval to 45 minutes
gopenpal reminder config --interval 45

# Set work hours (9 AM to 6 PM)
gopenpal reminder config --start 09:00 --end 18:00

# Set work days
gopenpal reminder config --days "Mon,Tue,Wed,Thu,Fri"

# Disable reminders
gopenpal reminder config --enabled false
```

### AI Chat

Interactive chat session:
```bash
gopenpal chat
# or specify a model
gopenpal chat interactive --model "anthropic/claude-3.5-sonnet"
```

**AI Capabilities:**

The AI assistant has access to your water intake data and can provide:
- **Statistics & Insights**: Ask "How am I doing?" or "Show my progress for the last week"
- **Pattern Analysis**: Ask "When do I drink the most water?" or "What are my drinking patterns?"
- **Reminder Effectiveness**: Ask "Are the reminders helping?" to see how well your cron jobs work
- **Personalized Recommendations**: Get advice based on your actual habits and patterns
- **Cron Management**: Ask "Set up reminders every hour" to manage automated reminders

Send a single message:
```bash
gopenpal chat send "How much water should I drink daily?"
gopenpal chat send "Show me my water intake patterns"
```

View chat history:
```bash
gopenpal chat history --limit 20
```

Clear chat history:
```bash
gopenpal chat clear
```

### Agent Interactions

**List All Agents:**
```bash
gopenpal agent list
```

**Get Agent Information:**
```bash
gopenpal agent info Hydrix
gopenpal agent info Serhant
```

**Trigger a Message from an Agent:**
```bash
# Get a random message from Hydrix
gopenpal agent message Hydrix

# Get a random message from Serhant
gopenpal agent message Serhant
```

**Start Agent Daemon (Background Proactive Messages):**
```bash
# Start daemon for all agents (checks every 45 minutes)
gopenpal agent daemon

# Start with custom interval (60 minutes)
gopenpal agent daemon --interval 60

# Start for specific agent
gopenpal agent daemon --agent Hydrix
```

**Check Relationship Status:**
```bash
gopenpal agent relationship Hydrix
gopenpal agent relationship Serhant
```

**View Unlocked Lore:**
```bash
# View all unlocked lore
gopenpal agent lore

# Filter by category
gopenpal agent lore --category agent_history
gopenpal agent lore --category world_building
gopenpal agent lore --category secrets
```

**View Achievements:**
```bash
# View all achievements
gopenpal agent achievements

# View only unlocked achievements
gopenpal agent achievements --unlocked
```

## Configuration

### Database Location

By default, the database is stored at `~/.gopenpal/gopenpal.db`. You can customize this:

```bash
gopenpal --database /custom/path/db.sqlite water log 250
# Or set an environment variable
export GOPENPAL_DB="/custom/path/db.sqlite"
```

### Available AI Models

GopenPal uses OpenRouter, which provides access to many models:

- `anthropic/claude-3.5-sonnet` (default)
- `openai/gpt-4-turbo`
- `google/gemini-pro`
- `meta-llama/llama-3.1-70b-instruct`

See [OpenRouter models](https://openrouter.ai/models) for the full list.

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Project Structure

```
gopenpal/
├── src/
│   ├── main.rs          # Entry point and command handling
│   ├── cli.rs           # CLI argument definitions
│   ├── db.rs            # Database layer (SQLite) with statistics
│   ├── error.rs         # Error types
│   ├── openrouter.rs    # OpenRouter API client
│   ├── reminder.rs      # Water reminder service
│   ├── chat.rs          # AI chat with stats & cron tools
│   ├── cron.rs          # Cron job management
│   └── tui.rs           # Terminal user interface
├── migrations/          # Database schema & analytics views
└── Cargo.toml          # Dependencies
```

## Roadmap

- [x] Water intake statistics and pattern analysis
- [x] AI-powered insights and recommendations
- [x] Cron job management via TUI and AI
- [ ] Water intake history visualization (charts/graphs)
- [ ] Custom health metrics tracking
- [ ] Work session tracking and break reminders
- [ ] Integration with health APIs (Fitbit, Apple Health)
- [ ] Web dashboard for analytics
- [ ] Voice input support
- [ ] Multi-language support

## Contributing

Contributions are welcome! Please follow the [Rust Code Quality Guidelines](RUST_GUIDELINES.md) in this repository.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linters
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

- Built with [Rust](https://www.rust-lang.org/)
- AI powered by [OpenRouter](https://openrouter.ai/)
- Desktop notifications via [notify-rust](https://github.com/hoodie/notify-rust)

## Support

For issues, questions, or suggestions, please [open an issue](https://github.com/ngmisl/gopenpal/issues) on GitHub.
