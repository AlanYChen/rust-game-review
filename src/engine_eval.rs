
#[derive(Debug)]
pub enum EvalType {Centipawn, Mate}

impl EvalType {
    pub fn from_str(str: &str) -> EvalType {
        match str {
            "cp" => EvalType::Centipawn,
            "mate" => EvalType::Mate,
            _ => panic!("Unable to create eval type")
        }
    }
}

#[derive(Debug)]
pub struct EngineEval {
    eval_type: EvalType,
    value: i8,
}

impl EngineEval {
    pub fn new(eval_type: EvalType, value: i8) -> EngineEval {
        EngineEval { eval_type: eval_type, value: value }
    }
}