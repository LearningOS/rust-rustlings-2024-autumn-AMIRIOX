/*
    heap
    This question requires you to implement a binary heap function
*/

// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Display;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;

        let mut cur = self.count;
        while cur > 1 {
            let cur_item = &self.items[cur - 1];

            let par_idx = self.parent_idx(cur);
            let par_item = &self.items[par_idx - 1];

            // 向父节点比较，如果满足比较器就应该向上浮动
            // 否则位置合法，不需要调整
            if (self.comparator)(cur_item, par_item)  {
                println!("swap: {} and {}", cur, par_idx);
                self.items.swap(cur - 1, par_idx - 1);
                cur = par_idx;
            }else {
                println!("no need to swap: {} and {}", cur, par_idx);
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /*
    fn smallest_child_idx(&self, idx: usize) -> usize {
        0
    }
    */
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 将根节点删除并将末位节点置换过来
        // root 存储根节点值
        let root = self.items.swap_remove(0);
        self.count -= 1;

        let mut cur = 1;
        
        loop {
            let left_child = self.left_child_idx(cur);
            let right_child = self.right_child_idx(cur);
            let mut bigger = cur;
            // 如果子节点有更符合比较器的就置换
            if left_child <= self.items.len() && (self.comparator)(&self.items[left_child - 1], &self.items[bigger - 1]) {
                bigger = left_child;
            }
            if right_child <= self.items.len() && (self.comparator)(&self.items[right_child - 1], &self.items[bigger - 1]) {
                bigger = right_child;
            }

            // 不能再被更改，位置合法
            if bigger == cur {
                break;
            }

            self.items.swap(cur - 1, bigger - 1);
            cur = bigger;
        }

        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}

