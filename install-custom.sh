#!/bin/bash

# Token-2022 Extensions Skill — Custom Installer
# Choose between a personal (~/.claude/skills) or project-local (./.claude/skills) install.

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/skill"

print_banner() {
    echo ""
    echo -e "${MAGENTA}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${MAGENTA}║${NC}   ${CYAN}Token-2022 Extensions Skill${NC} — Custom Installer            ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}║${NC}   ${YELLOW}Superteam Brazil${NC}                                            ${MAGENTA}║${NC}"
    echo -e "${MAGENTA}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_banner

echo -e "${WHITE}Where do you want to install the skill?${NC}"
echo "  1) Personal   — ~/.claude/skills/token-2022-extensions (all projects)"
echo "  2) Project     — ./.claude/skills/token-2022-extensions (this repo only)"
echo "  3) Cancel"
echo ""
read -r -p "Choice [1/2/3]: " choice

case "$choice" in
    1) SKILLS_DIR="$HOME/.claude/skills" ;;
    2) SKILLS_DIR="$(pwd)/.claude/skills" ;;
    3) echo "Cancelled."; exit 0 ;;
    *) echo "Invalid choice."; exit 1 ;;
esac

SKILL_PATH="$SKILLS_DIR/token-2022-extensions"

if [ -d "$SKILL_PATH" ]; then
    echo -e "${YELLOW}→ $SKILL_PATH already exists.${NC}"
    read -r -p "Overwrite? [y/N] " ow
    case "$ow" in
        [yY]*) rm -rf "$SKILL_PATH" ;;
        *) echo "Cancelled."; exit 0 ;;
    esac
fi

echo -e "${BLUE}→ Installing to $SKILL_PATH${NC}"
mkdir -p "$SKILL_PATH"
cp -R "$SOURCE_DIR/." "$SKILL_PATH/"

echo ""
echo -e "${GREEN}✓ Installed to $SKILL_PATH${NC}"
echo -e "${WHITE}Optional:${NC} copy agents/, commands/, and rules/ into the matching"
echo -e "  .claude/ folders if you want the bundled sub-agents and slash commands."
echo ""
echo -e "${YELLOW}Powered by Superteam Brazil${NC}"
