# llm-mock

A mock OpenAI-compatible LLM API server built with Rust and Axum. Useful for testing LLM integrations without calling real APIs.

## Features

- **OpenAI-compatible API** - Drop-in replacement for `/v1/chat/completions`, `/v1/completions`, `/v1/embeddings`, `/v1/models`
- **Streaming support** - SSE streaming with configurable token delays
- **Error simulation** - Configurable rate limiting and timeout errors
- **Simple config** - TOML-based configuration

## Quick Start

```bash
cargo run
```

Server starts on `0.0.0.0:8080` by default.

## Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

Binary output: `target/release/llm-mock`

## Cross-Compile

```bash
# Add targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for Windows (requires mingw-w64)
cargo build --release --target x86_64-pc-windows-gnu

# Build for Mac (requires osxcross)
cargo build --release --target x86_64-apple-darwin
```

## CI/CD

GitHub Actions 自动构建多平台产物：

- 推送代码：构建并上传 artifacts
- 推送 `v*` 标签：自动创建 Release 并上传二进制文件

```bash
# 创建 Release
git tag v1.0.0
git push origin v1.0.0
```

产物列表：
- `llm-mock-linux-amd64` - Linux x86_64
- `llm-mock-windows-amd64.exe` - Windows x86_64
- `llm-mock-macos-amd64` - macOS Intel
- `llm-mock-macos-arm64` - macOS Apple Silicon

## Configuration

Edit `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080

[response]
log_file = "mock-llm-resp.log"
stream_delay_min_ms = 10
stream_delay_max_ms = 50

[error_simulation]
enabled = true
rate_limit_probability = 0.0
timeout_probability = 0.0
```

The server returns the contents of `log_file` as mock responses.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/models` | List available models |
| POST | `/v1/chat/completions` | Chat completions (streaming supported) |
| POST | `/v1/completions` | Text completions (streaming supported) |
| POST | `/v1/embeddings` | Create embeddings |

## Testing

```bash
cargo test
```

## Example Usage

```bash
# List models
curl http://localhost:8080/v1/models

# Chat completion
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello"}]}'

# Streaming chat
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello"}], "stream": true}'

# Embeddings
curl -X POST http://localhost:8080/v1/embeddings \
  -H "Content-Type: application/json" \
  -d '{"input": "Hello world", "model": "text-embedding-ada-002"}'
```

## Error Simulation

Enable probabilistic errors in `config.toml`:

```toml
[error_simulation]
enabled = true
rate_limit_probability = 0.1   # 10% chance of 429
timeout_probability = 0.05     # 5% chance of 504
```
