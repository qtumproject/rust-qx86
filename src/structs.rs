
#[allow(dead_code)] //remove after design stuff is done

use crate::vm::*;

pub const MAX_ARGS:usize = 3;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ValueSize{
    None,
    Byte,
    Word,
    Dword
}



#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Register{
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    Segment,
    Null
}


#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum SizedValue{
    None,
    Byte(u8),
    Word(u16),
    Dword(u32)
}

impl SizedValue{
    pub fn u32_exact(&self) -> Result<u32, VMError>{
        match self{
            SizedValue::Dword(v) => Ok(*v),
            _ => Err(VMError::WrongSizeExpectation)
        }
    }
    pub fn u16_exact(&self) -> Result<u16, VMError>{
        match self{
            SizedValue::Word(v) => Ok(*v),
            _ => Err(VMError::WrongSizeExpectation)
        }
    }
    pub fn u8_exact(&self) -> Result<u8, VMError>{
        match self{
            SizedValue::Byte(v) => Ok(*v),
            _ => Err(VMError::WrongSizeExpectation)
        }
    }
    
    //zx = zero extend to fit into integer size
    pub fn u32_zx(&self) ->  Result<u32, VMError>{
        match self{
            SizedValue::Dword(v) => Ok(*v),
            SizedValue::Word(v) => Ok(*v as u32),
            SizedValue::Byte(v) => Ok(*v as u32),
            SizedValue::None => Ok(0),
        }
    }
    pub fn u16_zx(&self) ->  Result<u16, VMError>{
        match self{
            SizedValue::Word(v) => Ok(*v),
            SizedValue::Byte(v) => Ok(*v as u16),
            SizedValue::None => Ok(0),
            SizedValue::Dword(v) => Err(VMError::TooBigSizeExpectation)
        }
    }
    //sx = signed extend to fit into integer size
    pub fn u32_sx(&self) ->  Result<u32, VMError>{
        match self{
            SizedValue::Dword(v) => Ok(*v),
            SizedValue::Word(v) => Ok((*v as i32) as u32),
            SizedValue::Byte(v) => Ok((*v as i32) as u32),
            SizedValue::None => Ok(0),
        }
    }
    pub fn u16_sx(&self) ->  Result<u16, VMError>{
        match self{
            SizedValue::Word(v) => Ok(*v),
            SizedValue::Byte(v) => Ok((*v as i16) as u16),
            SizedValue::None => Ok(0),
            SizedValue::Dword(v) => Err(VMError::TooBigSizeExpectation)
        }
    }

    //trunc = make fit into the specified type even if data is lost
    //If the type is larger then equivalent to zero-extend
    //top bytes will be cut when casting to a smaller type
    //Note these can not error
    pub fn u32_trunc(&self) ->  u32{
        match self{
            SizedValue::Dword(v) => *v,
            SizedValue::Word(v) => *v as u32,
            SizedValue::Byte(v) => *v as u32,
            SizedValue::None => 0,
        }
    }
    pub fn u16_trunc(&self) ->  u16{
        match self{
            SizedValue::Word(v) => *v as u16,
            SizedValue::Byte(v) => *v as u16,
            SizedValue::None => 0,
            SizedValue::Dword(v) => *v as u16
        }
    }
    pub fn u8_trunc(&self) -> u8{
        match self{
            SizedValue::Word(v) => *v as u8,
            SizedValue::Byte(v) => *v as u8,
            SizedValue::None => 0,
            SizedValue::Dword(v) => *v as u8
        }
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ArgLocation{
    None,
    Immediate(SizedValue),
    Address(u32, ValueSize), //an immediate address
    RegisterValue(u8, ValueSize),
    RegisterAddress(u8, ValueSize),
    /*ModRMAddress16{ //Not supported except for with LEA
        offset: Option<u16>, 
        reg1: Option<u8>,
        reg2: Option<u8>,
        size: ValueSize
    }, */
    ModRMAddress{
        offset: Option<u32>,
        reg: Option<u8>,
        size: ValueSize
    },
    SIBAddress{
        offset: Option<u32>,
        base: Option<u8>, //register
        scale: u8, //1, 2, 4, 8
        index: Option<u8>,
        size: ValueSize
    }
}

#[derive(Copy, Clone)]
pub struct OpArgument{
    pub location: ArgLocation,
    pub size: u8 //size in bytes
}





impl Default for OpArgument{
    fn default() -> OpArgument{
        OpArgument{
            location: ArgLocation::None,
            size: 0
        }
    }
}

