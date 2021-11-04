use std::io::{Read, Write};

use crate::opcode::{Code, Opcode};
pub struct Interpreter {
    stack: Vec<u8>, // 无限长的纸带
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: vec![0; 10], // 在new 里面初始化一下，纸带默认长度是1，初始化的数据是0
        }
    }

    // run里面可能会出现数据错误
    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?; // 先做转换，将原始的data转换成code
        let code_len = code.instrs.len();
        let mut pc = 0; // Programm Counter 指示代码已经运行到那里面了 现在代码和纸带都是Vec<u8> pc表示代码已经执行到哪个指令了
        let mut sp = 0; // stack pointer 无限长的纸带，现在的指针指向哪里

        loop {
            // 解释器主循环
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHR => {
                    sp += 1;
                    let len = self.stack.len();
                    assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
                    if sp == len {
                        let temp = vec![0 as u8; len];
                        self.stack = [self.stack.clone(), temp].concat();
                    }
                } // > ASCII值
                Opcode::SHL => {
                    if sp != 0 {
                        sp -= 1;
                    }
                } // <
                Opcode::ADD => {
                    // u8(0-255)加法可能溢出，使用overflowing_add
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                } // +
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                } // -
                Opcode::PUTCHAR => {
                    // 标准输出，打印到屏幕上
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                } // .
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?; // 必须要从标准输入里面取得一个字符
                    self.stack[sp] = buf[0]; // 把用户输入存储到纸带上去
                } // ,
                Opcode::LB => {
                    // 如果此时对应的位置是0，则跳到对应右边小括号的位置
                    // 跳转
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                } // [
                Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                } // ]
            }
            pc += 1; // 当前指令执行完了，pc+1去执行下一个指令
        }
        Ok(())
    }
}
