use crate::vm::stack::Stack;

pub type OpcodeHandler = fn(&[usize], usize, &mut Vec<u8>) -> usize;

pub fn handle_nop(params: &[usize], offset: usize, _stack: &mut Vec<u8>) -> usize {
    assert_eq!(params.len(), 0);

    println!("NOP");

    return offset + 1;
}

pub fn handle_return(params: &[usize], offset: usize, stack: &mut Vec<u8>) -> usize {
    assert_eq!(params.len(), 1);

    println!("RETURN {} ({})", params[0], stack.get_byte(params[0]));

    stack.push(stack.get_byte(params[0]));

    return offset + 1 + params.len() * 4;
}

pub fn handle_add(params: &[usize], offset: usize, stack: &mut Vec<u8>) -> usize {
    assert_eq!(params.len(), 2);

    println!("ADD {} ({}) {} ({})", params[0], stack.get_byte(params[0]), params[1], stack.get_byte(params[1]));

    stack.push(stack.get_byte(params[0]) + stack.get_byte(params[1]));

    return offset + 1 + params.len() * 4;
}

pub fn handle_constant(params: &[usize], offset: usize, stack: &mut Vec<u8>) -> usize {
    assert_eq!(params.len(), 1);

    println!("CONSTANT {}", params[0]);

    stack.push(params[0] as u8);

    return offset + 1 + params.len() * 4;
}
