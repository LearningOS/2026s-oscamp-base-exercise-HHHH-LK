//! # Tokio 异步任务
//!
//! 在本练习中，你将使用 `tokio::spawn` 来创建并发异步任务。
//!
//! ## 概念
//! - `tokio::spawn` 用于创建异步任务
//! - `JoinHandle` 用于等待任务完成
//! - 异步任务之间的并发执行

use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/// 并发计算 0..n 中每个数的平方，收集结果并按顺序返回。
///
/// 提示：为每个 i 创建一个 `tokio::spawn` 任务，收集 JoinHandle，然后依次 await 它们。
pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    // TODO: 创建 n 个异步任务，每个计算 i * i
    // TODO: 收集所有的 JoinHandle
    // TODO: 依次 await 每个任务以获取结果
    let mut handle_items: Vec<JoinHandle<usize>> = Vec::new();
    for i in 0..n {
        let handle = tokio::spawn(async move {
            return i * i;
        });
        handle_items.push(handle);
    }

    //依次处理数据结果
    let mut results = Vec::new();
    for handle in handle_items {
        results.push(handle.await.unwrap());
    }
    results
}

/// 并发执行多个"耗时的"任务（用 sleep 模拟），返回所有结果。
/// 每个任务 sleep `duration_ms` 毫秒后返回其 `task_id`。
///
/// 关键点：所有任务应该并发执行，总耗时应该接近单个任务的耗时，而不是所有任务耗时之和。
pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    // TODO: 为 0..n 中的每个 id 创建一个异步任务
    // TODO: 每个任务 sleep 指定时长后返回自己的 id
    // TODO: 收集所有结果并排序
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_squares_basic() {
        let result = concurrent_squares(5).await;
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[tokio::test]
    async fn test_squares_zero() {
        let result = concurrent_squares(0).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_squares_one() {
        let result = concurrent_squares(1).await;
        assert_eq!(result, vec![0]);
    }

    #[tokio::test]
    async fn test_parallel_sleep() {
        let start = Instant::now();
        let result = parallel_sleep_tasks(5, 100).await;
        let elapsed = start.elapsed();

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        // Concurrent execution, total time should be much less than 5 * 100ms
        assert!(
            elapsed.as_millis() < 400,
            "任务应该并发执行，耗时 {}ms",
            elapsed.as_millis()
        );
    }
}
