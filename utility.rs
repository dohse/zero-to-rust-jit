#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LLVMOpaqueModule;
    pub type LLVMOpaqueError;
    pub type LLVMOrcOpaqueJITDylib;
    pub type LLVMOrcOpaqueThreadSafeContext;
    pub type LLVMOrcOpaqueThreadSafeModule;
    pub type LLVMOrcOpaqueLLJITBuilder;
    pub type LLVMOrcOpaqueLLJIT;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn LLVMShutdown();
    fn LLVMGetErrorMessage(Err_0: LLVMErrorRef) -> *mut libc::c_char;
    fn LLVMDisposeErrorMessage(ErrMsg: *mut libc::c_char);
    fn LLVMInitializeX86TargetInfo();
    fn LLVMInitializeX86Target();
    fn LLVMInitializeX86TargetMC();
    fn LLVMInitializeX86AsmPrinter();
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn LLVMOrcCreateLLJIT(
        Result: *mut LLVMOrcLLJITRef,
        Builder: LLVMOrcLLJITBuilderRef,
    ) -> LLVMErrorRef;
    fn LLVMOrcDisposeLLJIT(J: LLVMOrcLLJITRef) -> LLVMErrorRef;
    fn LLVMOrcLLJITGetMainJITDylib(J: LLVMOrcLLJITRef) -> LLVMOrcJITDylibRef;
    fn LLVMOrcLLJITAddLLVMIRModule(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcJITDylibRef,
        TSM: LLVMOrcThreadSafeModuleRef,
    ) -> LLVMErrorRef;
    fn LLVMParseCommandLineOptions(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
        Overview: *const libc::c_char,
    );
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn LLVMOrcCreateNewThreadSafeContext() -> LLVMOrcThreadSafeContextRef;
    fn LLVMOrcDisposeThreadSafeContext(TSCtx: LLVMOrcThreadSafeContextRef);
    fn LLVMOrcCreateNewThreadSafeModule(
        M: LLVMModuleRef,
        TSCtx: LLVMOrcThreadSafeContextRef,
    ) -> LLVMOrcThreadSafeModuleRef;
    fn LLVMOrcDisposeThreadSafeModule(TSM: LLVMOrcThreadSafeModuleRef);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type LLVMBool = libc::c_int;
pub type LLVMModuleRef = *mut LLVMOpaqueModule;
pub type LLVMErrorRef = *mut LLVMOpaqueError;
pub type LLVMOrcJITDylibRef = *mut LLVMOrcOpaqueJITDylib;
pub type LLVMOrcThreadSafeContextRef = *mut LLVMOrcOpaqueThreadSafeContext;
pub type LLVMOrcThreadSafeModuleRef = *mut LLVMOrcOpaqueThreadSafeModule;
pub type LLVMOrcLLJITBuilderRef = *mut LLVMOrcOpaqueLLJITBuilder;
pub type LLVMOrcLLJITRef = *mut LLVMOrcOpaqueLLJIT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn LLVMInitializeNativeTarget() -> LLVMBool {
    LLVMInitializeX86TargetInfo();
    LLVMInitializeX86Target();
    LLVMInitializeX86TargetMC();
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn LLVMInitializeNativeAsmPrinter() -> LLVMBool {
    LLVMInitializeX86AsmPrinter();
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut JitInst: LLVMOrcLLJITRef = 0 as *const LLVMOrcOpaqueLLJIT
    as LLVMOrcLLJITRef;
#[no_mangle]
pub static mut CtxInst: LLVMOrcThreadSafeContextRef = 0
    as *const LLVMOrcOpaqueThreadSafeContext as LLVMOrcThreadSafeContextRef;
#[no_mangle]
pub unsafe extern "C" fn handleError(mut Err_0: LLVMErrorRef) -> libc::c_int {
    let mut ErrMsg: *mut libc::c_char = LLVMGetErrorMessage(Err_0);
    fprintf(stderr, b"Error: %s\n\0" as *const u8 as *const libc::c_char, ErrMsg);
    LLVMDisposeErrorMessage(ErrMsg);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut Jit: *mut LLVMOrcLLJITRef,
    mut Ctx: *mut LLVMOrcThreadSafeContextRef,
) -> *const libc::c_char {
    if argc < 2 as libc::c_int {
        let mut BaseName: *const libc::c_char = strrchr(
            *argv.offset(0 as libc::c_int as isize),
            '/' as i32,
        );
        let mut ExecName: *const libc::c_char = if !BaseName.is_null() {
            BaseName.offset(1 as libc::c_int as isize)
        } else {
            b"zero-to-rust-jit\0" as *const u8 as *const libc::c_char
        };
        fprintf(
            stderr,
            b"Usage: %s sum.bc [ llvm-flags ]\n\0" as *const u8 as *const libc::c_char,
            ExecName,
        );
        shutdown(1 as libc::c_int);
    }
    LLVMParseCommandLineOptions(
        argc,
        argv as *const *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    LLVMInitializeNativeTarget();
    LLVMInitializeNativeAsmPrinter();
    let mut Err_0: LLVMErrorRef = LLVMOrcCreateLLJIT(
        &mut JitInst,
        0 as LLVMOrcLLJITBuilderRef,
    );
    if !Err_0.is_null() {
        shutdown(handleError(Err_0));
    }
    CtxInst = LLVMOrcCreateNewThreadSafeContext();
    *Jit = JitInst;
    *Ctx = CtxInst;
    return *argv.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn addModule(
    mut Jit: LLVMOrcLLJITRef,
    mut Mod: LLVMModuleRef,
) -> LLVMOrcJITDylibRef {
    let mut TSM: LLVMOrcThreadSafeModuleRef = LLVMOrcCreateNewThreadSafeModule(
        Mod,
        CtxInst,
    );
    let mut MainJD: LLVMOrcJITDylibRef = LLVMOrcLLJITGetMainJITDylib(Jit);
    let mut Err_0: LLVMErrorRef = LLVMOrcLLJITAddLLVMIRModule(Jit, MainJD, TSM);
    if !Err_0.is_null() {
        LLVMOrcDisposeThreadSafeModule(TSM);
        shutdown(handleError(Err_0));
    }
    return MainJD;
}
#[export_name = "loop"]
pub unsafe extern "C" fn loop_0(
    mut Sum: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>,
) {
    let mut answer: libc::c_char = 0;
    loop {
        let mut a: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        printf(b"a = \0" as *const u8 as *const libc::c_char);
        if scanf(b"%x\0" as *const u8 as *const libc::c_char, &mut a as *mut libc::c_int)
            != 1 as libc::c_int
        {
            shutdown(1 as libc::c_int);
        }
        printf(b"b = \0" as *const u8 as *const libc::c_char);
        if scanf(b"%x\0" as *const u8 as *const libc::c_char, &mut b as *mut libc::c_int)
            != 1 as libc::c_int
        {
            shutdown(1 as libc::c_int);
        }
        let mut Result: libc::c_int = Sum.expect("non-null function pointer")(a, b);
        printf(b"%i + %i = %i\n\0" as *const u8 as *const libc::c_char, a, b, Result);
        printf(b"Again? (y/n) \0" as *const u8 as *const libc::c_char);
        if scanf(
            b" %c\0" as *const u8 as *const libc::c_char,
            &mut answer as *mut libc::c_char,
        ) != 1 as libc::c_int
        {
            shutdown(1 as libc::c_int);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        if !(answer as libc::c_int == 'y' as i32 || answer as libc::c_int == 'Y' as i32)
        {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn shutdown(mut ExitCode: libc::c_int) {
    if !JitInst.is_null() {
        let mut Err_0: LLVMErrorRef = LLVMOrcDisposeLLJIT(JitInst);
        if !Err_0.is_null() {
            let mut JITShutdownExitCode: libc::c_int = handleError(Err_0);
            if ExitCode == 0 as libc::c_int {
                ExitCode = JITShutdownExitCode;
            }
        }
    }
    if !CtxInst.is_null() {
        LLVMOrcDisposeThreadSafeContext(CtxInst);
    }
    LLVMShutdown();
    exit(ExitCode);
}
