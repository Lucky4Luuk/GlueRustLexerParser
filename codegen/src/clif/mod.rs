pub(crate) mod func;
pub(crate) mod stmt;
pub(crate) mod variable;
pub(crate) mod expr;

use cranelift::codegen::Context;
use cranelift::codegen::settings;
use cranelift::codegen::binemit::{NullRelocSink, NullTrapSink, NullStackMapSink};

use cranelift_object::*;
use cranelift_module::{Module, Linkage};

pub fn gen(obj_name: &str, flags: settings::Flags, target: crate::CompileTarget, hir_root: hir::Root) {
    let obj_builder = ObjectBuilder::new(target.isa, obj_name, cranelift_module::default_libcall_names()).expect("Failed to construct object builder!");
    let mut obj_module = ObjectModule::new(obj_builder);
    for hir_func in hir_root.functions {
        let func = func::build_func(flags.clone(), target.call_conv, &hir_func);
        let mut context = Context::for_function(func.clone());
        let mut traps_sink = NullTrapSink {};
        let mut stack_maps_sink = NullStackMapSink {};
        println!("Function name: `{}`", &hir_func.name);
        let func_id = obj_module.declare_function(&hir_func.name, Linkage::Local, &func.signature).expect("Failed to declare function!");
        obj_module.define_function(func_id, &mut context, &mut traps_sink, &mut stack_maps_sink).expect("Failed to compile function");
    }
    let obj_product = obj_module.finish();
    let mem = obj_product.emit().expect("Failed to emit to memory!");
    dbg!(mem.len());
    println!("Writing {} bytes to {}.o!", mem.len(), obj_name);
    std::fs::write(format!("{}.o", obj_name), &mem[..]).expect(&format!("Failed to write to {}.o!", obj_name));
}
