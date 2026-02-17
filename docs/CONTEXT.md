# Development Context

**Last Updated**: 2026-02-17  
**Current Phase**: Start Implementation  
**Next Milestone**: Phase 2

---

## Current State

### Completed

- Project planning and architecture design
- Documentation structure established
- MVP scope defined

### In Progress

- Phase 2

### Blocked

- None

---

## Project Overview

**Goal**: Build a minimalist, productive Discord CLI client in Rust

**Core Features**:

- Lightweight and fast
- Real-time message updates
- Interactive TUI for full-featured use
- Command-line interface for quick operations (send, read, monitor)

**Non-Goals**:

- Mobile/web version
- Voice channel support
- Multiple account support
- Rich media (embeds, attachments)

---

## Key Decisions

### 1. Architecture Style

**Decision**: Modular workspace with separate crates  
**Rationale**:

- Better separation of concerns
- Easier for LLM tools to understand individual modules
- Can compile/test modules independently
- Clear dependency graph

### 2. Discord Library Choice

- **serenity**: More features, higher level, well documented

### 3. TUI Framework

**Decision**: `ratatui` + `crossterm`  
**Rationale**:

- Most active development
- Modern API
- Good documentation
- Used by many popular TUI apps

### 4. Async Runtime

**Decision**: `tokio` with full features  
**Rationale**:

- Industry standard
- Excellent documentation
- LLM tools have extensive training on tokio patterns
- Required by most Discord libraries anyway

### 5. Error Handling

**Decision**: `anyhow` for application, `thiserror` for libraries  
**Rationale**:

- anyhow: Simple, ergonomic for binary crate
- thiserror: Better for library errors with proper error types

### 6. Configuration

**Decision**: TOML file + environment variables  
**Location**: `~/.config/discord-cli/config.toml`  
**Rationale**:

- TOML is human-friendly
- Standard location for Linux tools
- Env vars for CI/testing

---

## Technical Stack Summary

```
Language:      Rust 1.75+
Async Runtime: tokio 1.x
HTTP Client:   reqwest 0.11
WebSocket:     tokio-tungstenite 0.21
TUI:           ratatui 0.26 + crossterm 0.27
CLI Parsing:   clap 4.x (with derive)
Config:        config 0.14 + serde
Errors:        anyhow + thiserror
Discord:       serenity
Cache:         lru 0.12
```

---

## Development Environment Setup

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Required tools
cargo install cargo-watch  # For development
cargo install cargo-edit   # For managing dependencies
```

### Project Initialization (Next Step)

```bash
# Create workspace
mkdir discline && cd discline
cargo init --lib crates/types
cargo init --lib crates/client
cargo init --lib crates/state
cargo init --lib crates/tui
cargo init --lib crates/commands
cargo init crates/cli

# Set up workspace Cargo.toml
```

---

## File Structure Status

```
discline/
├── README.md             Created
├── Cargo.toml            Created
├── .gitignore            Created
├── config.example.toml   To create
│
├── crates/               Created
│   ├── types/
│   ├── client/
│   ├── state/
│   ├── tui/
│   ├── commands/
│   └── cli/
│
├── docs/
│   ├── ARCHITECTURE.md   Created
│   ├── TASKS.md          Created
│   ├── CONTEXT.md        Created
│   └── DEVELOPMENT.md    To create
```

---

## Open Questions

### 1. Discord Authentication

**Question**: Use bot token or user token?  
**Current Thinking**: Support both, but recommend bot token  
**Why**: User token (self-bot) violates Discord TOS  
**Action**: Clearly document this in README

### 2. Rate Limiting Strategy

**Question**: How aggressive should rate limit handling be?  
**Options**:

- A) Simple exponential backoff
- B) Pre-emptive rate limit tracking
- C) Queue-based request management

**Recommendation**: Start with A for MVP, upgrade to B/C if needed

### 3. Message Cache Size

**Question**: How many messages to cache by default?  
**Current Thinking**: 1000 messages total (across all channels)  
**Rationale**: ~100KB in memory, covers most use cases  
**Make it configurable**: Yes

---

## Known Risks & Mitigation

### Risk 1: Discord API Changes

**Impact**: High  
**Probability**: Low  
**Mitigation**:

- Use established library (serenity/twilight) that handles versioning
- Pin API version in gateway connection
- Monitor Discord developer updates

### Risk 2: WebSocket Reconnection Complexity

**Impact**: Medium  
**Probability**: Medium  
**Mitigation**:

- Start with simple reconnect logic
- Use library's built-in reconnection if available
- Test with network interruptions

### Risk 3: TUI Rendering Performance

**Impact**: Medium  
**Probability**: Low  
**Mitigation**:

- Use efficient diff-based rendering (ratatui does this)
- Limit message history display
- Profile early and optimize hot paths

---

## LLM Collaboration Notes

### Working with AI Assistants

**Current Assistant**: Gemini CLI  
**Alternative Tools**: Cursor, GitHub Copilot, Claude Code

**Best Practices**:

1. **Provide Full Context**: Always share ARCHITECTURE.md + this file
2. **Reference Task Numbers**: "I'm working on Task 2.2"
3. **Show Examples**: Link to similar code when asking for help
4. **Verify Code**: Test all generated code before committing
5. **Update This File**: Record decisions and progress

**Example Workflow**:

```bash
# 1. Pick a task from TASKS.md
# 2. Read relevant architecture section
# 3. Ask LLM for implementation with context
# 4. Review and test generated code
# 5. Update CONTEXT.md with any decisions
# 6. Mark task complete in TASKS.md
```

### Code Generation Guidelines for LLMs

When requesting code:

- Specify exact file path
- Reference types from discord-types crate
- Request error handling with anyhow
- Ask for doc comments
- Request unit tests
- Don't generate >100 lines at once
- Don't mix concerns from multiple modules

---

## Recent Changes

**2026-02-17**:

- Initial project setup
- Created all planning documentation
- Defined MVP scope
- Ready to start implementation

---

## Resources

### Documentation

- Ratatui Book: https://ratatui.rs
- Serenity Docs: https://docs.rs/serenity
- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- Discord API Docs: https://discord.com/developers/docs

### Example Projects

- cordless: https://github.com/Bios-Marcel/cordless
- gtkcord4: https://github.com/diamondburned/gtkcord4
- arikawa (Go): https://github.com/diamondburned/arikawa

### Helpful Crates

- `lru`: https://docs.rs/lru
- `clap`: https://docs.rs/clap
- `anyhow`: https://docs.rs/anyhow
- `thiserror`: https://docs.rs/thiserror

---

## Contact & Feedback

**Primary Developer**: Victor Vasconcelos
**Project Status**: MVP Development  
**Contributions**: Not accepting until MVP complete  
**Feedback**: Use GitHub Issues
