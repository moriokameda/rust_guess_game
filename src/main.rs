use std::future::Future;
use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans = async_add(2, 3).await;

    println!("{}", ans);
    ans
}
fn some_great_function() -> impl Future<Output = i32> {
    async {
        let value = 5;
        send_to_another_thread_with_borrowing(&value).await
    }
}

async fn send_to_another_thread_with_borrowing(x: &i32) -> i32 {
    *x
}

fn main() {
    executor::block_on(something_great_async_function());
}