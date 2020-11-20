use ::core::{
    hint::unreachable_unchecked,
    result::Result::{
        self,
        Err,
        Ok,
    },
};

/// Performs a system call and returns the result.
///
/// The first argument specifies the system call, and the second is a slice of
/// arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall(x8: usize, a: &[usize]) -> Result<usize, usize> {
    match a.len() {
        0 => syscall_0(x8),
        1 => syscall_1(x8, a[0]),
        2 => syscall_2(x8, a[0], a[1]),
        3 => syscall_3(x8, a[0], a[1], a[2]),
        4 => syscall_4(x8, a[0], a[1], a[2], a[3]),
        5 => syscall_5(x8, a[0], a[1], a[2], a[3], a[4]),
        6 => syscall_6(x8, a[0], a[1], a[2], a[3], a[4], a[5]),
        _ => unreachable_unchecked(),
    }
}

/// Performs a system call which never returns.
///
/// The first argument specifies the system call, and the second is a slice of
/// arguments to pass it.
///
/// This should only be used for calls like `exit` or `exit_group` which are
/// guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_nr(x8: usize, a: &[usize]) -> ! {
    match a.len() {
        0 => syscall_0_nr(x8),
        1 => syscall_1_nr(x8, a[0]),
        2 => syscall_2_nr(x8, a[0], a[1]),
        3 => syscall_3_nr(x8, a[0], a[1], a[2]),
        4 => syscall_4_nr(x8, a[0], a[1], a[2], a[3]),
        5 => syscall_5_nr(x8, a[0], a[1], a[2], a[3], a[4]),
        6 => syscall_6_nr(x8, a[0], a[1], a[2], a[3], a[4], a[5]),
        _ => unreachable_unchecked(),
    }
}

/// Performs a system call with no arguments and returns the result.
///
/// The argument specifies the system call.
///
#[inline(always)]
pub unsafe fn syscall_0(x8: usize) -> Result<usize, usize> {
    let x0: usize;
    asm!(
        "svc #0",
        in("x8") x8,
        out("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with no arguments which never returns.
///
/// The argument specifies the system call.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_0_nr(x8: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        options(noreturn)
    );
}

/// Performs a system call with one argument and returns the result.
///
/// The first argument specifies the system call, and the second is the
/// argument to pass it.
///
#[inline(always)]
pub unsafe fn syscall_1(x8: usize, mut x0: usize) -> Result<usize, usize> {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        lateout("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with one argument and never returns.
///
/// The first argument specifies the system call, and the second is the
/// argument to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_1_nr(x8: usize, x0: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        options(noreturn)
    );
}

/// Performs a system call with two arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_2(x8: usize, mut x0: usize, x1: usize) -> Result<usize, usize> {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        lateout("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with two arguments which never returns.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_2_nr(x8: usize, x0: usize, x1: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        options(noreturn)
    );
}

/// Performs a system call with three arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_3(x8: usize, mut x0: usize, x1: usize, x2: usize) -> Result<usize, usize> {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        lateout("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with three arguments which never returns.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_3_nr(x8: usize, x0: usize, x1: usize, x2: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        options(noreturn)
    );
}

/// Performs a system call with four arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_4(x8: usize, mut x0: usize, x1: usize, x2: usize, x3: usize) -> Result<usize, usize> {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        lateout("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with four arguments which never returns.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_4_nr(x8: usize, x0: usize, x1: usize, x2: usize, x3: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        options(noreturn)
    );
}

/// Performs a system call with five arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_5(x8: usize, mut x0: usize, x1: usize, x2: usize, x3: usize, x4: usize) -> Result<usize, usize> {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        lateout("x0") x0
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with five arguments which never returns.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_5_nr(x8: usize, x0: usize, x1: usize, x2: usize, x3: usize, x4: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        options(noreturn)
    );
}

/// Performs a system call with six arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_6(
    x8: usize,
    mut x0: usize,
    x1: usize,
    x2: usize,
    x3: usize,
    x4: usize,
    x5: usize,
) -> Result<usize, usize>
{
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        in("x5") x5,
        lateout("x0") x0,
    );
    if x0 < 0xffff_ffff_ffff_f000 {
        Ok(x0)
    } else {
        Err(x0)
    }
}

/// Performs a system call with six arguments which never returns.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_6_nr(x8: usize, x0: usize, x1: usize, x2: usize, x3: usize, x4: usize, x5: usize) -> ! {
    asm!(
        "svc #0",
        in("x8") x8,
        in("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        in("x5") x5,
        options(noreturn)
    );
}
