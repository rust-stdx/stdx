//! Concurrency tests: the engine is `Send + Sync` and renders are independent.

use std::{sync::Arc, thread};

use template::{Engine, EscapeMode, context};

#[test]
fn engine_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Engine>();
}

#[test]
fn concurrent_renders_are_independent() {
    let mut engine = Engine::new(EscapeMode::Text);
    engine.add_template("t", "Hello, {{ name }}!").unwrap();
    engine
        .add_template("loop", "{% for i in items %}{{ i }},{% endfor %}")
        .unwrap();
    let engine = Arc::new(engine);

    let handles: Vec<_> = (0..8)
        .map(|worker| {
            let engine = Arc::clone(&engine);
            thread::spawn(move || {
                for i in 0..200 {
                    let name = format!("w{worker}-{i}");
                    let out = engine.render("t", context! { name: name.clone() }).unwrap();
                    assert_eq!(out, format!("Hello, {name}!"));

                    let out = engine.render("loop", context! { items: vec![1, 2, 3] }).unwrap();
                    assert_eq!(out, "1,2,3,");
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("render thread panicked");
    }
}
