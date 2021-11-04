pub mod interpreter;
pub mod opcode;

use crate::interpreter::Interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(String::from(&args[1]))?;
    // let code = Code::from(data)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(data)?;
    // println!("{:?}", code.instrs);
    Ok(())
}
