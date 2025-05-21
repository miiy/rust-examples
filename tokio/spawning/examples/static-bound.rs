use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // 默认情况下变量不会移动到异步块中
    // v 向量仍归 main 函数所有
    // println! 行借用了 v
    // task::spawn(async {
    //     println!("Here's a vec: {:?}", v);
    // });

    // move 将 v 移动到生成的任务中
    // 现在，任务拥有其所有数据，使其成为 'static
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}