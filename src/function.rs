use crate::{instruction::Instruction, data_type::DataType};

pub struct Function {
    pub arguments: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub instructions: Vec<Instruction>
}