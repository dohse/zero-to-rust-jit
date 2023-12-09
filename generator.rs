#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LLVMOpaqueError;
    pub type LLVMOrcOpaqueSymbolStringPoolEntry;
    pub type LLVMOrcOpaqueJITDylib;
    pub type LLVMOrcOpaqueMaterializationUnit;
    pub type LLVMOrcOpaqueDefinitionGenerator;
    pub type LLVMOrcOpaqueLookupState;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn LLVMOrcRetainSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef);
    fn LLVMOrcSymbolStringPoolEntryStr(
        S: LLVMOrcSymbolStringPoolEntryRef,
    ) -> *const libc::c_char;
    fn LLVMOrcAbsoluteSymbols(
        Syms: LLVMOrcCSymbolMapPairs,
        NumPairs: size_t,
    ) -> LLVMOrcMaterializationUnitRef;
    fn LLVMOrcJITDylibDefine(
        JD: LLVMOrcJITDylibRef,
        MU: LLVMOrcMaterializationUnitRef,
    ) -> LLVMErrorRef;
    fn LLVMOrcJITDylibAddGenerator(
        JD: LLVMOrcJITDylibRef,
        DG: LLVMOrcDefinitionGeneratorRef,
    );
    fn LLVMOrcCreateCustomCAPIDefinitionGenerator(
        F: LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction,
        Ctx: *mut libc::c_void,
        Dispose: LLVMOrcDisposeCAPIDefinitionGeneratorFunction,
    ) -> LLVMOrcDefinitionGeneratorRef;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type LLVMErrorRef = *mut LLVMOpaqueError;
pub type LLVMOrcJITTargetAddress = uint64_t;
pub type LLVMOrcExecutorAddress = uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const LLVMJITSymbolGenericFlagsMaterializationSideEffectsOnly: C2RustUnnamed = 8;
pub const LLVMJITSymbolGenericFlagsCallable: C2RustUnnamed = 4;
pub const LLVMJITSymbolGenericFlagsWeak: C2RustUnnamed = 2;
pub const LLVMJITSymbolGenericFlagsExported: C2RustUnnamed = 1;
pub const LLVMJITSymbolGenericFlagsNone: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLVMJITSymbolFlags {
    pub GenericFlags: uint8_t,
    pub TargetFlags: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLVMJITEvaluatedSymbol {
    pub Address: LLVMOrcExecutorAddress,
    pub Flags: LLVMJITSymbolFlags,
}
pub type LLVMOrcSymbolStringPoolEntryRef = *mut LLVMOrcOpaqueSymbolStringPoolEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLVMOrcCSymbolMapPair {
    pub Name: LLVMOrcSymbolStringPoolEntryRef,
    pub Sym: LLVMJITEvaluatedSymbol,
}
pub type LLVMOrcCSymbolMapPairs = *mut LLVMOrcCSymbolMapPair;
pub type LLVMOrcJITDylibRef = *mut LLVMOrcOpaqueJITDylib;
pub type LLVMOrcLookupKind = libc::c_uint;
pub const LLVMOrcLookupKindDLSym: LLVMOrcLookupKind = 1;
pub const LLVMOrcLookupKindStatic: LLVMOrcLookupKind = 0;
pub type LLVMOrcJITDylibLookupFlags = libc::c_uint;
pub const LLVMOrcJITDylibLookupFlagsMatchAllSymbols: LLVMOrcJITDylibLookupFlags = 1;
pub const LLVMOrcJITDylibLookupFlagsMatchExportedSymbolsOnly: LLVMOrcJITDylibLookupFlags = 0;
pub type LLVMOrcSymbolLookupFlags = libc::c_uint;
pub const LLVMOrcSymbolLookupFlagsWeaklyReferencedSymbol: LLVMOrcSymbolLookupFlags = 1;
pub const LLVMOrcSymbolLookupFlagsRequiredSymbol: LLVMOrcSymbolLookupFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLVMOrcCLookupSetElement {
    pub Name: LLVMOrcSymbolStringPoolEntryRef,
    pub LookupFlags: LLVMOrcSymbolLookupFlags,
}
pub type LLVMOrcCLookupSet = *mut LLVMOrcCLookupSetElement;
pub type LLVMOrcMaterializationUnitRef = *mut LLVMOrcOpaqueMaterializationUnit;
pub type LLVMOrcDefinitionGeneratorRef = *mut LLVMOrcOpaqueDefinitionGenerator;
pub type LLVMOrcLookupStateRef = *mut LLVMOrcOpaqueLookupState;
pub type LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction = Option::<
    unsafe extern "C" fn(
        LLVMOrcDefinitionGeneratorRef,
        *mut libc::c_void,
        *mut LLVMOrcLookupStateRef,
        LLVMOrcLookupKind,
        LLVMOrcJITDylibRef,
        LLVMOrcJITDylibLookupFlags,
        LLVMOrcCLookupSet,
        size_t,
    ) -> LLVMErrorRef,
>;
pub type LLVMOrcDisposeCAPIDefinitionGeneratorFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
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
pub type ResolveFn = unsafe extern "C" fn(
    *const libc::c_char,
) -> LLVMOrcJITTargetAddress;
#[no_mangle]
pub unsafe extern "C" fn dropLeadingUnderscores(
    mut Name: *const libc::c_char,
) -> *const libc::c_char {
    while *Name.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32 {
        Name = Name.offset(1 as libc::c_int as isize);
    }
    return Name;
}
#[no_mangle]
pub unsafe extern "C" fn redirect(
    mut Name: LLVMOrcSymbolStringPoolEntryRef,
    mut Addr: LLVMOrcJITTargetAddress,
) -> LLVMOrcMaterializationUnitRef {
    let mut Flags: LLVMJITSymbolFlags = {
        let mut init = LLVMJITSymbolFlags {
            GenericFlags: LLVMJITSymbolGenericFlagsWeak as libc::c_int as uint8_t,
            TargetFlags: 0 as libc::c_int as uint8_t,
        };
        init
    };
    let mut Sym: LLVMJITEvaluatedSymbol = {
        let mut init = LLVMJITEvaluatedSymbol {
            Address: Addr,
            Flags: Flags,
        };
        init
    };
    LLVMOrcRetainSymbolStringPoolEntry(Name);
    let mut Pair: LLVMOrcCSymbolMapPair = {
        let mut init = LLVMOrcCSymbolMapPair {
            Name: Name,
            Sym: Sym,
        };
        init
    };
    let mut Pairs: [LLVMOrcCSymbolMapPair; 1] = [Pair];
    return LLVMOrcAbsoluteSymbols(Pairs.as_mut_ptr(), 1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn generator(
    mut G: LLVMOrcDefinitionGeneratorRef,
    mut Ctx: *mut libc::c_void,
    mut LS: *mut LLVMOrcLookupStateRef,
    mut K: LLVMOrcLookupKind,
    mut JD: LLVMOrcJITDylibRef,
    mut F: LLVMOrcJITDylibLookupFlags,
    mut Names: LLVMOrcCLookupSet,
    mut NamesCount: size_t,
) -> LLVMErrorRef {
    let mut Resolve: Option::<ResolveFn> = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<ResolveFn>,
    >(Ctx);
    let mut I: size_t = 0 as libc::c_int as size_t;
    while I < NamesCount {
        let mut Element: LLVMOrcCLookupSetElement = *Names.offset(I as isize);
        let mut LinkerMangled: *const libc::c_char = LLVMOrcSymbolStringPoolEntryStr(
            Element.Name,
        );
        let mut MangledName: *const libc::c_char = dropLeadingUnderscores(LinkerMangled);
        let mut Handler: LLVMOrcJITTargetAddress = Resolve
            .expect("non-null function pointer")(MangledName);
        if Handler != 0 {
            fprintf(
                stderr,
                b"Undefined symbol %s: redirect to host function @ 0x%016llx\n\0"
                    as *const u8 as *const libc::c_char,
                MangledName,
                Handler,
            );
            let mut MU: LLVMOrcMaterializationUnitRef = redirect(Element.Name, Handler);
            let mut Err_0: LLVMErrorRef = LLVMOrcJITDylibDefine(JD, MU);
            if !Err_0.is_null() {
                return Err_0;
            }
        }
        I = (I as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return 0 as LLVMErrorRef;
}
#[no_mangle]
pub unsafe extern "C" fn addGenerator(
    mut Unit: LLVMOrcJITDylibRef,
    mut Resolve: Option::<ResolveFn>,
) {
    let mut Gen: LLVMOrcDefinitionGeneratorRef = LLVMOrcCreateCustomCAPIDefinitionGenerator(
        Some(
            generator
                as unsafe extern "C" fn(
                    LLVMOrcDefinitionGeneratorRef,
                    *mut libc::c_void,
                    *mut LLVMOrcLookupStateRef,
                    LLVMOrcLookupKind,
                    LLVMOrcJITDylibRef,
                    LLVMOrcJITDylibLookupFlags,
                    LLVMOrcCLookupSet,
                    size_t,
                ) -> LLVMErrorRef,
        ),
        ::core::mem::transmute::<Option::<ResolveFn>, *mut libc::c_void>(Resolve),
        None,
    );
    LLVMOrcJITDylibAddGenerator(Unit, Gen);
}
