
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
                // if n_operands == 1 {
                //     inst.execute(operands.pop()) 
                // }   
                // else if n_operands == 2 {
                    
                // }
            }
            // for operand in n_operands{
            //     rhs = operands.pop();
            //     if let Some(rhs) = rhs {

            //     }
            // }

        }
    }
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