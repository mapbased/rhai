#![cfg(not(feature = "no_object"))]

use rhai::{Engine, EvalAltResult, INT};

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestStruct {
    x: INT,
}

impl TestStruct {
    fn update(&mut self, n: INT) {
        self.x += n;
    }

    fn new() -> Self {
        Self { x: 1 }
    }
}

#[test]
fn test_method_call() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine
        .register_type::<TestStruct>()
        .register_fn("update", TestStruct::update)
        .register_fn("new_ts", TestStruct::new);

    assert_eq!(
        engine.eval::<TestStruct>("let x = new_ts(); x.update(1000); x")?,
        TestStruct { x: 1001 }
    );

    assert_eq!(
        engine.eval::<TestStruct>("let x = new_ts(); update(x, 1000); x")?,
        TestStruct { x: 1001 }
    );

    Ok(())
}

#[test]
fn test_method_call_style() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(engine.eval::<INT>("let x = -123; x.abs(); x")?, -123);

    Ok(())
}

#[cfg(not(feature = "no_optimize"))]
#[test]
fn test_method_call_with_full_optimization() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.set_optimization_level(rhai::OptimizationLevel::Full);

    engine
        .register_fn("new_ts", || TestStruct::new())
        .register_fn("ymd", |_: INT, _: INT, _: INT| 42 as INT)
        .register_fn("range", |_: &mut TestStruct, _: INT, _: INT| {
            TestStruct::new()
        });

    assert_eq!(
        engine.eval::<TestStruct>(
            "
                let xs = new_ts();
                let ys = xs.range(ymd(2022, 2, 1), ymd(2022, 2, 2));
                ys
            "
        )?,
        TestStruct::new()
    );

    Ok(())
}
