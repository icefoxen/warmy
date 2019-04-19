//! Shareable resources.

use std::sync::{Arc, Mutex, MutexGuard};

/// Shareable resource type.
///
/// Resources are wrapped in this type. You cannot do much with an object of this type, despite
/// borrowing immutable or mutably its content.
#[derive(Debug)]
pub struct Res<T>(Arc<Mutex<T>>);

impl<T> Clone for Res<T> {
  fn clone(&self) -> Self {
    Res(self.0.clone())
  }
}

impl<T> Res<T> {
  /// Wrap a value in a shareable resource.
  pub fn new(t: T) -> Self {
    Res(Arc::new(Mutex::new(t)))
  }

  /// Borrow a resource for as long as the return value lives.
    pub fn borrow(&self) -> MutexGuard<T> {
        let l = self.0.lock().unwrap();
        l
  }

  /// Mutably borrow a resource for as long as the return value lives.
    pub fn borrow_mut(&self) -> MutexGuard<T> {
    self.0.lock().unwrap()
  }
}
