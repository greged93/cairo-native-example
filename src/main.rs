use cairo_felt::Felt252;
use cairo_native::context::NativeContext;
use cairo_native::executor::NativeExecutor;
use cairo_native::values::JITValue;
use std::path::Path;

fn main() {
    let program_path = Path::new("./programs/fib.cairo");
    // Compile the cairo program to sierra.
    let sierra_program = cairo_native::utils::cairo_to_sierra(program_path);

    // Instantiate a Cairo Native MLIR context. This data structure is responsible for the MLIR
    // initialization and compilation of sierra programs into a MLIR module.
    let native_context = NativeContext::new();

    // Compile the sierra program into a MLIR module.
    let native_program = native_context.compile(&sierra_program).unwrap();

    // The parameters of the entry point.
    let params = &[];

    // Find the entry point id by its name.
    let entry_point = "fib::fib::main";
    let entry_point_id = cairo_native::utils::find_function_id(&sierra_program, entry_point);

    // Instantiate the executor.
    let native_executor = NativeExecutor::new(native_program);

    // Execute the program.
    let result = native_executor
        .execute(entry_point_id, params, Some(1000000))
        .unwrap();

    println!("Cairo program was compiled and executed successfully.");
    println!("{:?}", result);
}
