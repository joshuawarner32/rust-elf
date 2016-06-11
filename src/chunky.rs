use std::marker::PhantomData;
use std::cell::{UnsafeCell};
use std::cmp::max;
use std::mem::{swap, size_of, transmute};

pub struct Builder<'longer_than_self> {
    head: UnsafeCell<Vec<u8>>,
    chunks: UnsafeCell<Vec<Vec<u8>>>,
    _marker: PhantomData<*mut &'longer_than_self()>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Self::with_capacity(1024)
    }

    pub fn with_capacity(cap: usize) -> Builder<'a> {
        Builder {
            head: UnsafeCell::new(Vec::with_capacity(cap)),
            chunks: UnsafeCell::new(Vec::new()),
            _marker: PhantomData
        }
    }

    fn ensure(&self, cap: usize) {
        unsafe {
            let head = &mut *self.head.get();
            if head.capacity() < head.len() + cap {
                let new_len = max(cap, 2 * head.capacity());
                let mut vec = Vec::with_capacity(new_len);
                swap(&mut vec, head);
                (&mut *self.chunks.get()).push(vec);
            }
        }
    }

    pub fn alloc<T>(&self) -> &'a mut T {
        let len = size_of::<T>();
        self.ensure(len);
        unsafe {
            let head = &mut *self.head.get();
            let old_len = head.len();
            head.set_len(old_len + len);
            transmute((&mut head[old_len..]).as_mut_ptr())
        }
    }
}

#[cfg(test)]
mod test {
    use ::chunky::Builder;
    use std::mem::transmute;
    use std::collections::HashSet;

    #[test]
    fn test_lots_o_emptyness() {
        let b = Builder::new();

        let first: usize = b.alloc::<()>() as *const () as usize;

        for _ in 0..1000 {
            let a = b.alloc::<()>();
            let addr: usize = a as *const () as usize;

            assert!(first == addr);
        }
    }

    #[test]
    fn test_ints() {
        let b = Builder::new();
        let mut set = HashSet::new();

        for _ in 0..1000 {
            let a = b.alloc::<u32>();
            let addr: usize = a as *const u32 as usize;

            assert!(!set.contains(&addr));
            set.insert(addr);
        }
    }
}
