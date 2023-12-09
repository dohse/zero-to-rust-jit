#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LLVMOpaqueMemoryBuffer;
    pub type LLVMOpaqueContext;
    pub type LLVMOpaqueModule;
    pub type LLVMOpaqueDiagnosticInfo;
    pub type LLVMOrcOpaqueThreadSafeContext;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn LLVMDisposeMessage(Message: *mut libc::c_char);
    fn LLVMContextSetDiagnosticHandler(
        C: LLVMContextRef,
        Handler: LLVMDiagnosticHandler,
        DiagnosticContext: *mut libc::c_void,
    );
    fn LLVMGetDiagInfoDescription(DI: LLVMDiagnosticInfoRef) -> *mut libc::c_char;
    fn LLVMParseBitcode2(
        MemBuf: LLVMMemoryBufferRef,
        OutModule: *mut LLVMModuleRef,
    ) -> LLVMBool;
    fn LLVMCreateMemoryBufferWithContentsOfFile(
        Path: *const libc::c_char,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        OutMessage: *mut *mut libc::c_char,
    ) -> LLVMBool;
    fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef);
    fn shutdown(ExitCode: libc::c_int);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn LLVMOrcThreadSafeContextGetContext(
        TSCtx: LLVMOrcThreadSafeContextRef,
    ) -> LLVMContextRef;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type LLVMBool = libc::c_int;
pub type LLVMMemoryBufferRef = *mut LLVMOpaqueMemoryBuffer;
pub type LLVMContextRef = *mut LLVMOpaqueContext;
pub type LLVMModuleRef = *mut LLVMOpaqueModule;
pub type LLVMDiagnosticInfoRef = *mut LLVMOpaqueDiagnosticInfo;
pub type LLVMDiagnosticHandler = Option::<
    unsafe extern "C" fn(LLVMDiagnosticInfoRef, *mut libc::c_void) -> (),
>;
pub type LLVMOrcThreadSafeContextRef = *mut LLVMOrcOpaqueThreadSafeContext;
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
unsafe extern "C" fn DiagnosticHandler(
    mut DI: LLVMDiagnosticInfoRef,
    mut C: *mut libc::c_void,
) {
    let mut CErr: *mut libc::c_char = LLVMGetDiagInfoDescription(DI);
    fprintf(
        stderr,
        b"Error in bitcode parser: %s\n\0" as *const u8 as *const libc::c_char,
        CErr,
    );
    LLVMDisposeMessage(CErr);
    shutdown(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn loadModule(
    mut FileName: *const libc::c_char,
    mut Ctx: LLVMOrcThreadSafeContextRef,
) -> LLVMModuleRef {
    let mut Err_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut MemBuf: LLVMMemoryBufferRef = 0 as *mut LLVMOpaqueMemoryBuffer;
    let mut EC: LLVMBool = LLVMCreateMemoryBufferWithContentsOfFile(
        FileName,
        &mut MemBuf,
        &mut Err_0,
    );
    if EC != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Error reading file %s: %s\n\0" as *const u8 as *const libc::c_char,
            FileName,
            Err_0,
        );
        shutdown(EC);
    }
    let mut BareCtx: LLVMContextRef = LLVMOrcThreadSafeContextGetContext(Ctx);
    LLVMContextSetDiagnosticHandler(
        BareCtx,
        Some(
            DiagnosticHandler
                as unsafe extern "C" fn(LLVMDiagnosticInfoRef, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
    let mut Mod: LLVMModuleRef = 0 as *mut LLVMOpaqueModule;
    EC = LLVMParseBitcode2(MemBuf, &mut Mod);
    if EC != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Error parsing bitcode: %s\n\0" as *const u8 as *const libc::c_char,
            FileName,
        );
        LLVMDisposeMemoryBuffer(MemBuf);
        shutdown(EC);
    }
    return Mod;
}
