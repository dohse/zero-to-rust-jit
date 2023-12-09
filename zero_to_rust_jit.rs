#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LLVMOpaqueModule;
    pub type LLVMOpaqueError;
    pub type LLVMOrcOpaqueJITDylib;
    pub type LLVMOrcOpaqueThreadSafeContext;
    pub type LLVMOrcOpaqueLLJIT;
    fn handleError(Err_0: LLVMErrorRef) -> libc::c_int;
    #[link_name = "loop"]
    fn loop_0(
        Sum: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>,
    );
    fn addGenerator(Unit: LLVMOrcJITDylibRef, Resolve: Option::<ResolveFn>);
    fn addModule(Jit: LLVMOrcLLJITRef, Mod: LLVMModuleRef) -> LLVMOrcJITDylibRef;
    fn loadModule(
        FileName: *const libc::c_char,
        Ctx: LLVMOrcThreadSafeContextRef,
    ) -> LLVMModuleRef;
    fn shutdown(ExitCode: libc::c_int);
    fn init(
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        Jit: *mut LLVMOrcLLJITRef,
        Ctx: *mut LLVMOrcThreadSafeContextRef,
    ) -> *const libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn LLVMOrcLLJITLookup(
        J: LLVMOrcLLJITRef,
        Result: *mut LLVMOrcExecutorAddress,
        Name: *const libc::c_char,
    ) -> LLVMErrorRef;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type LLVMModuleRef = *mut LLVMOpaqueModule;
pub type LLVMErrorRef = *mut LLVMOpaqueError;
pub type LLVMOrcJITTargetAddress = uint64_t;
pub type LLVMOrcExecutorAddress = uint64_t;
pub type LLVMOrcJITDylibRef = *mut LLVMOrcOpaqueJITDylib;
pub type LLVMOrcThreadSafeContextRef = *mut LLVMOrcOpaqueThreadSafeContext;
pub type LLVMOrcLLJITRef = *mut LLVMOrcOpaqueLLJIT;
pub type ResolveFn = unsafe extern "C" fn(
    *const libc::c_char,
) -> LLVMOrcJITTargetAddress;
#[no_mangle]
pub unsafe extern "C" fn helloImpl() {
    printf(
        b"Oh hello, that's called from JITed code!\n\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn handleUndefinedSymbol(
    mut MangledName: *const libc::c_char,
) -> LLVMOrcJITTargetAddress {
    if strncmp(
        MangledName,
        b"hello\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            LLVMOrcJITTargetAddress,
        >(Some(helloImpl as unsafe extern "C" fn() -> ()));
    }
    return 0 as libc::c_int as LLVMOrcJITTargetAddress;
}
pub unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut Jit: LLVMOrcLLJITRef = 0 as *mut LLVMOrcOpaqueLLJIT;
    let mut Ctx: LLVMOrcThreadSafeContextRef = 0 as *mut LLVMOrcOpaqueThreadSafeContext;
    let mut FileName: *const libc::c_char = init(argc, argv, &mut Jit, &mut Ctx);
    let mut Mod: LLVMModuleRef = loadModule(FileName, Ctx);
    let mut Unit: LLVMOrcJITDylibRef = addModule(Jit, Mod);
    addGenerator(
        Unit,
        Some(
            handleUndefinedSymbol
                as unsafe extern "C" fn(*const libc::c_char) -> LLVMOrcJITTargetAddress,
        ),
    );
    let mut Err_0: LLVMErrorRef = 0 as *mut LLVMOpaqueError;
    let mut SumFnAddr: LLVMOrcJITTargetAddress = 0;
    Err_0 = LLVMOrcLLJITLookup(
        Jit,
        &mut SumFnAddr,
        b"sum\0" as *const u8 as *const libc::c_char,
    );
    if !Err_0.is_null() {
        shutdown(handleError(Err_0));
    }
    loop_0(
        ::core::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>,
        >(SumFnAddr as libc::intptr_t),
    );
    shutdown(0 as libc::c_int);
    return 0;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
