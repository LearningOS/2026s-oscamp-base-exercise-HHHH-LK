//! # 手动实现 Future
//!
//! 在本练习中，您将手动为自定义类型实现 `Future` trait，以理解异步运行时的核心机制。
//!
//! ## 概念
//! - `std::future::Future` trait
//! - `Poll::Ready` 和 `Poll::Pending`
//! - `Waker` 的作用：通知运行时再次轮询

use std::future::Future;
use std::io::Seek;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 倒计时 Future：每次轮询时 count 减 1，
/// 当 count 达到 0 时返回 `"liftoff!"`。
pub struct CountDown {
    pub count: u32,
}

impl CountDown {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

// TODO: 为 CountDown 实现 Future trait
// - 输出类型是 &'static str
// - 每次 poll：如果 count == 0，返回 Poll::Ready("liftoff!")
// - 否则 count -= 1，调用 cx.waker().wake_by_ref()，返回 Poll::Pending
//
// 提示：使用 `self.get_mut()` 获取 `&mut Self`（因为 self 是 Pin<&mut Self>）
impl Future for CountDown {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let count_down = self.get_mut();
        if count_down.count == 0 {
            return Poll::Ready("liftoff!");
        } else {
            count_down.count -= 1;
            // 当调用 cx.waker().wake_by_ref() 时，执行器会安排该任务稍后被再次 poll。
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
    }
}

/// 只 yield 一次的 Future：第一次轮询返回 Pending，第二次返回 Ready(())。
/// 这是异步状态机的最小示例。
pub struct YieldOnce {
    yielded: bool,
}

impl YieldOnce {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

// TODO: 为 YieldOnce 实现 Future trait
// - 输出类型是 ()
// - 第一次 poll：设置 yielded = true，唤醒 waker，返回 Pending
// - 第二次 poll：返回 Ready(())
impl Future for YieldOnce {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let used = self.get_mut();
        if used.yielded {
            return Poll::Ready(());
        } else {
            used.yielded = true;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_countdown_zero() {
        let result = CountDown::new(0).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_countdown_three() {
        let result = CountDown::new(3).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_yield_once() {
        YieldOnce::new().await;
    }

    #[tokio::test]
    async fn test_countdown_large() {
        let result = CountDown::new(100).await;
        assert_eq!(result, "liftoff!");
    }
}
