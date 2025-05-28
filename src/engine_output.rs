use crate::engine_eval::EngineEval;

#[derive(Debug)]
pub struct EngineOutput {
    eval: EngineEval,
    best_move: String,
}

impl EngineOutput {
    pub fn new(eval: EngineEval, best_move: String) -> EngineOutput {
        EngineOutput { eval: eval, best_move: best_move }
    }
}