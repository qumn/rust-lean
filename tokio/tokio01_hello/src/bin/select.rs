use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    // let (tx1, rx1) = oneshot::channel();
    // let (tx2, rx2) = oneshot::channel();

    // tokio::spawn(async {
    //     println!("send one");
    //     let _ = tx1.send("one");
    // });
    // tokio::spawn(async {
    //     println!("send two");
    //     let _ = tx2.send("two");
    // });

    // // select macro 会造成未完成的future取消
    // tokio::select! {
    //     val = rx1 => {
    //         println!("rx1 completed first with {:?}", val);
    //     }
    //     val = rx2 => {
    //         println!("rx2 completed first with {:?}", val)
    //     }
    // }

    // 1. Wait for an even number on the channel.
    // 2. Start the asynchronous operation using the even number as input.
    // 3. Wait for the operation, but at the same time listen for more even numbers on the channel.
    // 4. If a new even number is received before the existing operation completes, abort (中止) the existing operation and start it over with the new even number.

    let (mut tx, mut rx) = tokio::sync::mpsc::channel::<i32>(128);

    let operation = action(None);

    tokio::pin!(operation);

    loop {
        tokio::select! {
            _ = &mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                }
            }
        }
    }
}

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    Some(i.to_string())
}
