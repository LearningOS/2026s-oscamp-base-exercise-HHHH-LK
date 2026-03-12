//! # no_std Memory Primitives
//!
//! In a `#![no_std]` environment, you have no standard library — only `core`.
//! These memory operation functions are the most fundamental building blocks in an OS kernel.
//! Functions like memcpy/memset in libc must be implemented by ourselves in bare-metal environments.
//!
//! ## Task
//!
//! Implement the following five functions:
//! - Only use the `core` crate, no `std`
//! - Do not call `core::ptr::copy`, `core::ptr::copy_nonoverlapping`, etc. (write your own loops)
//! - Handle edge cases correctly (n=0, overlapping memory regions, etc.)
//! - Pass all tests

// Force no_std in production; allow std in tests (cargo test framework requires it)
#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]

/// Copy `n` bytes from `src` to `dst`.
///
/// - `dst` and `src` must not overlap (use `my_memmove` for overlapping regions)
/// - Returns `dst`
///
/// # Safety
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // 你的 memcpy 实现基本正确，但可以简化去掉多余的 unsafe 块（外层已经是 unsafe）
    for i in 0..n {
        *dst.add(i) = *src.add(i);
    }
    dst
}

/// Set `n` bytes starting at `dst` to the value `c`.
///
/// Returns `dst`.
///
/// # Safety
/// `dst` must point to at least `n` bytes of valid writable memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memset(dst: *mut u8, c: u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dst.add(i) = c;
    }
    dst
}

/// Copy `n` bytes from `src` to `dst`, correctly handling overlapping memory.
///
/// Returns `dst`.
///
/// # Safety
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if n == 0 {
        return dst;
    }

    // 将指针转为 usize 进行比较
    let src_addr = src as usize;
    let dst_addr = dst as usize;

    if dst_addr > src_addr && dst_addr < src_addr + n {
        // 情况 1: 重叠且 dst 在 src 后面 (向后覆盖)
        // 必须【从后往前】拷贝，防止数据被提前覆盖
        // 例如：src=[1,2,3,4], dst 指向 src+1. 
        // 如果从前向后：dst[0]=src[0](1) -> buf=[1,1,2,3]; dst[1]=src[1](现在的1) -> buf=[1,1,1,2] (错了！应该是 2)
        // 正确做法：先拷最后一个。
        for i in (0..n).rev() {
            *dst.add(i) = *src.add(i);
        }
    } else {
        // 情况 2: 不重叠 或 dst 在 src 前面
        // 可以【从前往后】拷贝 (也可以用 memcpy 的逻辑)
        for i in 0..n {
            *dst.add(i) = *src.add(i);
        }
    }
    
    dst
}

/// Return the length of a null-terminated byte string, excluding the trailing null.
///
/// # Safety
/// `s` must point to a valid null-terminated byte string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strlen(s: *const u8) -> usize {
    if s.is_null() {
        // 在 no_std 中无法 panic，通常这里如果是 invalid input 会导致未定义行为
        // 但为了安全起见，如果真的是 null，直接返回 0 或者让后续解引用崩溃
        // 既然契约是 unsafe，我们假设 s 不为 null。
        // 如果一定要处理，可以直接返回 0，但标准行为通常是 crash。
        return 0; 
    }

    let mut count = 0;
    let mut current = s; // 使用可变指针遍历

    loop {
        if *current == b'\0' {
            break;
        }
        count += 1;
        // 【关键修正】移动 current 指针，而不是重置为 s+1
        current = current.add(1);
    }
    
    count
}

/// Compare two null-terminated byte strings.
///
/// Returns:
/// - `0`  : strings are equal
/// - `< 0`: `s1` is lexicographically less than `s2`
/// - `> 0`: `s1` is lexicographically greater than `s2`
///
/// # Safety
/// `s1` and `s2` must each point to a valid null-terminated byte string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    // 你的 strcmp 实现逻辑是正确的，保留即可
    // 稍微精简一下 null 检查，unsafe 函数通常依赖调用者保证非空
    // 但为了鲁棒性，保留检查返回特定值也可以
    
    let mut current1 = s1;
    let mut current2 = s2;

    loop {
        let b1 = *current1;
        let b2 = *current2;

        if b1 != b2 {
            return (b1 as i32) - (b2 as i32);
        }
        
        if b1 == b'\0' {
            // 如果相等且都是 \0，返回 0
            return 0;
        }

        current1 = current1.add(1);
        current2 = current2.add(1);
    }
}

// ============================================================
// Tests (std is available under #[cfg(test)])
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memcpy_basic() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 5) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memcpy_zero_len() {
        let src = [0xFFu8; 4];
        let mut dst = [0u8; 4];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 0) };
        assert_eq!(dst, [0u8; 4]);
    }

    #[test]
    fn test_memset_basic() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xAB, 8) };
        assert!(buf.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_memset_partial() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xFF, 4) };
        assert_eq!(&buf[..4], &[0xFF; 4]);
        assert_eq!(&buf[4..], &[0x00; 4]);
    }

    #[test]
    fn test_memmove_no_overlap() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        unsafe { my_memmove(dst.as_mut_ptr(), src.as_ptr(), 4) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memmove_overlap_forward() {
        // Copy buf[0..4] to buf[1..5], shifting right by 1
        // 原始: [1, 2, 3, 4, 5]
        // 目标: [1, 1, 2, 3, 4] (前 4 个字节向右移一位)
        let mut buf = [1u8, 2, 3, 4, 5];
        unsafe { my_memmove(buf.as_mut_ptr().add(1), buf.as_ptr(), 4) };
        assert_eq!(buf, [1, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_memmove_overlap_backward() {
        // Copy buf[1..5] to buf[0..4], shifting left by 1
        // 原始: [1, 2, 3, 4, 5]
        // 目标: [2, 3, 4, 5, 5]
        let mut buf = [1u8, 2, 3, 4, 5];
        unsafe { my_memmove(buf.as_mut_ptr(), buf.as_ptr().add(1), 4) };
        assert_eq!(buf, [2, 3, 4, 5, 5]);
    }

    #[test]
    fn test_strlen_basic() {
        let s = b"hello\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 5);
    }

    #[test]
    fn test_strlen_empty() {
        let s = b"\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_equal() {
        let a = b"hello\0";
        let b = b"hello\0";
        assert_eq!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_less() {
        let a = b"abc\0";
        let b = b"abd\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } < 0);
    }

    #[test]
    fn test_strcmp_greater() {
        let a = b"abd\0";
        let b = b"abc\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } > 0);
    }
}