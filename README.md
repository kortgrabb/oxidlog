# Jot

A simple command-line journal application written in Rust.

## Features

- Initialize a new journal
- Add entries
- Remove entries
- View all entries
- Edit existing entries
- Search through entries

## Installation

Make sure you have Rust installed on your system. Then:

```bash
cargo install jot
```

Or, if you want to build from source:

```bash
git clone https://github.com/kortgrabb/jothttps://github.com/
cd jot
cargo build --release
```
## Basic Usage
Initialize a new journal:
```bash
jot init
```
Add an entry:
```bash
jot add "My second journal in Jot!"
```
View all entries:
```bash
jot view
```
Edit an entry:
```bash
jot edit 0
```
Enter new body: "My ~~second~~ first journal in Jot!"
Remove an entry:
```bash
jot remove 0
```
Search through entries:
```bash
jot search "first"
```