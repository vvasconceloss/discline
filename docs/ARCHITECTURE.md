# Architecture

## Overview

Discline is built as a modular, layered system designed for maintainability and LLM-assisted development.

## Core Principles

1. **Separation of Concerns**: Each crate has a single, well-defined responsibility
2. **Small Modules**: Functions < 50 lines, files < 300 lines when possible
3. **Type Safety**: Use Rust's type system to prevent invalid states
4. **Progressive Enhancement**: MVP features first, complexity added incrementally

## Module Descriptions

### types

**Purpose**: Shared type definitions used across all modules

**Exports**:

```rust
pub struct Message {
    pub id: MessageId,
    pub content: String,
    pub author: User,
    pub timestamp: DateTime<Utc>,
    pub channel_id: ChannelId,
}

pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub channel_type: ChannelType,
}

pub struct User {
    pub id: UserId,
    pub name: String,
    pub discriminator: String,
}

// ID types for type safety
pub struct MessageId(pub u64);
pub struct ChannelId(pub u64);
pub struct UserId(pub u64);
pub struct GuildId(pub u64);
```

**Dependencies**: None (serde only)

**Size Goal**: < 200 lines

---

### client

**Purpose**: Discord API interaction (REST + WebSocket)

**Modules**:

#### rest.rs

```rust
pub struct RestClient {
    http: reqwest::Client,
    token: String,
    base_url: String,
}

impl RestClient {
    // Core operations
    pub async fn send_message(&self, channel_id: ChannelId, content: &str) -> Result<Message>;
    pub async fn get_messages(&self, channel_id: ChannelId, limit: u8) -> Result<Vec<Message>>;
    pub async fn get_channels(&self, guild_id: GuildId) -> Result<Vec<Channel>>;
}
```

#### gateway.rs

```rust
pub struct Gateway {
    ws: WebSocketStream,
    heartbeat_interval: Duration,
}

impl Gateway {
    pub async fn connect(token: &str) -> Result<Self>;
    pub async fn next_event(&mut self) -> Result<Event>;
}

pub enum Event {
    MessageCreate(Message),
    MessageUpdate(Message),
    Ready { user: User, guilds: Vec<Guild> },
    // ... other events
}
```

#### cache.rs

```rust
use lru::LruCache;

pub struct Cache {
    messages: LruCache<ChannelId, Vec<Message>>,
    users: HashMap<UserId, User>,
    channels: HashMap<ChannelId, Channel>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self;
    pub fn insert_message(&mut self, msg: Message);
    pub fn get_messages(&self, channel_id: &ChannelId) -> Option<&[Message]>;
}
```

**Dependencies**:

- reqwest (HTTP)
- tokio (async runtime)
- serenity (Discord lib)
- tokio-tungstenite (WebSocket)

**Size Goal**: ~600 lines total (200 per module)

---

### state

**Purpose**: Application state and session management

**Modules**:

#### session.rs

```rust
pub struct Session {
    pub current_guild: Option<GuildId>,
    pub current_channel: Option<ChannelId>,
    pub user: User,
}

impl Session {
    pub fn new(user: User) -> Self;
    pub fn set_channel(&mut self, channel: ChannelId);
    pub fn current_context(&self) -> Option<(GuildId, ChannelId)>;
}
```

#### history.rs

```rust
pub struct MessageHistory {
    cache: Cache,
    unread_counts: HashMap<ChannelId, usize>,
}

impl MessageHistory {
    pub fn add_message(&mut self, msg: Message);
    pub fn get_recent(&self, channel: &ChannelId, limit: usize) -> Vec<&Message>;
    pub fn mark_read(&mut self, channel: &ChannelId);
}
```

**Dependencies**: types, client

**Size Goal**: ~300 lines total

---

### commands

**Purpose**: Parse and execute CLI commands

**Modules**:

#### send.rs

```rust
pub async fn execute_send(
    client: &RestClient,
    channel: &str,
    message: &str,
) -> Result<()> {
    // Parse channel name/ID
    // Send message
    // Print confirmation
}
```

#### read.rs

```rust
pub async fn execute_read(
    client: &RestClient,
    cache: &mut Cache,
    channel: &str,
    limit: u8,
) -> Result<()> {
    // Fetch messages
    // Update cache
    // Print formatted output
}
```

#### monitor.rs

```rust
pub async fn execute_monitor(
    gateway: Gateway,
    channels: Vec<ChannelId>,
) -> Result<()> {
    // Listen to events
    // Filter by channels
    // Print notifications
}
```

**Dependencies**: types, client, state, clap

**Size Goal**: ~400 lines total

---

### tui

**Purpose**: Terminal user interface with ratatui

**Structure**:

```rust
// app.rs - Main TUI application
pub struct App {
    state: AppState,
    client: Arc<RestClient>,
    gateway: Arc<Mutex<Gateway>>,
}

pub enum AppState {
    ChannelList,
    MessageView,
    MessageInput,
}

// components/channel_list.rs
pub struct ChannelList {
    channels: Vec<Channel>,
    selected: usize,
}

impl Component for ChannelList {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_key(&mut self, key: KeyEvent) -> Action;
}

// components/message_list.rs
pub struct MessageList {
    messages: Vec<Message>,
    scroll: usize,
}

// components/input.rs
pub struct MessageInput {
    buffer: String,
    cursor: usize,
}
```

**Layout**:

```
┌─────────────────────────────────────────────┐
│ [Server Name]                    [@username]│
├─────────┬───────────────────────────────────┤
│ Guilds  │ #general                          │
│         │                                   │
│ > Guild1│ user1: Hello!                     │
│   Guild2│ user2: Hi there                   │
│         │ user1: How are you?               │
│         │                                   │
│         │                                   │
│         │                                   │
│         │                                   │
├─────────┴───────────────────────────────────┤
│ > Type a message...                         │
└─────────────────────────────────────────────┘
```

**Dependencies**: ratatui, crossterm, types, state

**Size Goal**: ~800 lines total (200 per component)

---

### cli

**Purpose**: Entry point and orchestration

**main.rs**:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "discline")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(long)]
    token: Option<String>,

    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Send { channel: String, message: String },
    Read { channel: String, limit: Option<u8> },
    Monitor { channels: Vec<String> },
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load config
    let config = load_config(cli.config)?;

    // Initialize client
    let client = RestClient::new(&config.token);

    // Route to appropriate handler
    match cli.command.unwrap_or(Command::Tui) {
        Command::Send { channel, message } => {
            commands::send::execute_send(&client, &channel, &message).await?;
        }
        Command::Tui => {
            tui::run(client).await?;
        }
        // ... other commands
    }

    Ok(())
}
```

**Dependencies**: All other crates, clap, tokio

**Size Goal**: ~150 lines

---

## Data Flow

### Sending a Message (CLI Mode)

```
User Input
    ↓
cli/main.rs (parse args)
    ↓
commands/send.rs (validate)
    ↓
client/rest.rs (HTTP POST)
    ↓
Discord API
```

### Receiving Messages (TUI Mode)

```
Discord API
    ↓
client/gateway.rs (WebSocket event)
    ↓
state/history.rs (cache update)
    ↓
tui/app.rs (state change)
    ↓
tui/components/message_list.rs (re-render)
```

## Error Handling Strategy

**Use anyhow for application errors**:

```rust
use anyhow::{Context, Result};

pub async fn send_message(content: &str) -> Result<()> {
    validate_content(content)
        .context("Invalid message content")?;

    client.post(content)
        .await
        .context("Failed to send message")?;

    Ok(())
}
```

**Use thiserror for library errors**:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Rate limited: Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },
}
```

## Configuration Management

**Config file**: `~/.config/discline/config.toml`

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub ui: UiConfig,
    pub cache: CacheConfig,
}

#[derive(Deserialize, Serialize)]
pub struct AuthConfig {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UiConfig {
    pub theme: String,
    pub vim_mode: bool,
}

#[derive(Deserialize, Serialize)]
pub struct CacheConfig {
    pub max_messages: usize,
}

// Load with precedence: CLI args > Env vars > Config file > Defaults
```

## Testing Strategy

**Unit Tests**: Each module independently

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_validation() {
        assert!(validate_content("Hello").is_ok());
        assert!(validate_content(&"a".repeat(2001)).is_err());
    }
}
```

**Integration Tests**: End-to-end flows

```rust
#[tokio::test]
async fn test_send_and_receive() {
    let client = RestClient::new(test_token());
    // ... test flow
}
```

**Mock Gateway for Testing**:

```rust
pub struct MockGateway {
    events: VecDeque<Event>,
}
```

## Performance Considerations

1. **Message Cache**: LRU cache with configurable size
2. **Lazy Loading**: Only fetch messages when needed
3. **Async I/O**: Non-blocking operations throughout
4. **Minimal Redraws**: Only update changed UI components

## Security

1. **Token Storage**: Never log or display tokens
2. **Config Permissions**: Restrict config file to user-only read
3. **Rate Limiting**: Respect Discord's rate limits
4. **Input Validation**: Sanitize all user input

## Scope

**Include**:

- Send messages (CLI)
- Read messages (CLI)
- Basic TUI with channel list
- Real-time message reception
- Configuration file support

**Exclude**:

- Voice channel support
- Rich embeds/attachments
- Reactions
- User presence
- Advanced formatting
- Multiple accounts

## Future Enhancements

- Plugin system for custom commands
- Scriptable automation
- Advanced search and filtering
- Theming support
- Vim-like command mode
- Message editing and deletion
