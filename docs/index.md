# Welcome to OxidLog

OxidLog is a fast, efficient command-line note-taking application written in Rust. It allows you to quickly capture, search, and manage your notes directly from the terminal.

## Features

- 📝 Quick note creation and management
- 🏷️ Tag-based organization
- 🔍 Powerful search capabilities
- ⚡ Fast and lightweight
- 🛠️ Configurable settings
- 💻 Command-line interface

## Quick Start

1. Install OxidLog by running the following command:

```bash
cargo install oxidlog
```

2. Initialize the OxidLog database:

```bash
oxidlog init
```

## Add a new note:

```bash
oxidlog add "This is my first note using OxidLog"
```

### Tagging
You can add tags to your notes by appending '#' followed by the tag name:

```bash
oxidlog add "This note has a tag #example"
```

## View all notes:

```bash
oxidlog view
```

### Filtering notes by tag:

```bash
oxidlog view --tags example
```

### View notes between specific dates:

```bash
oxidlog view --from 2024-10-06 --to 2024-11-06
```

You can also use them separately:

```bash
oxidlog view --from 2024-10-06
oxidlog view --to 2024-11-06
```

## Delete a note:

```bash
oxidlog delete <note_id>
```

## Update a note:

```bash
oxidlog update <note_id>
```

## Search notes:

```bash
oxidlog search "keyword"
```

### Optional flags

Tag filter:

```bash
oxidlog search "keyword" --tags example
```

Date filter:

```bash
oxidlog search "keyword" --from 2024-10-06 --to 2024-11-06
```

Case-sensitive search:

```bash
oxidlog search "keyword" --case-sensitive
```