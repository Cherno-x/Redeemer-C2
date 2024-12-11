use coffee_ldr::loader::Coffee;
use std::error::Error;
use eyre::{Report, Result};

pub fn load_coff(bof_file:&Vec<u8>)->Result<String,Box<dyn Error>> {
    let output = Coffee::new(bof_file.as_slice())
        .map_err(|e| Report::msg(format!("Error loading BOF: {e}")))?
        .execute(
            None,
            None,
            None
        )
        .map_err(|e| Report::msg(format!("Error executing BOF: {e}")))?;

    //println!("Execution output: {output}");
    Ok(output)
} 

