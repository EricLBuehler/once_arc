use std::{sync::{Arc, Weak}, mem::{ManuallyDrop, MaybeUninit}, ops::Deref, fmt::Display};

#[deny(clippy::all)]
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OnceArc<T: ?Sized> {
    inner: ManuallyDrop<Arc<T>>
}

unsafe impl<T> Send for OnceArc<T> {}

unsafe impl<T> Sync for OnceArc<T> {}

impl<T: Display> Display for OnceArc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Arc::fmt(&self.inner, f)
    }
}

impl<T> OnceArc<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: ManuallyDrop::new(Arc::new(data))
        }
    }
    
    pub fn new_uninit() -> OnceArc<MaybeUninit<T>> {
        OnceArc {
            inner: ManuallyDrop::new(Arc::new(MaybeUninit::<T>::uninit()))
        }
    }
    
    pub fn new_cuclic(data_fn: impl FnOnce(&Weak<T>) -> T) -> OnceArc<T> {
        OnceArc {
            inner: ManuallyDrop::new(Arc::new_cyclic(data_fn))
        }
    }

    pub fn strong_count(this: &Self) -> usize {
        Arc::strong_count(&this.inner)
    }

    pub fn weak_count(this: &Self) -> usize {
        Arc::weak_count(&this.inner)
    }
}

impl<T: ?Sized> Deref for OnceArc<T> {
    type Target = Arc<T>;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}