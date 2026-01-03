#!/usr/bin/env bash
#
# GopenPal Comprehensive Startup Script
# This script starts all GopenPal services:
# - Agent daemons (Hydrix, Serhant, Mio)
# - Cron jobs for reminders
# - TUI interface
#
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${SCRIPT_DIR}/.env"
PID_DIR="${SCRIPT_DIR}/.gopenpal_pids"
LOG_DIR="${SCRIPT_DIR}/.gopenpal_logs"

# Create directories for PIDs and logs
mkdir -p "$PID_DIR" "$LOG_DIR"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   GopenPal - Daily Startup System     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}\n"

# Load environment variables if .env exists
if [ -f "$ENV_FILE" ]; then
    echo -e "${GREEN}✓ Loading environment from .env${NC}"
    set -a
    source "$ENV_FILE"
    set +a
else
    echo -e "${YELLOW}⚠ Warning: .env file not found${NC}"
    echo -e "${YELLOW}  Copy .env.example to .env and configure your API key${NC}"
    echo -e "${YELLOW}  Some features may not work without API configuration${NC}\n"
fi

# Check if gopenpal is installed or build it
if ! command -v gopenpal &> /dev/null; then
    echo -e "${YELLOW}⚙ Building gopenpal from source...${NC}"
    cargo build --release
    GOPENPAL_BIN="${SCRIPT_DIR}/target/release/gopenpal"
else
    GOPENPAL_BIN="gopenpal"
fi

# Initialize database
echo -e "${GREEN}✓ Initializing database...${NC}"
$GOPENPAL_BIN init

# Function to start a daemon
start_daemon() {
    local agent_name=$1
    local interval=$2
    local pid_file="${PID_DIR}/${agent_name}.pid"
    local log_file="${LOG_DIR}/${agent_name}.log"

    # Check if already running
    if [ -f "$pid_file" ]; then
        local old_pid=$(cat "$pid_file")
        if ps -p "$old_pid" > /dev/null 2>&1; then
            echo -e "${YELLOW}  ${agent_name} daemon already running (PID: $old_pid)${NC}"
            return 0
        else
            rm -f "$pid_file"
        fi
    fi

    # Start daemon in background
    echo -e "${GREEN}  Starting ${agent_name} daemon (interval: ${interval}min)...${NC}"
    nohup $GOPENPAL_BIN agent daemon --agent "$agent_name" --interval "$interval" >> "$log_file" 2>&1 &
    local pid=$!
    echo "$pid" > "$pid_file"
    echo -e "${GREEN}  ✓ ${agent_name} daemon started (PID: $pid)${NC}"
}

# Start agent daemons
echo -e "\n${BLUE}🤖 Starting Agent Daemons...${NC}"
start_daemon "Hydrix" 45
start_daemon "Serhant" 60
start_daemon "Mio" 30

# Set up cron job for water reminders
echo -e "\n${BLUE}⏰ Setting up automated reminders...${NC}"

# Check if cron job already exists
if crontab -l 2>/dev/null | grep -q "gopenpal reminder check"; then
    echo -e "${YELLOW}  Cron job already configured${NC}"
else
    echo -e "${GREEN}  Installing cron job for water reminders...${NC}"

    # Get current crontab
    crontab -l 2>/dev/null > /tmp/gopenpal_cron || true

    # Add gopenpal reminder job (every 30 minutes during waking hours)
    echo "# GopenPal water reminders - every 30 minutes (9 AM - 9 PM)" >> /tmp/gopenpal_cron
    echo "*/30 9-21 * * * $GOPENPAL_BIN reminder check" >> /tmp/gopenpal_cron

    # Install new crontab
    crontab /tmp/gopenpal_cron
    rm /tmp/gopenpal_cron

    echo -e "${GREEN}  ✓ Cron job installed (every 30 minutes, 9 AM - 9 PM)${NC}"
fi

# Log morning water intake
current_hour=$(date +%H)
if [ "$current_hour" -ge 6 ] && [ "$current_hour" -lt 12 ]; then
    echo -e "\n${BLUE}💧 Logging morning water (250ml)...${NC}"
    $GOPENPAL_BIN water log 250 --notes "Morning hydration (auto-logged)" || true
    echo -e "${GREEN}  ✓ Morning water logged${NC}"
fi

# Show status
echo -e "\n${BLUE}📊 Current Status${NC}"
echo -e "${BLUE}════════════════════════════════════════${NC}"

# Show agent list
echo -e "\n${GREEN}Active Agents:${NC}"
$GOPENPAL_BIN agent list || true

# Show today's stats
echo -e "\n${GREEN}Today's Water Intake:${NC}"
$GOPENPAL_BIN water today || true

# Show reminder settings
echo -e "\n${GREEN}Reminder Settings:${NC}"
$GOPENPAL_BIN reminder status || true

# Show daemon PIDs
echo -e "\n${GREEN}Running Daemons:${NC}"
for agent in Hydrix Serhant Mio; do
    pid_file="${PID_DIR}/${agent}.pid"
    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        if ps -p "$pid" > /dev/null 2>&1; then
            echo -e "  ${GREEN}✓${NC} ${agent} (PID: $pid)"
        else
            echo -e "  ${RED}✗${NC} ${agent} (stopped)"
        fi
    fi
done

# Cleanup function
cleanup() {
    echo -e "\n\n${YELLOW}Shutting down...${NC}"

    # Kill all daemon processes
    for agent in Hydrix Serhant Mio; do
        pid_file="${PID_DIR}/${agent}.pid"
        if [ -f "$pid_file" ]; then
            pid=$(cat "$pid_file")
            if ps -p "$pid" > /dev/null 2>&1; then
                echo -e "${YELLOW}  Stopping ${agent} daemon (PID: $pid)...${NC}"
                kill "$pid" 2>/dev/null || true
                rm -f "$pid_file"
            fi
        fi
    done

    echo -e "${GREEN}✓ All services stopped${NC}"
    echo -e "${BLUE}Run './stop-day.sh' to remove cron jobs${NC}"
    exit 0
}

# Set up trap for cleanup on exit
trap cleanup SIGINT SIGTERM EXIT

# Launch TUI
echo -e "\n${BLUE}════════════════════════════════════════${NC}"
echo -e "${GREEN}🚀 Launching TUI...${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop all services and exit${NC}"
echo -e "${BLUE}════════════════════════════════════════${NC}\n"

sleep 2

# Launch TUI (this blocks until user quits)
$GOPENPAL_BIN tui

# When TUI exits, cleanup will be called automatically by the trap
