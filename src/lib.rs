use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[pyclass]
pub struct PriorityQueue<T> {
    heap: BinaryHeap<Reverse<T>>,
}

#[pymethods]
impl<T: Ord + Copy + Clone + PyTypeInfo> PriorityQueue<T> {
    #[new]
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

#[pyclass]
pub struct Item {
    #[pyo3(get, set)]
    priority: i32,
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl Item {
    #[new]
    pub fn new(priority: i32, value: i32) -> Self {
        Item { priority, value }
    }
}

#[pyfunction]
fn create_priority_queue() -> PriorityQueue<Item> {
    PriorityQueue::new()
}

#[pymodule]
fn priorityq(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PriorityQueue<Item>>()?;
    m.add_function(wrap_pyfunction!(create_priority_queue, m)?)?;
    Ok(())
}
