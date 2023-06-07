use std::ptr::NonNull;

type Node = Option<NonNull<Vec<NonNull<Edge>>>>;

/// # info
/// ES | EF | TF
/// EF | LF | FF
#[derive(Debug)]
struct Edge {
    next: Node,
    prev: Node,
    cost: Cost,
    es: Moment,
    ef: Moment,
    ls: Moment,
    lf: Moment,
    tf: Moment,
    ff: Moment,
}

#[derive(Debug, Clone, Copy)]
enum Moment {
    Moment(usize),
    Unknown,
}

#[derive(Debug)]
enum Cost {
    Duration(usize),
    Unknown,
}

impl Edge {
    fn new(cost: usize) -> Self {
        Self {
            next: None,
            prev: None,
            cost: Cost::Duration(cost),
            es: Moment::Unknown,
            ef: Moment::Unknown,
            ls: Moment::Unknown,
            lf: Moment::Unknown,
            tf: Moment::Unknown,
            ff: Moment::Unknown,
        }
    }

    fn set_es(&mut self, num: usize) {
        self.es = Moment::Moment(num);
        if let Cost::Duration(dur) = self.cost {
            self.ef = Moment::Moment(num + dur);
        }
    }

    fn set_lf(&mut self, num: usize) {
        self.lf = Moment::Moment(num);
        if let Cost::Duration(dur) = self.cost {
            self.ls = Moment::Moment(num - dur);
        }
    }

    fn add_next(&mut self, next: &mut Edge) {
        let next_ptr = NonNull::new(next as *mut _).unwrap();
        match self.next {
            Some(nexts) => unsafe {
                if (*nexts.as_ptr()).contains(&next_ptr) {
                    return;
                };
                (*nexts.as_ptr()).push(next_ptr);
            },
            None => {
                let new_vec = NonNull::new(Box::into_raw(Box::new(vec![next_ptr]))).unwrap();
                self.next = Some(new_vec);
            }
        }
        next.add_prev(self);
    }

    fn add_prev(&mut self, prev: &mut Edge) {
        let prev_ptr = NonNull::new(prev as *mut _).unwrap();
        match self.prev {
            Some(prevs) => unsafe {
                if (*prevs.as_ptr()).contains(&prev_ptr) {
                    return;
                };
                (*prevs.as_ptr()).push(prev_ptr);
            },
            None => {
                let new_vec = NonNull::new(Box::into_raw(Box::new(vec![prev_ptr]))).unwrap();
                self.prev = Some(new_vec);
            }
        }
        prev.add_next(self);
    }
}

impl Drop for Edge {
    fn drop(&mut self) {
        // The memory allocated on the heap through a raw pointer
        // will not be deallocated automatically when leaving the scope.
        // We need to use Box::from_raw() to transfer the raw pointer to a Box,
        // so that Rust can automatically deallocate its memory.
        // By the way, the raw pointer itself would not be deallocated either,
        // After using Box::from_raw(), the raw pointer will be owned by the result Box,
        // with which the pointer and memery will be deallocated.
        let ptr = self.next.take();
        match ptr {
            Some(ptr) => unsafe {
                let _ = Box::from_raw(ptr.as_ptr());
            },
            None => {}
        }
        let ptr = self.prev.take();
        match ptr {
            Some(ptr) => unsafe {
                let _ = Box::from_raw(ptr.as_ptr());
            },
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo +nightly miri test -- double_code_arrow_diagram

    #[test]
    fn test1() {
        let mut edge1 = Edge::new(2);
        let mut edge2 = Edge::new(2);
        edge1.add_next(&mut edge2);
    }
}
