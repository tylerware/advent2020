use std::str::FromStr;
use std::string::ParseError;

#[derive(Clone,Copy)]
pub enum OperationType {
    Jump,
    Accumulator,
    NoOperation
}

#[derive(Clone,Copy)]
pub struct Operation {
    pub op_type: OperationType,
    pub arg: i32
}

impl FromStr for Operation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op_val: Vec<&str> = s.split(' ')
                                .collect();
        let operation = op_val.get(0).unwrap().to_owned();
        let arg = i32::from_str(op_val.get(1).unwrap()).unwrap();

        let operation_type;
        match operation {
            "acc" => operation_type = OperationType::Accumulator,
            "jmp" => operation_type = OperationType::Jump,
            _ => operation_type = OperationType::NoOperation
        }

        Ok(Operation {
            op_type: operation_type,
            arg: arg
        })
    }
}
