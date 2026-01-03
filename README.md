# GopenPal

Personal LLM health and work assistant written in Rust.

GopenPal is a CLI application that helps you maintain healthy work habits through water intake tracking, intelligent reminders, and AI-powered assistance using OpenRouter.

## Features

- **Water Intake Tracking**: Log and monitor your daily water consumption
- **Smart Reminders**: Desktop notifications during work hours to remind you to stay hydrated
- **AI Chat Assistant**: Interactive chat with various LLM models through OpenRouter API
- **Persistent History**: All data stored locally in SQLite database
- **Configurable**: Customize reminder intervals, work hours, and AI models

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
  - 🤖 **AI Cron Management**: Ask AI to set up/modify/remove cron jobs
  - Example: "Set up reminders every 30 minutes"
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

Send a single message:
```bash
gopenpal chat send "How much water should I drink daily?"
```

View chat history:
```bash
gopenpal chat history --limit 20
```

Clear chat history:
```bash
gopenpal chat clear
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
│   ├── db.rs            # Database layer (SQLite)
│   ├── error.rs         # Error types
│   ├── openrouter.rs    # OpenRouter API client
│   ├── reminder.rs      # Water reminder service
│   └── chat.rs          # AI chat functionality
├── migrations/          # Database schema
└── Cargo.toml          # Dependencies
```

## Roadmap

- [ ] Water intake history visualization
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
