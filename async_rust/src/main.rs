use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Spawn two tasks, one gets a key, the other sets a key
    let t2 = tokio::spawn(async {
        println!("hello world again!!");
    });
    let t1 = tokio::spawn(async {
        println!("hello world");
    });

    t1.await.unwrap();

    // task azin thread
    tokio::task::spawn(async { println!("hello task") })
        .await
        .unwrap();

    // task without explicitly calling it that
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("hello tokio spawn");
    })
    .await
    .unwrap();

    t2.await.unwrap();

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

async fn say_world() {
    // tokio::time::sleep(Duration::from_secs(1)).await;
    println!("world from another function");
}
