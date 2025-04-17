use tokio::{
    spawn,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Spawn two tasks, one gets a key, the other sets a key
    let t2 = spawn(async {
        println!("hello world again!!");
    });
    let t1 = spawn(async {
        println!("hello world");
    });

    // t1.await.unwrap();
    // t2.await.unwrap();
    let _ = tokio::join!(t2, t1);

    // task azin thread
    spawn(async { println!("hello task") }).await.unwrap();

    // task without explicitly calling it that
    spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("hello tokio spawn");
    })
    .await
    .unwrap();

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

async fn say_world() {
    sleep(Duration::from_secs(1)).await;
    println!("world from another function");
}
