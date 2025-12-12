pub mod core;
use genawaiter::{rc::Gen, GeneratorState};

fn main() {
    let v = crate::core::tiledata::parse().unwrap();
    // let mut l = proceed();
    // if let GeneratorState::Yielded(x) = l.resume_with(2) {
    //     println!("{x}");
    // }
    // if let GeneratorState::Yielded(x) = l.resume_with(3) {
    //     println!("{x}");
    // }
}

fn proceed() -> Gen<i32, i32, impl Future<Output=()>> {
    Gen::new(|co| async move {
        let x = co.yield_(1).await;
    })
}