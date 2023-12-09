#![feature(extern_types)]

mod generator;
mod module_build;
mod module_load;
mod utility;
mod zero_to_rust_jit;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut args = args.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
    println!("{}", args.len());
    unsafe {
        zero_to_rust_jit::main_0(
            args.len().try_into().unwrap(),
            args.as_mut_ptr() as *mut *const i8,
        );
    };
}
