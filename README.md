# OxidLog - A Personal Knowledge Manager

This is a powerful tool that allows you to take notes, todos, or even journal!
With many different features you can make this tool what you need it to be.
The only limit is.. text!!

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

## Learn more
Use the 'help' command to explore all available options
```bash
xlog help
```
Or view all options for a specific command
```bash
xlog help add
```

For more information, visit the documentation at [COMING SOON];
