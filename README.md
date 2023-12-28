# priorityq

> Python access to a priority queue in Rust

_priorityq_ is a Python extension written using PyO3 to benchmark the speed of a priority
queue in Rust.

## Installation

```sh
maturin develop --release
```

## Usage

```py
from priorityq import create_priority_queue

q = create_priority_queue()
```

## Benchmarking

Running `pdm install` (or `maturin develop`) to install the Rust wheel followed by  `python benchmark_1m.py`:

```
Enqueue time for 1 million items: 0.1798386573791504 seconds
Dequeue time for 1 million items: 0.5103330612182617 seconds
```
