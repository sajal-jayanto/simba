# Docker Stats Dashboard

A lightweight, real-time Docker container monitoring dashboard built with **Rust**. It reads live container statistics from `docker stats` and displays CPU usage, memory usage, and process IDs in a terminal-based dashboard.

## Features

- **Live monitoring** — continuously reads Docker container statistics.
- **CPU usage** — displays container CPU utilization with a progress bar.
- **Memory usage** — shows used and total memory with a usage percentage.
- **Process monitoring** — displays the number of processes (PIDs) associated with each container.
- **Multiple containers** — tracks multiple running containers in a single terminal dashboard.
- **Container status tracking** — removes containers from the dashboard when they are no longer running.
- **Adaptive terminal width** — uses the terminal dimensions to determine dashboard width.
- **Lightweight implementation** — built in Rust using the Docker CLI.

## Preview

```text
╔═══ my-container ═════════════════════════════════════════════╗
║                                                              ║
║  CPU:[####################--------------------] 52.45%       ║
║  Ram:[####------------------------------------] 10.25%       ║
║  Pids: 12                                                    ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

*The preview is illustrative. Actual layout and values depend on terminal width and container statistics.*

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- Docker Engine or Docker Desktop
- Docker CLI available in your system's `PATH`
- Permission to access the Docker daemon
- A terminal that supports ANSI escape sequences

## Installation

### 1. Clone the repository

```bash
git clone <your-repository-url>
cd <your-project-directory>
```

Replace the placeholders with your repository URL and project directory.

### 2. Install dependencies

The project uses the following Rust crates:

- [`crossterm`](https://crates.io/crates/crossterm) — terminal dimensions
- [`regex`](https://crates.io/crates/regex) — ANSI escape-sequence cleanup

Add them to your `Cargo.toml`:

```toml
[dependencies]
crossterm = "0.29"
regex = "1"
```

### 3. Run the dashboard

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

## How It Works

1. Starts `docker stats` as a child process.
2. Reads its standard output line by line.
3. Parses container IDs, names, CPU percentages, memory usage, and PIDs.
4. Stores the latest statistics in a `HashMap`, using container IDs as keys.
5. Checks whether tracked containers are still running and removes stopped containers.
6. Calculates CPU and memory usage percentages.
7. Renders a terminal dashboard with progress bars.
8. Refreshes the display as new statistics arrive.

## Memory Usage Calculation

Docker reports memory usage in a format such as:

```text
35.25MiB / 6.651GiB
```

The application converts MiB and GiB values to MiB before calculating the percentage:

\[
\text{Memory Usage (\%)} =
\frac{\text{Used Memory}}{\text{Total Memory}} \times 100
\]

The displayed percentage is rounded to two decimal places.

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

- `main.rs` — launches Docker statistics collection, parses container information, tracks container state, and renders the dashboard.
- `Cargo.toml` — declares project metadata and dependencies.
- `Cargo.lock` — records resolved dependency versions for reproducible builds.

## Troubleshooting

### Docker command fails

Check that Docker is installed and running:

```bash
docker version
docker ps
```

### Permission denied

On Linux, ensure your user has permission to access the Docker daemon. Follow your system's Docker installation guidance.

### Dashboard does not update

The dashboard depends on the output stream from `docker stats`. Confirm that Docker is running and that containers are available:

```bash
docker stats
```

### Terminal layout looks incorrect

Use a terminal that supports ANSI escape sequences and resize it to provide sufficient horizontal space.

## Limitations

- Requires Docker CLI access; it does not connect directly to the Docker Engine API.
- Uses a terminal-based interface rather than a graphical UI.
- Container status checks invoke `docker inspect`, which adds overhead when monitoring many containers.
- Terminal resizing and Unicode character display can vary by terminal emulator.
- Memory parsing currently supports `MiB` and `GiB` units only.

## Future Improvements

- [ ] Support KiB, B, TiB, and other memory units.
- [ ] Improve terminal resizing and redraw behavior.
- [ ] Add network I/O and block I/O metrics.
- [ ] Add container filtering and sorting.
- [ ] Improve handling of containers starting and stopping.
- [ ] Reduce repeated Docker CLI calls.
- [ ] Add configurable refresh intervals.
- [ ] Add CPU and memory threshold warnings.

## License

Choose a license for your project, such as the MIT License, and include the corresponding `LICENSE` file if you publish it.

---

Built with Rust and Docker.
