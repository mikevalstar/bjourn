# blist

A commnd line Bullet List tool

## Features

### Add

Add a new bullet point to the list

```bash
blist add This is a new bullet point
```

### List

You can view the day's bullet points by specifying the day or view today's bullet points without specifying a day.

```bash
blist
```

```bash
blist list 2025-01-01
```

## Development Notes

Run:

```bash
cargo run
```

Run with arguments:

```bash
cargo run -- add This is a new bullet point
```

Debug Mode

```bash
DEBUG=true cargo run
```
