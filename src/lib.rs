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

pub trait Type {
    fn eq(&self, &mut state: State);

    fn dup(&self, &mut state: State);

    fn flush(&self, &mut state: State);

    fn pop(&self, &mut state: State);

    fn rand(&self, &mut state: State);

    fn rot(&self, &mut state: State);

    fn shove(&self, &mut state: State);

    fn stack_depth(&self, &mut state: State);

    fn swap(&self, &mut state: State);

    fn yank(&self, &mut state: State);

    fn yank_dup(&self, &mut state: State);
}

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
    Do_,
    DoCount,
    DoRange,
    DoTimes,
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

pub enum Exec {
    Eq,
    Define,
    DoCount,
    DoRange,
    DoTimes,
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

pub enum Float {
    Mod,
    Mul,
    Add,
    Sub,
    Div,
    Lt,
    Eq,
    Gt,
    Cos,
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
    Sin,
    StackDepth,
    Swap,
    Tan,
    Yank,
    YankDup
}

pub enum Integer {
    Mod,
    Mul,
    Add,
    Sub,
    Div,
    Lt,
    Eq,
    Gt,
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

pub struct Stack<T> where T: Clone {
    stack: Vec<T>
}

impl <T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.stack.get(idx)
    }

    pub fn yank(&mut self, idx: usize) -> Option<T> {
        if self.stack.len() > idx {
            Some(self.stack.remove(idx))
        } else {
            None
        }
    }

    pub fn shove(&mut self, idx: usize) {
        if idx != 0 && idx < self.stack.len() {
            self.stack.swap(0, idx);
        }
    }

    pub fn yank_dup(&mut self, idx: usize) {
        if let Some(deep_item) = self.stack.get(idx) {
            self.stack.push(deep_item.clone());
        }
    }

    pub fn swap(&mut self) {
        let len = self.stack.len();

        if len >= 2 {
            self.stack.swap(len - 2, len - 1);
        }
    }

    pub fn rot(&mut self) {
        if self.stack.len() >= 3 {
            let third_item = self.stack.remove(2);
            self.stack.push(third_item);
        }
    }

    pub fn dup(&mut self) {
        if let Some(item) = self.stack.pop() {
            let copied_item = item.clone();
            self.stack.push(item);
            self.stack.push(copied_item);
        }
    }

    pub fn flush(&mut self) {
        mem::replace(&mut self.stack, Vec::new());
    }
}

pub struct State {
    boolean_stack: Stack<Boolean>,
    code_stack: Stack<Code>,
    exec_stack: Stack<Exec>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: Stack<Name>
}

impl State {
    pub fn new() -> State {
        State {
            boolean_stack: Stack::new(),
            code_stack: Stack::new(),
            exec_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: Stack::new()
        }
    }

    pub fn borrow_type_stack(&mut self) -> &mut Stack<Type> {
        &mut self.type_stack
    }

    pub fn borrow_boolean_stack(&mut self) -> &mut Stack<Boolean> {
        &mut self.boolean_stack
    }

    pub fn borrow_code_stack(&mut self) -> &mut Stack<Code> {
        &mut self.code_stack
    }

    pub fn borrow_exec_stack(&mut self) -> &mut Stack<Exec> {
        &mut self.exec_stack
    }

    pub fn borrow_float_stack(&mut self) -> &mut Stack<Float> {
        &mut self.float_stack
    }

    pub fn borrow_integer_stack(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
    }

    pub fn borrow_name_stack(&mut self) -> &mut Stack<Name> {
        &mut self.name_stack
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
