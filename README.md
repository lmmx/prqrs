# prqrs

> Python access to a priority queue in Rust

The `prqrs` package provides a simple interface to a priority queue implemented in Rust. This
guide will show you how to create a priority queue, add items to it, and retrieve items based on
their priority.

## Installation

First, ensure that the prqrs package is installed.

```sh
pip install prqrs
```

To build the Rust project from source run:

```sh
maturin develop --release
```

## Usage

### Creating a Priority Queue

To create a priority queue, import the `PriorityQueue` class from the `prqrs` package and
instantiate it:

```
from prqrs import PriorityQueue

pq = PriorityQueue()
```

### Creating Items

Items that can be added to the priority queue need a value and a priority.
Import the `Item` class and create items:

```py
from prqrs import Item

item1 = Item(value=5, priority=3)  # An item with value 5 and priority 3
item2 = Item(value=10, priority=1) # An item with value 10 and priority 1
```

### Adding Items to the Queue

Add items to the queue using the push method:

```py
pq.push(item1)
pq.push(item2)
```

### Retrieving Items from the Queue

Retrieve items based on their priority using the `pop` method.
This method returns the item with the highest priority (lowest numerical value of priority):

```py
highest_priority_item = pq.pop()
if highest_priority_item is not None:
    print(f"Value: {highest_priority_item.value}, Priority: {highest_priority_item.priority}")
```

### Checking if the Queue is Empty

To check whether the priority queue is empty, use the `is_empty` method:

```py
if pq.is_empty():
    print("The priority queue is empty.")
else:
    print("The priority queue is not empty.")
```

### Example

Here is a complete example that demonstrates creating a priority queue, adding items, and retrieving
them:

```py
from prqrs import PriorityQueue, Item

# Create a priority queue
pq = PriorityQueue()

# Add items
pq.push(Item(value=5, priority=3))
pq.push(Item(value=10, priority=1))
pq.push(Item(value=7, priority=2))

# Retrieve and print items
while not pq.is_empty():
    item = pq.pop()
    print(f"Value: {item.value}, Priority: {item.priority}")
```

This will output

```
Value: 5, Priority: 3
Value: 7, Priority: 2
Value: 10, Priority: 1
```

## Benchmarking

Running `pdm install` (or `maturin develop`) to install the Rust wheel followed by  `python benchmark_1m.py`:

```
Enqueue time for 1 million items: 0.1798386573791504 seconds
Dequeue time for 1 million items: 0.5103330612182617 seconds
```
