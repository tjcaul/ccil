use crate::{compiler::Compiler, vm::{chunk::Chunk, opcode::Argument}};

// todo: turn these into macros
impl Compiler<'_> {
    pub fn emit_instr(&self, instruction: &str) -> Vec<u8> {
        vec![self.lookup.from_symbol(instruction).unwrap().byte]
    }

    pub fn emit_constant(&self, value: Argument) -> Vec<u8> {
        let const_opcode = self.lookup.from_symbol("CONST").unwrap();
        let mut retval = vec![const_opcode.byte];
        retval.write_arg(value);

        return retval;
    }

    pub fn emit_assignment(&self, var_id: Argument, type_id: Argument) -> Vec<u8> {
        let store_opcode = self.lookup.from_symbol("STORE").unwrap();
        let mut retval = vec![store_opcode.byte];
        retval.write_arg(var_id);
        retval.write_arg(type_id);

        return retval;
    }

    pub fn emit_load(&self, var_id: Argument) -> Vec<u8> {
        let load_opcode = self.lookup.from_symbol("LOAD").unwrap();
        let mut retval = vec![load_opcode.byte];
        retval.write_arg(var_id);

        return retval;
    }

    pub fn emit_write(&self, fileno: Argument) -> Vec<u8> {
        let writes_opcode = self.lookup.from_symbol("WRITE").unwrap();
        let mut retval = vec![writes_opcode.byte];
        retval.write_arg(fileno);

        return retval;
    }

    pub fn emit_writes(&self, fileno: Argument) -> Vec<u8> {
        let writes_opcode = self.lookup.from_symbol("WRITES").unwrap();
        let mut retval = vec![writes_opcode.byte];
        retval.write_arg(fileno);

        return retval;
    }
}

