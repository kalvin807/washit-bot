# CLAUDE.md - AI Assistant Guide for washit-bot

## Project Overview

**washit-bot** is a Discord bot built in Rust, featuring AI-powered conversations, image generation, and utility functions. The bot integrates with OpenAI (ChatGPT & DALL-E-3), Redis for state management, and external APIs for sports data.

- **Version**: 0.2.0
- **Language**: Rust (Edition 2021)
- **License**: MIT

## Quick Reference

### Build & Test Commands

```bash
# Build the project
cargo build --verbose

# Run tests
cargo test --verbose

# Run the bot (requires .env file with credentials)
cargo run
```

### Required Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DISCORD_TOKEN` | Discord bot authentication token | Yes |
| `REDIS_DSL` | Redis connection string (DSN/URL) | Yes |
| `OPENAI_KEY` | OpenAI API key for ChatGPT and DALL-E | Yes |
| `SYSTEM_PROMPT` | Custom system prompt for ChatGPT | No |
| `MODEL_ID` | OpenAI model (defaults to `gpt-3.5-turbo`) | No |

## Project Structure

```
washit-bot/
├── Cargo.toml                    # Project manifest & dependencies
├── Cargo.lock                    # Locked dependency versions
├── .github/workflows/rust.yml    # CI pipeline (build + test on master)
├── .devcontainer/                # VSCode dev container config
└── src/
    ├── main.rs                   # Entry point, event handler setup
    ├── libs.rs                   # Module exports for libs/
    ├── commands.rs               # Module exports for commands/
    ├── handlers.rs               # Module exports for handlers/
    ├── utils.rs                  # Module exports for utils/
    ├── libs/
    │   └── epl_data_client.rs    # EPL standings API client
    ├── commands/
    │   ├── math.rs               # ~multiply command
    │   ├── meta.rs               # ~ping command
    │   ├── rw.rs                 # ~read, ~write Redis commands
    │   ├── imagine.rs            # /imagine slash command (DALL-E-3)
    │   └── epl_standing.rs       # /epl_standing slash command
    ├── handlers/
    │   ├── chat.rs               # ChatGPT @mention handler
    │   ├── ming.rs               # URL deduplication detector
    │   └── twitter.rs            # Twitter→vxtwitter converter (disabled)
    └── utils/
        ├── openai.rs             # OpenAI API client (ChatGPT + DALL-E)
        └── redis_client.rs       # Serenity TypeMapKey for Redis
```

## Architecture

### Core Components

1. **Entry Point** (`main.rs`)
   - Initializes Discord client with Serenity framework
   - Configures prefix commands (prefix: `~`)
   - Registers slash commands on bot ready
   - Sets up Redis client in Serenity's shared data context
   - Handles graceful shutdown via Ctrl+C

2. **Commands** - User-invoked functionality
   - **Prefix commands** (`~`): `multiply`, `ping`, `read`, `write`
   - **Slash commands** (`/`): `imagine`, `epl_standing`

3. **Handlers** - Auto-triggered message processors
   - **chat.rs**: Responds to @mentions with ChatGPT, maintains conversation history in Redis
   - **ming.rs**: Detects duplicate URLs, reacts with emoji, uses 24-hour TTL in Redis
   - **twitter.rs**: Converts Twitter URLs to vxtwitter (currently disabled)

4. **Utils** - Shared utilities
   - **openai.rs**: OpenAI API wrapper for chat completions and image generation
   - **redis_client.rs**: Serenity TypeMapKey implementation

5. **Libs** - External data integrations
   - **epl_data_client.rs**: Fetches EPL standings from Cloudflare Worker endpoint

### Technology Stack

| Component | Technology |
|-----------|------------|
| Async Runtime | Tokio 1.32.0 |
| Discord Framework | Serenity 0.11 |
| HTTP Client | reqwest 0.11 |
| Data Storage | Redis 0.23.3 |
| Serialization | serde 1.0.188 |
| Logging | tracing + tracing-subscriber |
| Table Formatting | comfy-table 7.1.1 |

## Code Conventions

### Naming

- **Functions**: `snake_case` (e.g., `ask_chat_gpt`, `get_standings`)
- **Structs**: `PascalCase` (e.g., `TeamStanding`, `ChatGPTResponse`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `BOT_ID`, `TTL_SECONDS`)
- **Modules**: `snake_case` matching file names

### Patterns

- **Async-first**: All I/O operations use async/await with Tokio
- **Error propagation**: Use `?` operator and `Result` types
- **Lazy initialization**: `lazy_static!` for compiled regex patterns
- **Serenity macros**: `#[command]`, `#[async_trait]`, `#[group]`
- **Serde attributes**: `#[serde(rename_all = "camelCase")]` for API responses

### Adding New Commands

**Prefix Command** (e.g., `~mycommand`):
1. Create `src/commands/mycommand.rs`
2. Use `#[command]` macro from Serenity
3. Export in `src/commands.rs`: `pub mod mycommand;`
4. Add to group in `main.rs`: `#[commands(multiply, ping, write, read, mycommand)]`

**Slash Command** (e.g., `/mycommand`):
1. Create `src/commands/mycommand.rs` with `register()` and `run()` functions
2. Export in `src/commands.rs`
3. Register in `main.rs` `ready()` handler
4. Add match arm in `interaction_create()` handler

### Adding New Handlers

1. Create `src/handlers/myhandler.rs` with async handler function
2. Export in `src/handlers.rs`: `pub mod myhandler;`
3. Import and call in `main.rs` `message()` event handler

## Testing

Tests are embedded in source files using `#[cfg(test)]`:

```bash
# Run all tests
cargo test --verbose

# Run specific test
cargo test test_name
```

**Existing test coverage**:
- `ming.rs`: URL extraction tests
- `twitter.rs`: Twitter URL pattern matching
- `epl_standing.rs`: Table formatting
- `epl_data_client.rs`: API integration test (requires network)

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`):
- **Triggers**: Push/PR to `master` branch
- **Environment**: ubuntu-latest
- **Steps**: Install Rust stable → Cache dependencies → Build → Test

## External Services

| Service | Purpose | Endpoint |
|---------|---------|----------|
| Discord API | Bot communication | Serenity-managed |
| OpenAI API | ChatGPT & DALL-E-3 | `https://api.openai.com/v1/` |
| EPL Data | Football standings | `https://epl-discord-bot.kalvin.workers.dev/standing` |
| Redis | State storage | Configured via `REDIS_DSL` |

## Internationalization

The bot supports multilingual responses:
- **Cantonese**: Default for casual questions
- **English**: For technical/serious questions
- **Japanese**: When user communicates in Japanese
- **Chinese**: EPL table headers use Chinese characters

## Common Tasks for AI Assistants

### Understanding the codebase
- Start with `main.rs` for the entry point and event routing
- Check `commands/` for user-facing functionality
- Check `handlers/` for automatic message processing
- Check `utils/openai.rs` for AI integration details

### Debugging
- Use `tracing` macros (`info!`, `error!`) for logging
- Check Redis connection if state-related issues occur
- Verify environment variables are set correctly

### Modifying OpenAI integration
- Primary file: `src/utils/openai.rs`
- Chat handler: `src/handlers/chat.rs`
- Image generation: `src/commands/imagine.rs`

### Working with Redis
- Client setup: `src/main.rs` lines 109-126
- Usage example: `src/handlers/ming.rs`, `src/handlers/chat.rs`
- TypeMapKey: `src/utils/redis_client.rs`
