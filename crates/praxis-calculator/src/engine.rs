use crate::calc::Calculator;
use crate::op::{BinaryOp, UnaryOp};
use crate::value::{AngleMode, Value};
use praxis_engine::{Action, Engine, Precondition, PreconditionResult, Situation};

impl Situation for Calculator {
    fn describe(&self) -> String {
        format!(
            "display={} memory={} mode={:?}",
            self.display, self.memory, self.angle_mode
        )
    }

    fn is_terminal(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcAction {
    Enter(Value),
    Unary(UnaryOp),
    Binary(BinaryOp, Value),
    Clear,
    AllClear,
    StoreMemory,
    RecallMemory,
    AddToMemory,
    ClearMemory,
    SetAngleMode(AngleMode),
}

impl Action for CalcAction {
    type Sit = Calculator;

    fn describe(&self) -> String {
        match self {
            CalcAction::Enter(v) => format!("enter {}", v),
            CalcAction::Unary(op) => format!("{:?}", op),
            CalcAction::Binary(op, v) => format!("{:?} {}", op, v),
            CalcAction::Clear => "clear".into(),
            CalcAction::AllClear => "all clear".into(),
            CalcAction::StoreMemory => "M store".into(),
            CalcAction::RecallMemory => "M recall".into(),
            CalcAction::AddToMemory => "M+".into(),
            CalcAction::ClearMemory => "MC".into(),
            CalcAction::SetAngleMode(m) => format!("angle mode {:?}", m),
        }
    }
}

/// Domain enforcement: checks if the operation is valid before applying.
pub struct DomainCheck;

impl Precondition<CalcAction> for DomainCheck {
    fn check(&self, calc: &Calculator, action: &CalcAction) -> PreconditionResult {
        match action {
            CalcAction::Unary(op) => match op.apply(&calc.display, calc.angle_mode) {
                Ok(_) => PreconditionResult::satisfied(
                    "domain_check",
                    &format!("{:?}({}) is valid", op, calc.display),
                ),
                Err(e) => PreconditionResult::violated(
                    "domain_check",
                    &format!("{}", e),
                    &calc.describe(),
                    &action.describe(),
                ),
            },
            CalcAction::Binary(op, rhs) => match op.apply(&calc.display, rhs) {
                Ok(_) => PreconditionResult::satisfied(
                    "domain_check",
                    &format!("{} {:?} {} is valid", calc.display, op, rhs),
                ),
                Err(e) => PreconditionResult::violated(
                    "domain_check",
                    &format!("{}", e),
                    &calc.describe(),
                    &action.describe(),
                ),
            },
            _ => PreconditionResult::satisfied("domain_check", "no domain constraints"),
        }
    }

    fn describe(&self) -> &str {
        "mathematical domain must be valid (no division by zero, sqrt of negative, etc.)"
    }
}

fn apply_calc(calc: &Calculator, action: &CalcAction) -> Calculator {
    let mut next = calc.clone();
    match action {
        CalcAction::Enter(v) => next.enter(v.clone()),
        CalcAction::Unary(op) => {
            let _ = next.unary(*op);
        }
        CalcAction::Binary(op, v) => {
            let _ = next.binary(*op, v.clone());
        }
        CalcAction::Clear => next.clear(),
        CalcAction::AllClear => next.all_clear(),
        CalcAction::StoreMemory => next.memory_op(crate::calc::MemoryOp::Store),
        CalcAction::RecallMemory => next.memory_op(crate::calc::MemoryOp::Recall),
        CalcAction::AddToMemory => next.memory_op(crate::calc::MemoryOp::Add),
        CalcAction::ClearMemory => next.memory_op(crate::calc::MemoryOp::Clear),
        CalcAction::SetAngleMode(m) => next.set_angle_mode(*m),
    }
    next
}

pub type CalcEngine = Engine<CalcAction>;

pub fn new_calculator() -> CalcEngine {
    Engine::new(Calculator::new(), vec![Box::new(DomainCheck)], apply_calc)
}
