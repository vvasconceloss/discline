# Tasks - Development

**Project**: Discline

---

## Task Breakdown Strategy

Each task is:

- **Small**: Completable in 1-3 hours
- **Independent**: Minimal dependencies on other tasks
- **Testable**: Clear success criteria

---

## Phase 1:

### 1.1 Project Structure Setup

**Priority**: P0 (Blocker)  
**Estimate**: 30 min  
**Files**: `Cargo.toml`, directory structure  
**Dependencies**: None

**Tasks**:

- [x] Create workspace Cargo.toml
- [x] Create all crate directories
- [x] Set up workspace dependencies
- [x] Add basic .gitignore
- [x] Verify `cargo build` works

**Success Criteria**: `cargo build` compiles empty project

---

### 1.2 Discord Types Crate

**Priority**: P0 (Blocker)  
**Estimate**: 1 hour  
**Files**: `crates/types/lib.rs`  
**Dependencies**: Task 1.1

**Tasks**:

- [x] Define `MessageId`, `ChannelId`, `UserId`, `GuildId` newtypes
- [x] Define `Message` struct with serde
- [x] Define `Channel` struct
- [x] Define `User` struct
- [x] Add basic Display implementations
- [x] Write unit tests for type conversions

**Success Criteria**: All types compile with serde, tests pass

---

### 1.3 Configuration Management

**Priority**: P0 (Blocker)  
**Estimate**: 1.5 hours  
**Files**: `crates/cli/config.rs`  
**Dependencies**: Task 1.1

**Tasks**:

- [x] Define `Config` struct with serde
- [x] Implement config file loading from `~/.config/discline/config.toml`
- [x] Implement environment variable override (`DISCORD_TOKEN`)
- [x] Add config validation
- [x] Create example config.toml
- [x] Write tests for config loading precedence

**Success Criteria**: Config loads from file OR env var, validates token exists

---

## Phase 2

### 2.1 REST Client - Core Setup

**Priority**: P0 (Blocker)  
**Estimate**: 2 hours  
**Files**: `crates/client/rest.rs`  
**Dependencies**: Task 1.2

**Tasks**:

- [x] Create `RestClient` struct with reqwest
- [x] Implement authentication headers
- [x] Add base URL configuration
- [x] Implement error type (`ClientError`)
- [x] Add rate limit handling (basic)
- [x] Write connection test

**Success Criteria**: Can create authenticated client, handles 401 errors

---

### 2.2 REST Client - Send Message

**Priority**: P0 (Blocker)  
**Estimate**: 1.5 hours  
**Files**: `crates/client/rest.rs`  
**Dependencies**: Task 2.1

**Tasks**:

- [ ] Implement `send_message(channel_id, content)`
- [ ] Validate message length (≤ 2000 chars)
- [ ] Handle Discord API errors
- [ ] Add retry logic for 5xx errors
- [ ] Write integration test (with test token)

**Success Criteria**: Can send message to test channel

**API Endpoint**: `POST /channels/{channel.id}/messages`

---

### 2.3 REST Client - Fetch Messages

**Priority**: P0 (Blocker)  
**Estimate**: 1.5 hours  
**Files**: `crates/client/rest.rs`  
**Dependencies**: Task 2.1

**Tasks**:

- [ ] Implement `get_messages(channel_id, limit)`
- [ ] Parse response into `Vec<Message>`
- [ ] Handle pagination params
- [ ] Add sorting (newest first)
- [ ] Write integration test

**Success Criteria**: Can fetch last 50 messages from test channel

**API Endpoint**: `GET /channels/{channel.id}/messages?limit={limit}`

---

### 2.4 Gateway - Connection

**Priority**: P1 (High)  
**Estimate**: 3 hours  
**Files**: `crates/client/gateway.rs`  
**Dependencies**: Task 1.2

**Tasks**:

- [ ] Implement WebSocket connection to Discord Gateway
- [ ] Handle HELLO and READY events
- [ ] Implement heartbeat mechanism
- [ ] Add reconnection logic
- [ ] Define `Event` enum for common events
- [ ] Write connection test

**Success Criteria**: Can connect, maintain heartbeat, receive READY event

**Discord Gateway URL**: `wss://gateway.discord.gg/?v=10&encoding=json`

---

### 2.5 Gateway - Message Events

**Priority**: P1 (High)  
**Estimate**: 2 hours  
**Files**: `crates/client/gateway.rs`  
**Dependencies**: Task 2.4

**Tasks**:

- [ ] Handle MESSAGE_CREATE events
- [ ] Parse events into `Event::MessageCreate(Message)`
- [ ] Add event filtering by channel
- [ ] Implement `next_event()` async method
- [ ] Write event reception test

**Success Criteria**: Can receive and parse new message events

---

### 2.6 Cache Implementation

**Priority**: P2 (Medium)  
**Estimate**: 2 hours  
**Files**: `crates/client/cache.rs`  
**Dependencies**: Task 1.2

**Tasks**:

- [ ] Create `Cache` struct with LruCache
- [ ] Implement message storage per channel
- [ ] Implement user/channel metadata caching
- [ ] Add cache size limits (configurable)
- [ ] Write cache eviction tests

**Success Criteria**: Can store 1000 messages, evicts oldest on overflow

---

## Phase 3:

### 3.1 Session Management

**Priority**: P1 (High)  
**Estimate**: 1.5 hours  
**Files**: `crates/state/session.rs`  
**Dependencies**: Task 1.2

**Tasks**:

- [ ] Create `Session` struct
- [ ] Track current guild/channel
- [ ] Track authenticated user
- [ ] Implement context switching methods
- [ ] Write session state tests

**Success Criteria**: Can set/get current channel context

---

### 3.2 Message History

**Priority**: P1 (High)  
**Estimate**: 2 hours  
**Files**: `crates/state/history.rs`  
**Dependencies**: Task 2.6, Task 3.1

**Tasks**:

- [ ] Create `MessageHistory` wrapper around Cache
- [ ] Implement unread count tracking
- [ ] Add mark-as-read functionality
- [ ] Implement query methods (get_recent, get_unread)
- [ ] Write history management tests

**Success Criteria**: Can track unread messages per channel

---

## Phase 4

### 4.1 CLI Argument Parsing

**Priority**: P0 (Blocker)  
**Estimate**: 1 hour  
**Files**: `crates/cli/main.rs`  
**Dependencies**: None

**Tasks**:

- [ ] Define `Cli` struct with clap
- [ ] Define `Command` enum (Send, Read, Monitor, Tui)
- [ ] Add global options (--token, --config)
- [ ] Implement subcommand parsing
- [ ] Test argument combinations

**Success Criteria**: `cargo run -- --help` shows all commands

---

### 4.2 Send Command

**Priority**: P0 (Blocker)  
**Estimate**: 2 hours  
**Files**: `crates/commands/send.rs`  
**Dependencies**: Task 2.2, Task 4.1

**Tasks**:

- [ ] Implement `execute_send()` function
- [ ] Parse channel name/ID from argument
- [ ] Validate message content
- [ ] Call REST client
- [ ] Print success confirmation
- [ ] Handle errors gracefully

**Success Criteria**: `discline send "#test" "Hello"` sends message

**Example**:

```bash
$ discline send "#general" "Hello from CLI!"
✓ Message sent to #general
```

---

### 4.3 Read Command

**Priority**: P0 (Blocker)  
**Estimate**: 2 hours  
**Files**: `crates/commands/read.rs`  
**Dependencies**: Task 2.3, Task 4.1

**Tasks**:

- [ ] Implement `execute_read()` function
- [ ] Parse channel and optional limit
- [ ] Fetch messages via REST client
- [ ] Format output (timestamp, author, content)
- [ ] Add color coding for readability
- [ ] Handle errors

**Success Criteria**: `discline read "#test" --limit 10` displays messages

**Example Output**:

```
[2024-02-15 14:30] user1: Hello!
[2024-02-15 14:31] user2: Hi there!
```

---

### 4.4 Monitor Command

**Priority**: P1 (High)  
**Estimate**: 2.5 hours  
**Files**: `crates/commands/monitor.rs`  
**Dependencies**: Task 2.5, Task 4.1

**Tasks**:

- [ ] Implement `execute_monitor()` function
- [ ] Connect to Gateway
- [ ] Filter events by specified channels
- [ ] Format and print new messages
- [ ] Add graceful shutdown (Ctrl+C)
- [ ] Handle reconnection

**Success Criteria**: Shows new messages in real-time

**Example Output**:

```
Monitoring: #general, #dev
[#general] user1: New message!
[#dev] user2: Bug fixed!
```

---

## Phase 5: TUI

### 5.1 TUI - Basic Layout

**Priority**: P1 (High)  
**Estimate**: 3 hours  
**Files**: `crates/tui/app.rs`  
**Dependencies**: Task 2.4, Task 3.1

**Tasks**:

- [ ] Set up ratatui terminal initialization
- [ ] Create 3-panel layout (guilds, channels, messages)
- [ ] Implement event loop
- [ ] Add basic keyboard handling (q to quit)
- [ ] Handle terminal cleanup on exit
- [ ] Test rendering

**Success Criteria**: TUI launches and displays layout

---

### 5.2 TUI - Channel List Component

**Priority**: P1 (High)  
**Estimate**: 2.5 hours  
**Files**: `crates/tui/components/channel_list.rs`  
**Dependencies**: Task 5.1

**Tasks**:

- [ ] Create `ChannelList` widget
- [ ] Implement list rendering
- [ ] Add navigation (j/k or arrow keys)
- [ ] Highlight selected channel
- [ ] Add Enter to select
- [ ] Write component tests

**Success Criteria**: Can navigate and select channels with keyboard

---

### 5.3 TUI - Message List Component

**Priority**: P1 (High)  
**Estimate**: 3 hours  
**Files**: `crates/tui/components/message_list.rs`  
**Dependencies**: Task 5.1, Task 2.3

**Tasks**:

- [ ] Create `MessageList` widget
- [ ] Fetch and display messages for selected channel
- [ ] Implement scrolling
- [ ] Format messages (time, author, content)
- [ ] Add auto-scroll on new message
- [ ] Handle long messages (word wrap)

**Success Criteria**: Messages display and scroll correctly

---

### 5.4 TUI - Message Input Component

**Priority**: P1 (High)  
**Estimate**: 2.5 hours  
**Files**: `crates/tui/components/input.rs`  
**Dependencies**: Task 5.1, Task 2.2

**Tasks**:

- [ ] Create `MessageInput` widget
- [ ] Handle text input
- [ ] Implement cursor movement
- [ ] Add Enter to send, Esc to cancel
- [ ] Show character count
- [ ] Validate before sending

**Success Criteria**: Can type and send messages from TUI

---

### 5.5 TUI - Real-time Updates

**Priority**: P1 (High)  
**Estimate**: 3 hours  
**Files**: `crates/tui/app.rs`  
**Dependencies**: Task 5.3, Task 2.5

**Tasks**:

- [ ] Integrate Gateway events into TUI
- [ ] Update message list on MESSAGE_CREATE
- [ ] Handle events in background task
- [ ] Implement channel-based message routing
- [ ] Add visual notification for new messages
- [ ] Test concurrent updates

**Success Criteria**: New messages appear in real-time without manual refresh

---

## Phase 6

### 6.1 Error Handling Improvements

**Priority**: P2 (Medium)  
**Estimate**: 2 hours  
**Files**: All modules  
**Dependencies**: All previous tasks

**Tasks**:

- [ ] Review all error handling
- [ ] Add context to errors with anyhow
- [ ] Implement user-friendly error messages
- [ ] Handle network failures gracefully
- [ ] Add error logging

**Success Criteria**: No panics, all errors reported clearly

---

### 6.2 Documentation

**Priority**: P2 (Medium)  
**Estimate**: 2 hours  
**Files**: All lib.rs files  
**Dependencies**: All previous tasks

**Tasks**:

- [ ] Add rustdoc comments to public APIs
- [ ] Include code examples in docs
- [ ] Update README with build instructions
- [ ] Create usage examples
- [ ] Generate docs: `cargo doc --open`

**Success Criteria**: All public items documented

---

### 6.3 Integration Testing

**Priority**: P2 (Medium)  
**Estimate**: 3 hours  
**Files**: `tests/` directory  
**Dependencies**: All feature tasks

**Tasks**:

- [ ] Write end-to-end send/receive test
- [ ] Test TUI initialization and cleanup
- [ ] Test config loading from different sources
- [ ] Test error scenarios
- [ ] Set up CI test runner (optional)

**Success Criteria**: `cargo test --all` passes

---

### 6.4 Performance Optimization

**Priority**: P3 (Low)  
**Estimate**: 2 hours  
**Files**: Cache, rendering code  
**Dependencies**: Working MVP

**Tasks**:

- [ ] Profile rendering performance
- [ ] Optimize message cache size
- [ ] Reduce unnecessary redraws
- [ ] Benchmark startup time
- [ ] Document performance characteristics

**Success Criteria**: TUI renders at 60fps, startup < 1 second

---

## Quick Reference

### Priority Levels

- **P0**: Must have for MVP (blocker)
- **P1**: Important for MVP (high priority)
- **P2**: Nice to have for MVP (medium priority)
- **P3**: Post-MVP (low priority)

---

## LLM Collaboration Tips

When working with an LLM on a task:

1. **Provide Context**: Share ARCHITECTURE.md + CONTEXT.md + specific task
2. **One Task at a Time**: Don't mix multiple tasks
3. **Show Examples**: Reference similar working code
4. **Verify Output**: Test generated code before moving on
5. **Update CONTEXT.md**: Record decisions and progress

**Example Prompt**:

```
I'm working on Task 2.2: REST Client - Send Message

Context:
- We're using reqwest for HTTP
- Authentication is via Bearer token
- Need to handle rate limiting

Please implement the send_message() function following
the pattern in ARCHITECTURE.md section 2.2
```
