/// Deduplicate concurrent async function calls by key.
///
/// A [`Group`] ensures that only one execution is in-flight for a given key
/// at a time. Duplicate callers wait for the original to finish and receive
/// the same result. This is useful for cache stampede prevention, expensive
/// lookups, or any scenario where concurrent identical work should be
/// coalesced into a single operation.
///
/// # Example
///
/// ```rust
/// use singleflight::Group;
///
/// # async fn example() {
/// let group = Group::<String, String>::new();
///
/// let (tx, mut rx) = tokio::sync::mpsc::channel(16);
///
/// for _ in 0..10 {
///     let group = group.clone();
///     let mut tx = tx.clone();
///     tokio::spawn(async move {
///         let call = group
///             .call_async("key".to_string(), || async {
///                 tokio::time::sleep(std::time::Duration::from_millis(10)).await;
///                 "computed".to_string()
///             })
///             .await;
///         tx.send(call.shared).await.unwrap();
///     });
/// }
/// drop(tx);
///
/// // Exactly one caller was the original (shared = false), the rest are duplicates.
/// let mut originals = 0;
/// let mut duplicates = 0;
/// while let Some(shared) = rx.recv().await {
///     if shared { duplicates += 1; } else { originals += 1; }
/// }
/// assert_eq!(originals, 1);
/// assert_eq!(duplicates, 9);
/// # }
/// ```
use std::collections::HashMap;
use std::{
    future::Future,
    hash::Hash,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use tokio::sync::Notify;

/// The result of a singleflight call.
///
/// `val` is the value produced by the executed function.
/// `shared` is `true` when this caller received a result that was computed
/// by another caller rather than executing the function itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<V> {
    pub val: V,
    pub shared: bool,
}

struct Entry<K, V> {
    key: K,
    val: Mutex<Option<V>>,
    notify: Notify,
    dups: AtomicUsize,
}

/// A `Group` represents a namespace in which work can be deduplicated by key.
///
/// Only one caller per key executes the function; concurrent callers for the
/// same key receive the same result once it is ready.
///
/// # Panics
///
/// The function passed to [`call_async`][Group::call_async] **must not panic**.
/// If it panics, all waiting callers will hang forever because the value will
/// never be stored and they will never be notified.
pub struct Group<K, V> {
    inner: Arc<Mutex<HashMap<K, Arc<Entry<K, V>>>>>,
}

// --- inherent impls independent of Eq/Hash ----------------------------------

impl<K, V> Group<K, V> {
    /// Creates a new empty `Group`.
    pub fn new() -> Self {
        Group {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<K, V> Default for Group<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// --- Clone ------------------------------------------------------------------

impl<K, V> Clone for Group<K, V> {
    fn clone(&self) -> Self {
        Group {
            inner: self.inner.clone(),
        }
    }
}

// --- API requiring Eq + Hash on K -------------------------------------------

impl<K, V> Group<K, V>
where
    K: Eq + Hash,
{
    /// Tells the `Group` to forget about a key. Future calls to [`call_async`]
    /// for this key will execute the function rather than waiting for an
    /// earlier call to complete.
    ///
    /// If no call is in-flight for the given key, this is a no-op.
    pub fn forget(&self, key: &K) {
        let _ = self.inner.lock().unwrap().remove(key);
    }
}

// --- call_async -------------------------------------------------------------

impl<K, V> Group<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send,
{
    /// Executes `f` for the given `key`, coalescing duplicate callers.
    ///
    /// If no other call is in-flight for `key`, this call executes `f` and
    /// returns its result with `shared = false`.
    ///
    /// If another call for the same key is already in-flight, this call
    /// awaits that call's completion and returns the same result with
    /// `shared = true`.
    ///
    /// # Panics
    ///
    /// `f` **must not panic**. See the [type-level docs][Group#panics].
    pub async fn call_async<F, Fut>(&self, key: K, f: F) -> Call<V>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = V> + Send,
    {
        // Scoped map access so the MutexGuard is dropped before any await.
        let existing = {
            let map = self.inner.lock().unwrap();
            map.get(&key).cloned()
        };

        if let Some(entry) = existing {
            entry.dups.fetch_add(1, Ordering::Relaxed);
            loop {
                let notified = entry.notify.notified();
                if let Some(val) = entry.val.lock().unwrap().clone() {
                    return Call {
                        val,
                        shared: true,
                    };
                }
                notified.await;
            }
        }

        // First caller – store key inside the entry so we don't hold `key`
        // across the await point (avoids requiring `'static` on K).
        let entry = Arc::new(Entry {
            key, // moved into the entry
            val: Mutex::new(None),
            notify: Notify::new(),
            dups: AtomicUsize::new(0),
        });

        // Insert into the map (scoped).
        {
            let mut map = self.inner.lock().unwrap();
            map.insert(entry.key.clone(), entry.clone());
        }

        // **Must not panic.** If this panics, waiters block forever.
        let result = f().await;

        // Store the result and wake every waiter.
        *entry.val.lock().unwrap() = Some(result.clone());
        entry.notify.notify_waiters();

        // Remove the entry from the map (scoped).
        {
            let mut map = self.inner.lock().unwrap();
            map.remove(&entry.key);
        }

        Call {
            val: result,
            shared: false,
        }
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn single_caller_is_not_shared() {
        let group = Group::<&str, u32>::new();
        let call = group.call_async("a", || async { 42 }).await;
        assert_eq!(call.val, 42);
        assert!(!call.shared);
    }

    #[tokio::test]
    async fn duplicate_callers_share_result() {
        let group = Group::<String, String>::new();

        let barrier = Arc::new(tokio::sync::Barrier::new(10));
        let started = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for _ in 0..10 {
            let group = group.clone();
            let barrier = barrier.clone();
            let started = started.clone();
            handles.push(tokio::spawn(async move {
                barrier.wait().await;

                group
                    .call_async("key".to_string(), || async {
                        started.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                        "hello".to_string()
                    })
                    .await
            }));
        }

        let mut originals = 0;
        let mut duplicates = 0;
        for h in handles {
            let call = h.await.unwrap();
            assert_eq!(call.val, "hello");
            if call.shared {
                duplicates += 1;
            } else {
                originals += 1;
            }
        }

        assert_eq!(originals, 1);
        assert_eq!(duplicates, 9);
        // The function should have only executed once.
        assert_eq!(started.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn different_keys_dont_interfere() {
        let group = Group::<String, u32>::new();

        let (tx, mut rx) = tokio::sync::mpsc::channel(8);

        for k in ["a", "b"] {
            let group = group.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                let call = group
                    .call_async(k.to_string(), || async {
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                        1
                    })
                    .await;
                tx.send((k.to_string(), call.shared)).await.unwrap();
            });
        }
        drop(tx);

        let mut results = HashMap::new();
        while let Some((k, shared)) = rx.recv().await {
            results.entry(k).or_insert(Vec::new()).push(shared);
        }

        assert_eq!(results.len(), 2);
        for shareds in results.values() {
            assert_eq!(shareds.len(), 1);
            assert!(!shareds[0]); // each was the original for its key
        }
    }

    #[tokio::test]
    async fn forget_stops_dedup() {
        let group = Group::<&str, u32>::new();

        let first = group
            .call_async("k", || async {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                1
            })
            .await;
        assert_eq!(first.val, 1);
        assert!(!first.shared);

        group.forget(&"k");

        let second = group
            .call_async("k", || async {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                2
            })
            .await;
        assert_eq!(second.val, 2);
        assert!(!second.shared);
    }

    #[tokio::test]
    async fn forget_unknown_key_is_noop() {
        let group = Group::<&str, u32>::new();
        group.forget(&"nonexistent");

        let call = group.call_async("k", || async { 7 }).await;
        assert_eq!(call.val, 7);
    }

    #[tokio::test]
    async fn call_after_completion_starts_fresh() {
        let group = Group::<&str, u32>::new();

        let counter = Arc::new(AtomicUsize::new(0));

        let c = counter.clone();
        let first = group
            .call_async("k", || async {
                c.fetch_add(1, Ordering::SeqCst);
                10
            })
            .await;
        assert_eq!(first.val, 10);

        // Wait a tiny bit so the completion has propagated.
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        // At this point the entry has been removed from the map.

        let c = counter.clone();
        let second = group
            .call_async("k", || async {
                c.fetch_add(1, Ordering::SeqCst);
                20
            })
            .await;
        assert_eq!(second.val, 20);
        assert!(!second.shared);

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn high_concurrency_stress() {
        let group = Group::<String, usize>::new();
        let n = 200_usize;
        let started = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::new();
        for i in 0..n {
            let group = group.clone();
            let started = started.clone();
            handles.push(tokio::spawn(async move {
                let key = (i % 10).to_string();

                group
                    .call_async(key, || async {
                        started.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(std::time::Duration::from_micros(100)).await;
                        i
                    })
                    .await
            }));
        }

        for h in handles {
            // val is the `i` of whichever caller won the race for that key.
            // All 20 callers for the same key-mod-10 share the same result.
            let _call = h.await.unwrap();
        }

        // At most 10 executions (one per key), but could be fewer if some
        // hadn't started when the first for that key finished.
        assert!(started.load(Ordering::SeqCst) <= 10);
    }

    /// Ensures that forgetting a key while a call is in-flight doesn't
    /// hang the original caller (they hold an `Arc`), and a subsequent
    /// call for the same key starts a fresh execution.
    #[tokio::test]
    async fn forget_during_flight_lets_new_callers_start_fresh() {
        let group = Group::<&str, u32>::new();
        let run_count = Arc::new(AtomicUsize::new(0));

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let r1 = run_count.clone();

        let group_for_spawn = group.clone();
        let h1 = tokio::spawn(async move {
            group_for_spawn
                .call_async("k", || async {
                    r1.fetch_add(1, Ordering::SeqCst);
                    let _ = tx.send(());
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    42
                })
                .await
        });

        // Wait until the first caller has started but not finished.
        rx.await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        assert_eq!(run_count.load(Ordering::SeqCst), 1);

        // Forget the key while the call is in-flight.
        group.forget(&"k");

        // Start a second caller for the same key after forget.
        // It should execute its own function rather than waiting.
        let r2 = run_count.clone();
        let group_for_spawn2 = group.clone();
        let h2 = tokio::spawn(async move {
            group_for_spawn2
                .call_async("k", || async {
                    r2.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    99
                })
                .await
        });

        let c1 = h1.await.unwrap();
        let c2 = h2.await.unwrap();

        // Both functions executed independently.
        assert_eq!(run_count.load(Ordering::SeqCst), 2);

        assert_eq!(c1.val, 42);
        assert!(!c1.shared);

        assert_eq!(c2.val, 99);
        assert!(!c2.shared);
    }
}
