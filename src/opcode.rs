use std::collections::HashMap;

// >	指针加一
// <	指针减一
// +	指针指向的字节的值加一
// -	指针指向的字节的值减一
// .	输出指针指向的单元内容（ASCII码）
// ,	输入内容到指针指向的单元（ASCII码）
// [	如果指针指向的单元值为零，向后跳转到对应的]指令的次一指令处
// ]	如果指针指向的单元值不为零，向前跳转到对应的[指令的次一指令处
#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3e,     // > ASCII值
    SHL = 0x3c,     // <
    ADD = 0x2b,     // +
    SUB = 0x2d,     // -
    PUTCHAR = 0x2e, // .
    GETCHAR = 0x2c, // ,
    LB = 0x5b,      // [
    RB = 0x5d,      // ]
}

// 文件里面读取的是u8，而这里需要的是Opcode，需要转换一下
impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3e => Opcode::SHR,     // > ASCII值
            0x3c => Opcode::SHL,     // <
            0x2b => Opcode::ADD,     // +
            0x2d => Opcode::SUB,     // -
            0x2e => Opcode::PUTCHAR, // .
            0x2c => Opcode::GETCHAR, // ,
            0x5b => Opcode::LB,      // [
            0x5d => Opcode::RB,      // ]
            _ => unimplemented!(),   // u8里面除了这八个符号，还有其他的符号，其他符号程序退出
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>, // 指令
    pub jtable: HashMap<usize, usize>, // 为了加速，语言的执行，流程跳转使用左方括号和右方括号，
                             // 左右方括号永远是一对的，当代码读到左方括号的时候，就可以提前知道他的右方括号，在那个位置
                             // 读到右方括号时，就知道左方括号的位置，这样就会让代码的执行速度变快
                             //  把配对关系放到jtable里面
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        // 把所有的Opcode填到里面
        // 做一个字典
        // data里面的数据一定要在字典里面

        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,     // > ASCII值
            Opcode::SHL as u8,     // <
            Opcode::ADD as u8,     // +
            Opcode::SUB as u8,     // -
            Opcode::PUTCHAR as u8, // .
            Opcode::GETCHAR as u8, // ,
            Opcode::LB as u8,      // [
            Opcode::RB as u8,      // ]
        ];

        // 忽略不是这8个操作码的值
        let mut temp: Vec<Opcode> = vec![];
        for index in 0..data.len() {
            let mut falg = false;
            let item = data[index];
            for &i in dict.iter() {
                if item == i {
                    falg = true;
                    break;
                }
            }
            if falg {
                temp.push(Opcode::from(item))
            } else {
                // println!("其他字符：{:?} {}", std::str::from_utf8(&[item]), index)
            }
        }
        let instrs = temp;
        // println!("{:?}", data);

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i /*索引*/, e /*值*/) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                // 匹配到做小括号[则入栈索引
                jstack.push(i);
            }
            if Opcode::RB == *e {
                // 匹配到右小括号，查看栈中是否有做小括号的索引，有则出栈，无则报错
                // ok_or: Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
                // 这种处理方式比较好
                let j = jstack.pop().ok_or_else(|| {
                    println!("pop from empty list");
                    return "pop from empty list";
                })?;
                jtable.insert(j, i);
                jtable.insert(i, j);

                // let j = jstack.pop(); // Some
                // match j {
                //     Some(data) => {
                //         jtable.insert(data, i);
                //         jtable.insert(i, data);
                //     }
                //     None => {
                //         println!("pop from empty list");
                //     }
                // }
            }
        }

        Ok(Self { instrs, jtable })
    }
}
