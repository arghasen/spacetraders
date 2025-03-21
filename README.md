# Space Traders API Client

A terminal-based client for the [Space Traders API](https://spacetraders.io/), built in Rust.

## Features

- Terminal UI with tabbed interface
- View agent information
- View your ships
- Browse star systems
- (Coming soon) Market information

## Installation

### Prerequisites

- Rust and Cargo installed
- A Space Traders API token

### Setup

1. Clone the repository
2. Create a `.env` file in the root directory with your Space Traders API token:

```
SPACE_TRADERS_API_TOKEN=your_token_here
```

If you don't have a token yet, you can register a new agent at https://spacetraders.io/

## Running the Application

```bash
cargo run
```

## Navigation

- Use `Tab` key to switch between tabs
- Press `r` to refresh data
- Press `q` to quit

## Development

This project uses:

- `ratatui` for the terminal UI
- `crossterm` for terminal manipulation
- `reqwest` for API requests
- Official SpaceTraders API client

## License

MIT
