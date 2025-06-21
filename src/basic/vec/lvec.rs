use std::fmt::Debug;

// 节点
#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem: elem,
            next: None,
        }
    }
}

// 链表 Vec
#[derive(Debug)]
pub struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    pub fn push(&mut self, elem: T) {
        let node = Node::new(elem);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();

            // 找到链表最后一个节点
            for _i in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 在最后一个节点后插入新数据
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    // 栈末尾加入新的 LVec
    pub fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear();
    }

    pub fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size {
            index = self.size;
        }

        // 分三种情况插入新节点
        let mut node = Node::new(elem);
        if self.is_empty() {
            // LVec 为空
            self.head = Some(Box::new(node));
        } else if index == 0 {
            // 插入链表首部
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {
            // 插入链表中间
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                // 找到插入位置
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size < 1 {
            return None;
        } else {
            self.remove(self.size - 1)
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        // 分两种情况删除节点，首节点删除最好处理
        let mut node;
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            // 非首节点需要找到待删除节点，并处理前后链接
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;

        Some(node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    // 打印 LVec
    pub fn print_lvec(&self) {
        if 0 == self.size {
            println!("Empty lvec");
        }

        for item in self.iter() {
            println!("{:?}", item);
        }
    }
}

// 实现三种迭代功能
pub struct IntoIter<T: Copy + Debug>(LVec<T>);
impl<T: Copy + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
