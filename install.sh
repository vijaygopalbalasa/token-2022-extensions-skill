#!/bin/bash

# Token-2022 Extensions Skill — Standard Installer
# Installs the skill into ~/.claude/skills with recommended defaults.
# For custom locations (project-local install), use ./install-custom.sh

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/skill"

# Standard defaults
SKILLS_DIR="$HOME/.claude/skills"
SKILL_PATH="$SKILLS_DIR/token-2022-extensions"

print_banner() {
    echo ""
    echo -e "${MAGENTA}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${MAGENTA}║${NC}   ${CYAN}Token-2022 Extensions Skill${NC} for Claude Code / Codex        ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}║${NC}   ${WHITE}Pick · wire · ship SPL Token Extensions safely${NC}             ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}║${NC}   ${YELLOW}Superteam Brazil${NC}                                            ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_help() {
    echo "Token-2022 Extensions Skill — Standard Installer"
    echo ""
    echo "Usage: ./install.sh [OPTIONS]"
    echo ""
    echo "Installs to: $SKILL_PATH"
    echo ""
    echo "Options:"
    echo "  -y, --yes    Skip confirmation prompt"
    echo "  -h, --help   Show this help"
    echo ""
    echo "For a project-local install (./.claude/skills), use ./install-custom.sh"
}

CONFIRM=true
for arg in "$@"; do
    case "$arg" in
        -y|--yes) CONFIRM=false ;;
        -h|--help) print_help; exit 0 ;;
        *) echo "Unknown option: $arg"; print_help; exit 1 ;;
    esac
done

print_banner
echo -e "${WHITE}This will install the Token-2022 Extensions skill to:${NC}"
echo -e "  ${CYAN}$SKILL_PATH${NC}"
echo ""

if [ "$CONFIRM" = true ]; then
    read -r -p "Proceed? [Y/n] " reply
    case "$reply" in
        [nN]*) echo "Cancelled."; exit 0 ;;
    esac
fi

mkdir -p "$SKILLS_DIR"

if [ -d "$SKILL_PATH" ]; then
    echo -e "${YELLOW}→ Removing previous install at $SKILL_PATH${NC}"
    rm -rf "$SKILL_PATH"
fi

echo -e "${BLUE}→ Installing skill files...${NC}"
mkdir -p "$SKILL_PATH"
cp -R "$SOURCE_DIR/." "$SKILL_PATH/"

echo ""
echo -e "${GREEN}✓ Installed.${NC}"
echo ""
echo -e "${WHITE}Try asking Claude:${NC}"
echo -e "  ${CYAN}\"Which Token-2022 extensions should I use for a loyalty point token?\"${NC}"
echo -e "  ${CYAN}\"Add a transfer hook that enforces an allowlist to my mint.\"${NC}"
echo -e "  ${CYAN}\"Will my transfer-fee token work on Jupiter and Phantom?\"${NC}"
echo ""
echo -e "${YELLOW}Powered by Superteam Brazil${NC}"
