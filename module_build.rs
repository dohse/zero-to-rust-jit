#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LLVMOpaqueContext;
    pub type LLVMOpaqueModule;
    pub type LLVMOpaqueType;
    pub type LLVMOpaqueValue;
    pub type LLVMOpaqueBasicBlock;
    pub type LLVMOpaqueBuilder;
    pub type LLVMOrcOpaqueThreadSafeContext;
    fn LLVMModuleCreateWithNameInContext(
        ModuleID: *const libc::c_char,
        C: LLVMContextRef,
    ) -> LLVMModuleRef;
    fn LLVMAddFunction(
        M: LLVMModuleRef,
        Name: *const libc::c_char,
        FunctionTy: LLVMTypeRef,
    ) -> LLVMValueRef;
    fn LLVMInt32Type() -> LLVMTypeRef;
    fn LLVMFunctionType(
        ReturnType: LLVMTypeRef,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: libc::c_uint,
        IsVarArg: LLVMBool,
    ) -> LLVMTypeRef;
    fn LLVMGetParam(Fn: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
    fn LLVMAppendBasicBlock(
        Fn: LLVMValueRef,
        Name: *const libc::c_char,
    ) -> LLVMBasicBlockRef;
    fn LLVMCreateBuilder() -> LLVMBuilderRef;
    fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
    fn LLVMDisposeBuilder(Builder: LLVMBuilderRef);
    fn LLVMBuildRet(_: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
    fn LLVMBuildAdd(
        _: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const libc::c_char,
    ) -> LLVMValueRef;
    fn LLVMOrcThreadSafeContextGetContext(
        TSCtx: LLVMOrcThreadSafeContextRef,
    ) -> LLVMContextRef;
}
pub type LLVMBool = libc::c_int;
pub type LLVMContextRef = *mut LLVMOpaqueContext;
pub type LLVMModuleRef = *mut LLVMOpaqueModule;
pub type LLVMTypeRef = *mut LLVMOpaqueType;
pub type LLVMValueRef = *mut LLVMOpaqueValue;
pub type LLVMBasicBlockRef = *mut LLVMOpaqueBasicBlock;
pub type LLVMBuilderRef = *mut LLVMOpaqueBuilder;
pub type LLVMOrcThreadSafeContextRef = *mut LLVMOrcOpaqueThreadSafeContext;
#[no_mangle]
pub unsafe extern "C" fn buildModule(
    mut Ctx: LLVMOrcThreadSafeContextRef,
) -> LLVMModuleRef {
    let mut BareCtx: LLVMContextRef = LLVMOrcThreadSafeContextGetContext(Ctx);
    let mut M: LLVMModuleRef = LLVMModuleCreateWithNameInContext(
        b"demo\0" as *const u8 as *const libc::c_char,
        BareCtx,
    );
    let mut ParamTypes: [LLVMTypeRef; 2] = [LLVMInt32Type(), LLVMInt32Type()];
    let mut SumFunctionType: LLVMTypeRef = LLVMFunctionType(
        LLVMInt32Type(),
        ParamTypes.as_mut_ptr(),
        2 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    );
    let mut SumFunction: LLVMValueRef = LLVMAddFunction(
        M,
        b"sum\0" as *const u8 as *const libc::c_char,
        SumFunctionType,
    );
    let mut EntryBB: LLVMBasicBlockRef = LLVMAppendBasicBlock(
        SumFunction,
        b"entry\0" as *const u8 as *const libc::c_char,
    );
    let mut Builder: LLVMBuilderRef = LLVMCreateBuilder();
    LLVMPositionBuilderAtEnd(Builder, EntryBB);
    let mut SumArg0: LLVMValueRef = LLVMGetParam(
        SumFunction,
        0 as libc::c_int as libc::c_uint,
    );
    let mut SumArg1: LLVMValueRef = LLVMGetParam(
        SumFunction,
        1 as libc::c_int as libc::c_uint,
    );
    let mut Result: LLVMValueRef = LLVMBuildAdd(
        Builder,
        SumArg0,
        SumArg1,
        b"result\0" as *const u8 as *const libc::c_char,
    );
    LLVMBuildRet(Builder, Result);
    LLVMDisposeBuilder(Builder);
    return M;
}
