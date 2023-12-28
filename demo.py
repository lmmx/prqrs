import priorityq
import random

pq = priorityq.PriorityQueue()

# Enqueue items
for _ in range(10):
    item = priorityq.Item(value=random.randint(0, 10), priority=random.randint(1, 10))
    pq.push(item)

# Benchmark
print(priorityq.benchmark_enqueue(pq, [priorityq.Item(value=random.randint(0, 10), priority=random.randint(1, 10)) for _ in range(10)]))
print(priorityq.benchmark_dequeue(pq))
