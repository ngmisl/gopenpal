#!/usr/bin/env bash
#
# GopenPal Shutdown Script
# This script stops all GopenPal services:
# - Agent daemons
# - Cron jobs (optional)
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
PID_DIR="${SCRIPT_DIR}/.gopenpal_pids"
LOG_DIR="${SCRIPT_DIR}/.gopenpal_logs"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   GopenPal - Shutdown System          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}\n"

# Stop agent daemons
echo -e "${YELLOW}🛑 Stopping Agent Daemons...${NC}"

stopped_count=0
for agent in Hydrix Serhant Mio Karen; do
    pid_file="${PID_DIR}/${agent}.pid"
    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        if ps -p "$pid" > /dev/null 2>&1; then
            echo -e "${YELLOW}  Stopping ${agent} daemon (PID: $pid)...${NC}"
            kill "$pid" 2>/dev/null || true
            sleep 1

            # Force kill if still running
            if ps -p "$pid" > /dev/null 2>&1; then
                echo -e "${RED}  Force killing ${agent}...${NC}"
                kill -9 "$pid" 2>/dev/null || true
            fi

            rm -f "$pid_file"
            echo -e "${GREEN}  ✓ ${agent} daemon stopped${NC}"
            ((stopped_count++))
        else
            echo -e "${YELLOW}  ${agent} daemon not running${NC}"
            rm -f "$pid_file"
        fi
    else
        echo -e "${YELLOW}  ${agent} daemon not running${NC}"
    fi
done

if [ $stopped_count -eq 0 ]; then
    echo -e "${YELLOW}  No daemons were running${NC}"
else
    echo -e "${GREEN}  ✓ Stopped $stopped_count daemon(s)${NC}"
fi

# Ask about removing cron jobs
echo -e "\n${YELLOW}⏰ Remove cron jobs?${NC}"
echo -e "  This will remove automated water reminders from your crontab"
echo -e "  ${BLUE}Remove cron jobs? (y/n)${NC}"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}  Removing GopenPal cron jobs...${NC}"

    # Get current crontab and filter out gopenpal entries
    if crontab -l 2>/dev/null | grep -v "gopenpal" > /tmp/gopenpal_cron; then
        crontab /tmp/gopenpal_cron
        rm /tmp/gopenpal_cron
        echo -e "${GREEN}  ✓ Cron jobs removed${NC}"
    else
        # If all lines were filtered, clear crontab
        crontab -r 2>/dev/null || true
        echo -e "${GREEN}  ✓ Crontab cleared${NC}"
    fi
else
    echo -e "${BLUE}  Cron jobs kept (reminders will continue)${NC}"
fi

# Show log file locations
if [ -d "$LOG_DIR" ]; then
    echo -e "\n${BLUE}📋 Log Files:${NC}"
    for agent in Hydrix Serhant Mio Karen; do
        log_file="${LOG_DIR}/${agent}.log"
        if [ -f "$log_file" ]; then
            size=$(du -h "$log_file" | cut -f1)
            echo -e "  ${agent}: ${log_file} (${size})"
        fi
    done
    echo -e "\n${YELLOW}  To clear logs: rm -rf ${LOG_DIR}${NC}"
fi

echo -e "\n${GREEN}✓ Shutdown complete${NC}"
echo -e "${BLUE}Run './start-day.sh' to start services again${NC}\n"
