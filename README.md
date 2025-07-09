# voxmix

A CLI tool for high-quality speech synthesis using VOICEVOX engine. Generate natural-sounding Japanese speech from text with customizable voice parameters.

## Features

- 🎤 **Text-to-speech synthesis** with high-quality VOICEVOX engine
- 🎭 **Multiple speaker support** with easy name-based selection
- ⚡ **HTTP API integration** for flexible deployment
- 🎛️ **Voice parameter customization** (speed, pitch, volume)
- 📊 **Progress indication** for long operations
- 🛡️ **Comprehensive error handling** and structured logging
- 🧪 **Full test coverage** with unit and integration tests

## Installation

### From Source

```bash
git clone https://github.com/your-username/voxmix
cd voxmix
cargo build --release
```

The binary will be available at `./target/release/voxmix`.

### Prerequisites

- Rust 1.70 or later
- VOICEVOX engine running on HTTP API mode

## VOICEVOX Setup

1. Download and install VOICEVOX from [https://voicevox.hiroshiba.jp/](https://voicevox.hiroshiba.jp/)
2. Start VOICEVOX application
3. Enable HTTP API mode (default: `http://127.0.0.1:50021`)

## Usage

### Basic Usage

```bash
# Simple text-to-speech (uses default speaker "ずんだもん")
voxmix say "こんにちは"

# Specify output file
voxmix say --output greeting.wav "おはようございます"

# Use different speaker
voxmix say --speaker "四国めたん" "こんばんは"
```

### Voice Parameter Customization

```bash
# Adjust speech speed (0.5-2.0 range)
voxmix say --speed 1.2 "速めに話します"

# Adjust pitch (0.5-2.0 range, 1.0 = default)
voxmix say --pitch 0.8 "低めの声で話します"

# Adjust volume (0.5-2.0 range)
voxmix say --volume 1.5 "大きな声で話します"

# Combine parameters
voxmix say --speaker "ずんだもん" --speed 1.1 --pitch 0.9 --volume 1.2 --output custom.wav "カスタム設定で話します"
```

### Speaker Selection

```bash
# Use speaker name
voxmix say --speaker "四国めたん" "話者名で指定"

# Use speaker with style
voxmix say --speaker "四国めたん（あまあま）" "スタイル付き話者"

# Use speaker ID directly
voxmix say --speaker "1" "話者IDで指定"
```

### Server Configuration

```bash
# Use different VOICEVOX server
voxmix say --host 192.168.1.100 --port 50021 --speaker "ずんだもん" "リモートサーバーを使用"

# Use custom port
voxmix say --port 50022 "カスタムポートを使用"
```

## Command Line Options

### `say` Subcommand

| Option | Description | Default | Range |
|--------|-------------|---------|-------|
| `<TEXT>` | Text to synthesize | Required | - |
| `--speaker` | Speaker name or ID | "ずんだもん" | - |
| `--output, -o` | Output file path | "out.wav" | - |
| `--speed` | Speech speed | 1.0 | 0.5-2.0 |
| `--pitch` | Voice pitch multiplier | 1.0 | 0.5-2.0 |
| `--volume` | Voice volume | 1.0 | 0.5-2.0 |
| `--host` | VOICEVOX server host | "127.0.0.1" | - |
| `--port` | VOICEVOX server port | 50021 | - |

### Examples with Parameters

```bash
# Natural speech
voxmix say "自然な話し方です"

# Fast speech
voxmix say --speed 1.5 "速い話し方です"

# Slow speech
voxmix say --speed 0.7 "ゆっくりとした話し方です"

# High pitch
voxmix say --pitch 1.3 "高い声で話します"

# Low pitch
voxmix say --pitch 0.7 "低い声で話します"

# Loud volume
voxmix say --volume 1.8 "大きな声で話します"

# Quiet volume
voxmix say --volume 0.6 "小さな声で話します"
```

## Available Speakers

Common speakers include:
- ずんだもん (default)
- 四国めたん
- 春日部つむぎ
- 雨晴はう
- 波音リツ
- 玄野武宏
- 白上虎太郎
- 青山龍星
- 冥鳴ひまり
- 九州そら

Use `--speaker` with the exact name or check available speakers by running the tool with an invalid speaker name.

## Error Handling

voxmix provides detailed error messages:

```bash
# Invalid parameters
voxmix say --speed 0 "test"
# Error: Speed must be greater than 0

# Server connection issues
voxmix say --port 12345 "test"
# Error: HTTP request failed: Connection refused

# Invalid speaker
voxmix say --speaker "invalid" "test"
# Error: Speaker 'invalid' not found
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=info cargo run -- say "テスト"
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run specific test
cargo test test_client_creation
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test integration_tests
```

## Architecture

- **CLI Layer**: Command-line argument parsing with `clap`
- **HTTP Client**: VOICEVOX API integration with `reqwest`
- **Audio Processing**: Direct WAV file output
- **Error Handling**: Custom error types with `thiserror`
- **Logging**: Structured logging with `tracing`
- **Testing**: Unit and integration tests with `assert_cmd`

## API Integration

voxmix integrates with VOICEVOX HTTP API:

1. **Speaker Resolution**: `/speakers` endpoint to resolve speaker names to IDs
2. **Audio Query**: `/audio_query` endpoint to generate synthesis parameters
3. **Audio Synthesis**: `/synthesis` endpoint to generate WAV audio data

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Ensure VOICEVOX application is running
   - Check if HTTP API is enabled
   - Verify host and port settings

2. **Speaker Not Found**
   - Check speaker name spelling
   - Use exact speaker names (case-sensitive)
   - Try using speaker ID instead

3. **Invalid Parameters**
   - Speed, pitch, volume must be > 0
   - Text cannot be empty
   - Output path must be writable

### Logging

Enable detailed logging:

```bash
RUST_LOG=debug voxmix say "デバッグログ付きで実行"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run code formatting (`cargo fmt`)
7. Run linter (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- [VOICEVOX](https://voicevox.hiroshiba.jp/) for the excellent text-to-speech engine
- All the voice actors who provided the character voices