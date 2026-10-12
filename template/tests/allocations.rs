//! Resource-usage regression tests: bounded allocations and no leaks.
//!
//! Counting comes from `alloc_counter`, which attributes allocations to the
//! current thread, so these assertions are not perturbed by parallel tests.

use alloc_counter::counters;
use template::{Engine, EscapeMode, context};

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

    let before = counters();
    let output = engine.render("page", context! { items: items.clone() }).unwrap();
    let per_render = counters().delta(before).calls;
    assert!(output.contains("item-999"));

    // The previous implementation performed ~4 allocations per loop iteration
    // (a scope map per item plus two temporary strings per interpolation). Guard
    // against regressions with a loose but meaningful bound.
    assert!(per_render < 3000, "engine allocated {per_render} times for a 1000-item render");

    let before = counters();
    for _ in 0..2000 {
        let _ = engine.render("page", context! { items: items.clone() }).unwrap();
    }
    let live_growth = counters().delta(before).live_bytes;

    // Live bytes must not grow meaningfully over repeated renders.
    assert!(
        live_growth <= 256 * 1024,
        "live bytes grew by {live_growth} over 2000 renders (possible leak)"
    );
}
