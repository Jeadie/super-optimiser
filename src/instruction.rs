use std::error::Error;
use std::str::FromStr;
use std::fmt;


#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Load(u32),
    Swap(usize, usize),
    Xor(usize, usize),
    Inc(usize),
}
pub type Program = Vec<Instruction>;



#[derive(Debug, PartialEq, Eq)]
pub struct ParseInstructionError {
    details: String
}


impl ParseInstructionError {
    fn new(msg: &str) -> ParseInstructionError {
        ParseInstructionError{details: msg.to_string()}
    }
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParseInstructionError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ParseInstructionError::new(format!("String: '{:?}' does not satisfy format", s).as_str()));
        }
        
        let op_str = parts[0];
        let args_str: Vec<_> = parts.iter().skip(1).map(|&s| s.parse::<u32>().unwrap_or_default()).collect();

        if args_str.len() < 2 && (op_str == "SWAP" || op_str == "XOR") {
            return Err(ParseInstructionError::new(format!("Not enough arguments for {} operation", op_str).as_str()));
        }

        match op_str {
            "LOAD" => Ok(Instruction::Load(args_str[0])),
            "SWAP" => Ok(Instruction::Swap(args_str[0] as usize, args_str[1] as usize)),
            "XOR" => Ok(Instruction::Xor(args_str[0] as usize, args_str[1] as usize)),
            "INC" => Ok(Instruction::Inc(args_str[0] as usize)),
            _ => Err(ParseInstructionError::new(format!("Operation: {} not valid", op_str).as_str())),
        }
    }
    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let re = Regex::new(r"(\w+)\s+([-\d]+)(?:,\s*([-\d]+)(?:,\s*([-\d]+))?)?").unwrap();
    //     if let Some(cap) = re.captures(s) {
    //         let op_str = &cap[1];
    //         let args_str: Vec<_> = cap.iter().skip(2).collect();
            
    //         match op_str {
    //             "LOAD" => Ok(Instruction::Load(args_str[0].unwrap().as_str().parse::<u32>().map_err(|e| ParseInstructionError::new(e.to_string().as_str()))?)),
    //             "SWAP" => Ok(Instruction::Swap(args_str[0].unwrap().as_str().parse::<usize>().map_err(|e| ParseInstructionError::new(e.to_string().as_str()))?, args_str[1].unwrap().as_str().parse().map_err(|_| ParseInstructionError::new(format!("failed to parse {:?}", args_str[1]).as_str()))?)),
    //             "XOR" => Ok(Instruction::Xor(args_str[0].unwrap().as_str().parse::<usize>().map_err(|e| ParseInstructionError::new(e.to_string().as_str()))?, args_str[1].unwrap().as_str().parse().map_err(|_| ParseInstructionError::new(format!("failed to parse {:?}", args_str[1]).as_str()))?)),
    //             "INC" => Ok(Instruction::Inc(args_str[0].unwrap().as_str().parse::<usize>().map_err(|e| ParseInstructionError::new(e.to_string().as_str()))?)),
    //             _ => Result::Err(ParseInstructionError::new(format!("Operation: {} not valid", op_str).as_str())),
    //         }
    //     } else {
    //         Result::Err(ParseInstructionError::new(format!("String: '{:?}' does not satisfy regex", s).as_str()))
    //     }
    // }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Load(val) => write!(f, "LOAD {}", val),
            Instruction::Swap(mem1, mem2) => write!(f, "SWAP {}, {}", mem1, mem2),
            Instruction::Xor(mem1, mem2) => write!(f, "XOR {}, {}", mem1, mem2),
            Instruction::Inc(mem) => write!(f, "INC {}", mem),
        }
    }
}