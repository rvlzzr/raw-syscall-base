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
pub unsafe fn syscall(rax: usize, a: &[usize]) -> Result<usize, usize> {
    match a.len() {
        0 => syscall_0(rax),
        1 => syscall_1(rax, a[0]),
        2 => syscall_2(rax, a[0], a[1]),
        3 => syscall_3(rax, a[0], a[1], a[2]),
        4 => syscall_4(rax, a[0], a[1], a[2], a[3]),
        5 => syscall_5(rax, a[0], a[1], a[2], a[3], a[4]),
        6 => syscall_6(rax, a[0], a[1], a[2], a[3], a[4], a[5]),
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
pub unsafe fn syscall_nr(rax: usize, a: &[usize]) -> ! {
    match a.len() {
        0 => syscall_0_nr(rax),
        1 => syscall_1_nr(rax, a[0]),
        2 => syscall_2_nr(rax, a[0], a[1]),
        3 => syscall_3_nr(rax, a[0], a[1], a[2]),
        4 => syscall_4_nr(rax, a[0], a[1], a[2], a[3]),
        5 => syscall_5_nr(rax, a[0], a[1], a[2], a[3], a[4]),
        6 => syscall_6_nr(rax, a[0], a[1], a[2], a[3], a[4], a[5]),
        _ => unreachable_unchecked(),
    }
}

/// Performs a system call with no arguments and returns the result.
///
/// The argument specifies the system call.
///
#[inline(always)]
pub unsafe fn syscall_0(mut rax: usize) -> Result<usize, usize> {
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
    }
}

/// Performs a system call with no arguments which never returns.
///
/// The argument specifies the system call.
///
/// This function should only be used for calls guaranteed to never return.
///
#[inline(always)]
pub unsafe fn syscall_0_nr(rax: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        options(noreturn)
    );
}

/// Performs a system call with one argument and returns the result.
///
/// The first argument specifies the system call, and the second is the
/// argument to pass it.
///
#[inline(always)]
pub unsafe fn syscall_1(mut rax: usize, rdi: usize) -> Result<usize, usize> {
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_1_nr(rax: usize, rdi: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        options(noreturn)
    );
}

/// Performs a system call with two arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_2(mut rax: usize, rdi: usize, rsi: usize) -> Result<usize, usize> {
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        in("rsi") rsi,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_2_nr(rax: usize, rdi: usize, rsi: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        in("rsi") rsi,
        options(noreturn)
    );
}

/// Performs a system call with three arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_3(mut rax: usize, rdi: usize, rsi: usize, rdx: usize) -> Result<usize, usize> {
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_3_nr(rax: usize, rdi: usize, rsi: usize, rdx: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        options(noreturn)
    );
}

/// Performs a system call with four arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_4(mut rax: usize, rdi: usize, rsi: usize, rdx: usize, r10: usize) -> Result<usize, usize> {
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_4_nr(rax: usize, rdi: usize, rsi: usize, rdx: usize, r10: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        options(noreturn)
    );
}

/// Performs a system call with five arguments and returns the result.
///
/// The first argument specifies the system call, and the remaining arguments
/// are the arguments to pass it.
///
#[inline(always)]
pub unsafe fn syscall_5(
    mut rax: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    r10: usize,
    r8: usize,
) -> Result<usize, usize>
{
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        in("r8") r8,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_5_nr(rax: usize, rdi: usize, rsi: usize, rdx: usize, r10: usize, r8: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        in("r8") r8,
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
    mut rax: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    r10: usize,
    r8: usize,
    r9: usize,
) -> Result<usize, usize>
{
    asm!(
        "syscall",
        inlateout("rax") rax => rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        in("r8") r8,
        in("r9") r9,
        out("rcx") _,
        out("r11") _
    );
    if rax < 0xffff_ffff_ffff_f000 {
        Ok(rax)
    } else {
        Err(rax)
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
pub unsafe fn syscall_6_nr(rax: usize, rdi: usize, rsi: usize, rdx: usize, r10: usize, r8: usize, r9: usize) -> ! {
    asm!(
        "syscall",
        in("rax") rax,
        in("rdi") rdi,
        in("rsi") rsi,
        in("rdx") rdx,
        in("r10") r10,
        in("r8") r8,
        in("r9") r9,
        options(noreturn)
    );
}
