/*
   Copyright 2017-2018 Jesse C. Grillo

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::vec::Vec;
use std::mem;

// push interpreter

#[derive(Debug)]
pub struct Config {
    min_random_integer: i64,
    max_random_integer: i64,
    min_random_float: f64,
    max_random_float: f64,
    max_points_in_random_expressions: usize,
    max_points_in_program: usize,
    eval_push_limit: usize,
    new_erc_name_probability: f64,
    random_seed: usize,
    top_level_pop_code: bool
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    state: State<'a>,
    config: Config
}

impl <'a> Interpreter<'a> {
    pub fn new(config: Config) -> Interpreter<'a> {
        Interpreter{
            state: State::new(),
            config
        }
    }

    pub fn run(&mut self) {
        while let Some(state) = self.state.advance() {
            unimplemented!() // FIXME: log some stuff?
        }
    }
}

// program state

#[derive(Debug)]
pub struct State<'a> {
    boolean_stack: Stack<bool>,
    code_stack: Stack<Program<'a>>,
    exec_stack: Stack<Program<'a>>,
    float_stack: Stack<f64>,
    integer_stack: Stack<i64>,
    name_stack: Stack<&'a str>
}

impl <'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            boolean_stack: Stack::new(),
            code_stack: Stack::new(),
            exec_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: Stack::new()
        }
    }

    pub fn advance(&mut self) -> Option<&State> {
        if let Some(exec) = self.exec_stack.get(0) {
            match exec {
                Program::Literal(literal) => {
                    // FIXME
                },
                Program::Instruction(instruction) => {
                    // FIXME
                },
                Program::List(list) => {
                    // FIXME
                }
            }

            Some(self)
        } else {
            None
        }
    }
}

// generic stack implementation

#[derive(Debug)]
pub struct Stack<T: Clone> {
    stack: Vec<T>
}

impl <T: Clone> Stack<T> { // FIXME: pull out all the common stack methods
    fn new() -> Stack<T> {
        Stack {
            stack: Vec::new()
        }
    }

    fn dup(&mut self) {
        if let Some(item) = self.pop() {
            let copied_item = item.clone();
            self.push(item);
            self.push(copied_item);
        }
    }

    fn flush(&mut self) {
        mem::replace(&mut self.stack, Vec::new());
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.stack.get(idx)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    fn rot(&mut self) {
        if self.len() >= 3 {
            let third_item = self.stack.remove(2);
            self.push(third_item);
        }
    }

    fn shove(&mut self, idx: usize) {
        if idx != 0 && idx < self.len() {
            self.stack.swap(0, idx);
        }
    }

    fn swap(&mut self) {
        let len = self.len();

        if len >= 2 {
            self.stack.swap(len - 2, len - 1);
        }
    }

    fn yank(&mut self, idx: usize) {
        if self.len() > idx {
            let item = self.stack.remove(idx);
            self.push(item);
        }
    }

    fn yank_dup(&mut self, idx: usize) {
        if self.len() > idx {
            let deep_item = self.stack[idx].clone();
            self.push(deep_item);
        }
    }
}

// "AST"

pub trait Dispatch<'a> {
    fn dispatch(self, state: &'a mut State<'a>);
}

#[derive(Debug, Clone)]
pub enum Program<'a> {
    Instruction(Instruction),
    Literal(Literal<'a>),
    List(Vec<Program<'a>>)
}

impl <'a> Dispatch<'a> for Program<'a> {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self {
            Program::Instruction(instruction) => instruction.dispatch(state),
            Program::Literal(literal) => literal.dispatch(state),
            Program::List(list) => {
                for program in list.into_iter().rev() {
                    state.exec_stack.push(program);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Boolean(Boolean),
    Code(Code),
    Exec(Exec),
    Float(Float),
    Integer(Integer),
    Name(Name)
}

impl <'a> Dispatch<'a> for Instruction {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self {
            Instruction::Boolean(boolean) => boolean.dispatch(state),
            Instruction::Code(code) => code.dispatch(state),
            Instruction::Exec(exec) => exec.dispatch(state),
            Instruction::Float(float) => float.dispatch(state),
            Instruction::Integer(integer) => integer.dispatch(state),
            Instruction::Name(name) => name.dispatch(state)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Boolean(bool),
    Float(f64),
    Integer(i64),
    Name(&'a str)
}

impl <'a> Dispatch<'a> for Literal<'a> {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self {
            Literal::Boolean(boolean) => state.boolean_stack.push(boolean),
            Literal::Float(float) => state.float_stack.push(float),
            Literal::Integer(integer) => state.integer_stack.push(integer),
            Literal::Name(name) => state.name_stack.push(name)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Boolean {
    Eq,
    And,
    Define,
    Dup,
    Flush,
    FromFloat,
    FromInteger,
    Not,
    Or,
    Pop,
    Rand,
    Rot,
    Shove,
    StackDepth,
    Swap,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Boolean {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Boolean::Eq => unimplemented!(),
            Boolean::And => unimplemented!(),
            Boolean::Define => unimplemented!(),
            Boolean::Dup => unimplemented!(),
            Boolean::Flush => unimplemented!(),
            Boolean::FromFloat => unimplemented!(),
            Boolean::FromInteger => unimplemented!(),
            Boolean::Not => unimplemented!(),
            Boolean::Or => unimplemented!(),
            Boolean::Pop => unimplemented!(),
            Boolean::Rand => unimplemented!(),
            Boolean::Rot => unimplemented!(),
            Boolean::Shove => unimplemented!(),
            Boolean::StackDepth => unimplemented!(),
            Boolean::Swap => unimplemented!(),
            Boolean::Yank => unimplemented!(),
            Boolean::YankDup => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Code {
    Eq,
    Append,
    Atom,
    Car,
    Cdr,
    Cons,
    Container,
    Contains,
    Define,
    Definition,
    Discrepancy,
    Do,
    DoStar,
    DoStarCount,
    DoStarRange,
    DoStarTimes,
    Dup,
    Extract,
    Flush,
    FromBoolean,
    FromFloat,
    FromInteger,
    FromName,
    If,
    Insert,
    Instructions,
    Length,
    List,
    Member,
    Noop,
    Nth,
    NthCdr,
    Null,
    Pop,
    Position,
    Quote,
    Rand,
    Rot,
    Shove,
    Size,
    StackDepth,
    Subst,
    Swap,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Code {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Code::Eq => unimplemented!(),
            Code::Append => unimplemented!(),
            Code::Atom => unimplemented!(),
            Code::Car => unimplemented!(),
            Code::Cdr => unimplemented!(),
            Code::Cons => unimplemented!(),
            Code::Container => unimplemented!(),
            Code::Contains => unimplemented!(),
            Code::Define => unimplemented!(),
            Code::Definition => unimplemented!(),
            Code::Discrepancy => unimplemented!(),
            Code::Do => unimplemented!(),
            Code::DoStar => unimplemented!(),
            Code::DoStarCount => unimplemented!(),
            Code::DoStarRange => unimplemented!(),
            Code::DoStarTimes => unimplemented!(),
            Code::Dup => unimplemented!(),
            Code::Extract => unimplemented!(),
            Code::Flush => unimplemented!(),
            Code::FromBoolean => unimplemented!(),
            Code::FromFloat => unimplemented!(),
            Code::FromInteger => unimplemented!(),
            Code::FromName => unimplemented!(),
            Code::If => unimplemented!(),
            Code::Insert => unimplemented!(),
            Code::Instructions => unimplemented!(),
            Code::Length => unimplemented!(),
            Code::List => unimplemented!(),
            Code::Member => unimplemented!(),
            Code::Noop => unimplemented!(),
            Code::Nth => unimplemented!(),
            Code::NthCdr => unimplemented!(),
            Code::Null => unimplemented!(),
            Code::Pop => unimplemented!(),
            Code::Position => unimplemented!(),
            Code::Quote => unimplemented!(),
            Code::Rand => unimplemented!(),
            Code::Rot => unimplemented!(),
            Code::Shove => unimplemented!(),
            Code::Size => unimplemented!(),
            Code::StackDepth => unimplemented!(),
            Code::Subst => unimplemented!(),
            Code::Swap => unimplemented!(),
            Code::Yank => unimplemented!(),
            Code::YankDup => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Exec {
    Eq,
    Define,
    DoStarCount,
    DoStarRange,
    DoStarTimes,
    Dup,
    Flush,
    If,
    K,
    Pop,
    Rot,
    S,
    Shove,
    StackDepth,
    Swap,
    Y,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Exec {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Exec::Eq => unimplemented!(),
            Exec::Define => unimplemented!(),
            Exec::DoStarCount => unimplemented!(),
            Exec::DoStarRange => unimplemented!(),
            Exec::DoStarTimes => unimplemented!(),
            Exec::Dup => unimplemented!(),
            Exec::Flush => unimplemented!(),
            Exec::If => unimplemented!(),
            Exec::K => unimplemented!(),
            Exec::Pop => unimplemented!(),
            Exec::Rot => unimplemented!(),
            Exec::S => unimplemented!(),
            Exec::Shove => unimplemented!(),
            Exec::StackDepth => unimplemented!(),
            Exec::Swap => unimplemented!(),
            Exec::Y => unimplemented!(),
            Exec::Yank => unimplemented!(),
            Exec::YankDup => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Float {
    Mod,
    Times,
    Plus,
    Minus,
    Divide,
    LessThan,
    Eq,
    GreaterThan,
    Cosine,
    Define,
    Dup,
    Flush,
    FromBoolean,
    FromInteger,
    Max,
    Min,
    Pop,
    Rand,
    Rot,
    Shove,
    Sine,
    Swap,
    Tangent,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Float {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Float::Mod => unimplemented!(),
            Float::Times => unimplemented!(),
            Float::Plus => unimplemented!(),
            Float::Minus => unimplemented!(),
            Float::Divide => unimplemented!(),
            Float::LessThan => unimplemented!(),
            Float::Eq => unimplemented!(),
            Float::GreaterThan => unimplemented!(),
            Float::Cosine => unimplemented!(),
            Float::Define => unimplemented!(),
            Float::Dup => unimplemented!(),
            Float::Flush => unimplemented!(),
            Float::FromBoolean => unimplemented!(),
            Float::FromInteger => unimplemented!(),
            Float::Max => unimplemented!(),
            Float::Min => unimplemented!(),
            Float::Pop => unimplemented!(),
            Float::Rand => unimplemented!(),
            Float::Rot => unimplemented!(),
            Float::Shove => unimplemented!(),
            Float::Sine => unimplemented!(),
            Float::Swap => unimplemented!(),
            Float::Tangent => unimplemented!(),
            Float::Yank => unimplemented!(),
            Float::YankDup => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Integer {
    Mod,
    Times,
    Plus,
    Minus,
    Divide,
    LessThan,
    Eq,
    GreaterThan,
    Define,
    Dup,
    Flush,
    FromBoolean,
    FromFloat,
    Max,
    Min,
    Pop,
    Rand,
    Rot,
    Shove,
    StackDepth,
    Swap,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Integer {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Integer::Mod => unimplemented!(),
            Integer::Times => unimplemented!(),
            Integer::Plus => unimplemented!(),
            Integer::Minus => unimplemented!(),
            Integer::Divide => unimplemented!(),
            Integer::LessThan => unimplemented!(),
            Integer::Eq => unimplemented!(),
            Integer::GreaterThan => unimplemented!(),
            Integer::Define => unimplemented!(),
            Integer::Dup => unimplemented!(),
            Integer::Flush => unimplemented!(),
            Integer::FromBoolean => unimplemented!(),
            Integer::FromFloat => unimplemented!(),
            Integer::Max => unimplemented!(),
            Integer::Min => unimplemented!(),
            Integer::Pop => unimplemented!(),
            Integer::Rand => unimplemented!(),
            Integer::Rot => unimplemented!(),
            Integer::Shove => unimplemented!(),
            Integer::StackDepth => unimplemented!(),
            Integer::Swap => unimplemented!(),
            Integer::Yank => unimplemented!(),
            Integer::YankDup => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    Eq,
    Dup,
    Flush,
    Pop,
    Quote,
    Rand,
    RandBoundName,
    Rot,
    Shove,
    StackDepth,
    Swap,
    Yank,
    YankDup
}

impl <'a> Dispatch<'a> for Name {
    fn dispatch(self, state: &'a mut State<'a>) {
        match self { // FIXME: implement
            Name::Eq => unimplemented!(),
            Name::Dup => unimplemented!(),
            Name::Flush => unimplemented!(),
            Name::Pop => unimplemented!(),
            Name::Quote => unimplemented!(),
            Name::Rand => unimplemented!(),
            Name::RandBoundName => unimplemented!(),
            Name::Rot => unimplemented!(),
            Name::Shove => unimplemented!(),
            Name::StackDepth => unimplemented!(),
            Name::Swap => unimplemented!(),
            Name::Yank => unimplemented!(),
            Name::YankDup => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
