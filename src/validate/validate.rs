
pub fn validate() {
    let mut operators = [];
    let mut operands = [];
    let mut code = [I32CONST, I32CONST, I32ADD, I32CONST, I32ADD];
    for inst in code {
        if inst.is_operand(){
            operands.push(inst);
        }
        if inst.is_operator(){
            n_operands = get_n_operands(inst);
            if len(operands) >= n_operands() {
                inst.execute(operands.get_slice());
            }
            return false;
        }
    }
    if operands.len() >= 0{
        return false;
    }
    return true;
}

static i32_range = (0, 20);
static i64_range = (20, 60);
pub fn opcode_type(opcode: OP){
    if opcode >= i32_range.0 and opcode <= i32_range {
        return I32;
    }
    else if ....

}

pub fn execute(n_args: OP, operand : OP){
    type = operand.type();
    for arg in n_args(){
        if arg.type() != type{
            return false;
        }
    }
    return true;
}