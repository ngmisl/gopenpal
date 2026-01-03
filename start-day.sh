#!/usr/bin/env bash
#
# GopenPal Daily Startup Script
# This script initializes your daily workflow with water tracking and AI assistance
#
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${SCRIPT_DIR}/.env"

echo -e "${BLUE}=== GopenPal Daily Startup ===${NC}\n"

# Load environment variables if .env exists
if [ -f "$ENV_FILE" ]; then
    echo -e "${GREEN}Loading environment from .env${NC}"
    set -a
    source "$ENV_FILE"
    set +a
else
    echo -e "${YELLOW}Warning: .env file not found${NC}"
    echo -e "${YELLOW}Copy .env.example to .env and configure your API key${NC}\n"
fi

# Check if gopenpal is installed
if ! command -v gopenpal &> /dev/null; then
    echo -e "${YELLOW}gopenpal not found in PATH${NC}"
    echo "Building from source..."
    cargo build --release
    GOPENPAL_BIN="${SCRIPT_DIR}/target/release/gopenpal"
else
    GOPENPAL_BIN="gopenpal"
fi

# Initialize database if needed
echo -e "\n${GREEN}Initializing database...${NC}"
$GOPENPAL_BIN init

# Log morning water (optional)
echo -e "\n${BLUE}Would you like to log your morning water? (y/n)${NC}"
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    echo "Enter amount in ml (default: 250):"
    read -r amount
    amount=${amount:-250}
    $GOPENPAL_BIN water log "$amount" --notes "Morning hydration"
    echo -e "${GREEN}Logged $amount ml of water${NC}"
fi

# Show today's stats
echo -e "\n${BLUE}Today's Water Intake:${NC}"
$GOPENPAL_BIN water today

# Check reminder settings
echo -e "\n${BLUE}Reminder Settings:${NC}"
$GOPENPAL_BIN reminder status

# Reminder setup check
echo -e "\n${YELLOW}Make sure reminders are set up!${NC}"
echo "Options:"
echo "  1. Cron job: Add to crontab with 'crontab -e'"
echo "     */30 * * * * $GOPENPAL_BIN reminder check"
echo "  2. Systemd timer: Install timer units from examples/"
echo "  3. Foreground service: Run 'gopenpal reminder start' in a separate terminal"

# Optional: Start TUI
echo -e "\n${BLUE}Launch TUI? (y/n)${NC}"
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}Starting TUI...${NC}"
    echo "Press 'q' to quit TUI"
    sleep 1
    $GOPENPAL_BIN tui
else
    echo -e "\n${GREEN}Setup complete!${NC}"
    echo "Run 'gopenpal tui' anytime to launch the dashboard"
    echo "Run 'gopenpal --help' to see all commands"
fi
