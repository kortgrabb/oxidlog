# OxidLog - A Powerful Terminal Journal

Transform your terminal into a personal journaling space with OxidLog. This lightweight CLI tool makes daily logging, note-taking, and task management seamless through a simple yet powerful interface.

## Quick Start

```bash
# Install OxidLog
cargo install oxidlog

# Initialize your journal
xlog init

# Create your first entry
xlog add "Starting my journey with OxidLog"
```

## Key Features

### Smart Tagging
Organize entries with hashtags
```bash
xlog add "Meeting notes #work #important"
```

### Easy Retrieval
View all entries with a simple command
```bash
xlog view
```

### Custom Filters
Filter entries by tags
```bash
xlog view --tags "work"
```
Or by specifying a timeframe
```bash
xlog view --from "2021-01-01" --to "2021-12-31"
```

### Search Entries
Find specific entries using keywords
```bash
xlog search "meeting" --tags "important"
```

## Getting Started

Run `xlog help` to see available commands or check our documentation [COMING SOON].
