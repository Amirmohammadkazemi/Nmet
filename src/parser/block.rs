use rand::random;

/**********************************************************************************************
*
*   parser/block: parse blocks syntax (e.g: if/while block)
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use crate::{
    lexer::{Lexer, TokenType},
    parser::stmt::Stmt,
    utils::long2base32,
};

use super::{
    assign::assign, expr::expr, preprocessing::parse_pre_functions, stmt::{for_loop, if_stmt, while_stmt, StmtType}, variable_decl::variable_declare
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Condition,
    Loop,
    Function,
}

/// Block Stmt
/// Holds a list of stmt in a block of code
#[derive(Debug, Clone)]
pub struct Block {
    pub master: String,
    pub stmts: Vec<Stmt>,
    pub btype: BlockType,
    pub id: i64,
}

impl Block {
    pub fn new(master: String, btype: BlockType, stmts: Vec<Stmt>) -> Self {
        let id = random();
        Self {
            master,
            stmts,
            btype,
            id,
        }
    }

    pub fn start_name(&self) -> String {
        if self.btype == BlockType::Function {
            self.master.clone()
        } else {
            format!("{}.BS__{}", self.master, long2base32(self.id))
        }
    }

    pub fn end_name(&self) -> String {
        if self.btype == BlockType::Function {
            format!("{}.Defer", self.master)
        } else {
            format!("{}.BE__{}", self.master, long2base32(self.id))
        }
    }

    pub fn name_with_prefix(&self, prefix: &str) -> String {
        format!("{}.{prefix}__{}", self.master, long2base32(self.id))
    }
}

pub fn parse_stmt(lexer: &mut Lexer, master: &String) -> Vec<Stmt> {
    match lexer.get_token_type() {
        TokenType::Hash => {
            let loc = lexer.get_token_loc();
            parse_pre_functions(lexer, loc, master)
        }
        TokenType::Var => {
            let loc = lexer.get_token_loc();
            let stmt = vec![Stmt {
                stype: StmtType::VariableDecl(variable_declare(lexer)),
                loc,
            }];
            lexer.match_token(TokenType::SemiColon);
            stmt
        }
        TokenType::Print => {
            let loc = lexer.get_token_loc();
            lexer.match_token(TokenType::Print);
            let expr = expr(lexer);
            let stmt = vec![Stmt {
                stype: StmtType::Print(expr),
                loc,
            }];
            lexer.match_token(TokenType::SemiColon);
            stmt
        }
        TokenType::Break => {
            let loc = lexer.get_token_loc();
            lexer.match_token(TokenType::Break);
            let stmt = vec![Stmt {
                stype: StmtType::Break,
                loc,
            }];
            lexer.match_token(TokenType::SemiColon);
            stmt
        }
        TokenType::Continue => {
            let loc = lexer.get_token_loc();
            lexer.match_token(TokenType::Continue);
            let stmt = vec![Stmt {
                stype: StmtType::Continue,
                loc,
            }];
            lexer.match_token(TokenType::SemiColon);
            stmt
        }
        TokenType::If => {
            let loc = lexer.get_token_loc();
            vec![Stmt {
                stype: StmtType::If(if_stmt(lexer, master)),
                loc,
            }]
        }
        TokenType::While => {
            let loc = lexer.get_token_loc();
            vec![Stmt {
                stype: StmtType::While(while_stmt(lexer, master)),
                loc,
            }]
        }
        TokenType::For => {
            let loc = lexer.get_token_loc();
            vec![Stmt {
                stype: StmtType::ForLoop(for_loop(lexer, master)),
                loc,
            }]
        }
        TokenType::Return => {
            let loc = lexer.get_token_loc();
            lexer.match_token(TokenType::Return);
            let stmt = vec![Stmt {
                stype: StmtType::Return(expr(lexer)),
                loc,
            }];
            lexer.match_token(TokenType::SemiColon);
            stmt
        }
        TokenType::Identifier => {
            //Assgin Op
            vec![assign(lexer)]
        }
        TokenType::Asm => {
            let loc = lexer.get_token_loc();
            lexer.match_token(TokenType::Asm);
            lexer.match_token(TokenType::OCurly);
            let mut instructs = Vec::<String>::new();
            while lexer.get_token_type() == TokenType::String {
                instructs.push(lexer.get_token().literal);
                lexer.match_token(TokenType::String);
            }
            lexer.match_token(TokenType::CCurly);
            vec![Stmt {
                stype: StmtType::InlineAsm(instructs),
                loc,
            }]
        }
        _ => {
            todo!();
        }
    }
}

/// Parse Blocks
/// # Argumenrs
/// * lexer - address of mutable lexer
/// Returns a vec of stmts
pub fn parse_block(lexer: &mut Lexer, master: &String) -> Vec<Stmt> {
    lexer.match_token(TokenType::OCurly);
    let mut stmts = Vec::<Stmt>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        stmts.append(&mut parse_stmt(lexer, master));
    }
    lexer.match_token(TokenType::CCurly);
    stmts
}
