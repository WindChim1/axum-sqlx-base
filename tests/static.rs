use std::{
    cell::OnceCell,
    sync::{Once, OnceLock},
    thread,
};

static mut ADD: i32 = 0;
#[test]
fn static_test() {
    unsafe { println!("add ={}", ADD) }
    assert_eq!(1, 2)
}

static WINNER: OnceLock<&str> = OnceLock::new();

#[test]
fn once_lock_test() {
    thread::scope(|s| {
        s.spawn(|| WINNER.set("thread"));
        thread::yield_now();
        WINNER.get_or_init(|| "main")
    });
    println!("winner ={:?}", WINNER);
    assert_eq!(1, 2)
}

static mut CELL: OnceCell<Vec<i32>> = OnceCell::new();

#[test]
fn once_cell_test() {
    unsafe {
        CELL.get_or_init(|| {
            let v = vec![1, 2, 3];
            v
        });
        if let Some(v) = CELL.get() {
            println!("{v:?}")
        }
    };
    assert_eq!(1, 2)
}

static mut CACHE: Option<Vec<i32>> = None;
static ONCE: Once = Once::new();

#[test]
fn once_test() {
    let data = get_data();
    println!("data={data:?}");

    let data = get_data();
    println!("data={data:?}");

    assert_eq!(1, 2)
}

fn get_data() -> &'static Vec<i32> {
    unsafe {
        ONCE.call_once(|| {
            let data = vec![1, 2];
            CACHE = Some(data);
            println!("Initializing cache")
        });
        CACHE.as_ref().unwrap()
    }
}
