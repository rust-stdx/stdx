//! Resource-usage regression tests: bounded allocations and no leaks.
//!
//! This test binary installs a counting global allocator. Keep allocator-sensitive
//! assertions in a single test so they are not perturbed by parallel tests.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
};

use template::{Engine, EscapeMode, context};

static ALLOC_CALLS: AtomicUsize = AtomicUsize::new(0);
static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_CALLS.fetch_add(1, Ordering::Relaxed);
        LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_CALLS.fetch_add(1, Ordering::Relaxed);
        LIVE_BYTES.fetch_add(new_size, Ordering::Relaxed);
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn build_engine() -> Engine {
    let mut engine = Engine::new(EscapeMode::Html);
    engine
        .add_template("base", "<html><body>{% block content %}{% endblock %}</body></html>")
        .unwrap();
    engine
        .add_template(
            "page",
            "{% extends \"base\" %}{% block content %}\
             {% for item in items %}<li>{{ item }}</li>{% endfor %}\
             {% endblock %}",
        )
        .unwrap();
    engine
}

#[test]
fn allocations_are_bounded_and_do_not_leak() {
    let engine = build_engine();
    let items: Vec<String> = (0..1000).map(|i| format!("item-{i}")).collect();

    // Warm up so one-time allocations are excluded from the per-render budget.
    for _ in 0..20 {
        let _ = engine.render("page", context! { items: items.clone() }).unwrap();
    }

    let before_calls = ALLOC_CALLS.load(Ordering::Relaxed);
    let output = engine.render("page", context! { items: items.clone() }).unwrap();
    let per_render = ALLOC_CALLS.load(Ordering::Relaxed) - before_calls;
    assert!(output.contains("item-999"));

    // The previous implementation performed ~4 allocations per loop iteration
    // (a scope map per item plus two temporary strings per interpolation). Guard
    // against regressions with a loose but meaningful bound.
    assert!(per_render < 3000, "engine allocated {per_render} times for a 1000-item render");

    let live_before = LIVE_BYTES.load(Ordering::Relaxed);
    for _ in 0..2000 {
        let _ = engine.render("page", context! { items: items.clone() }).unwrap();
    }
    let live_after = LIVE_BYTES.load(Ordering::Relaxed);

    // Live bytes must not grow meaningfully over repeated renders.
    assert!(
        live_after <= live_before + 256 * 1024,
        "live bytes grew from {live_before} to {live_after} over 2000 renders (possible leak)"
    );
}
