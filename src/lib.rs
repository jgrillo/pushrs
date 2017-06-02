/*
   Copyright 2017 Jesse C. Grillo

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

enum Type {
    Boolean,
    Code,
    Exec,
    Float,
    Integer,
    Name
}

enum Boolean {
    Eq,
    And,
    Define,
    Dup,
    Flush,
    Fromfloat,
    Frominteger,
    Not,
    Or,
    Pop,
    Rand,
    Rot,
    Shove,
    Stackdepth,
    Swap,
    Yank,
    Yankdup
}

enum Code {
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
    Fromboolean,
    Fromfloat,
    Frominteger,
    Fromname,
    If,
    Insert,
    Instructions,
    Length,
    List,
    Member,
    Noop,
    Nth,
    Nthcdr,
    Null,
    Pop,
    Position,
    Quote,
    Rand,
    Rot,
    Shove,
    Size,
    Stackdepth,
    Subst,
    Swap,
    Yank,
    Yankdup
}

enum Exec {
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
    Stackdepth,
    Swap,
    Y,
    Yank,
    Yankdup
}

enum Float {
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
    Fromboolean,
    Frominteger,
    Max,
    Min,
    Pop,
    Rand,
    Rot,
    Shove,
    Sin,
    Stackdepth,
    Swap,
    Tan,
    Yank,
    Yankdup
}

enum Integer {
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
    Fromboolean,
    Fromfloat,
    Max,
    Min,
    Pop,
    Rand,
    Rot,
    Shove,
    Stackdepth,
    Swap,
    Yank,
    Yankdup
}

enum Name {
    Eq,
    Dup,
    Flush,
    Pop,
    Quote,
    Rand,
    Randboundname,
    Rot,
    Shove,
    Stackdepth,
    Swap,
    Yank,
    Yankdup
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
