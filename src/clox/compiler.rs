use core::panic;
use std::{collections::HashMap, mem, ops::Add};

use tracing::trace;

use crate::clox::token::{Token, TokenType};

use super::{
    chunk::Chunk, clox_error::CloxError, clox_value::CloxValue, opcode::Opcode, scanner::Scanner,
    string_indexer::StringId,
};

pub(super) struct Compiler<'src> {
    scanner: Scanner<'src>,
    parser: Parser,
    chunk: Chunk,
    parse_rules: HashMap<TokenType, ParseRule<'src>>,

    has_error: bool,
    panic_mode: bool,
}

#[derive(Debug)]
enum MessageType<'msg> {
    StringIndex(StringId),
    Message(&'msg str),
}

struct Parser {
    current: Option<Token>,
    previous: Option<Token>,
}

impl<'src> Compiler<'src> {
    pub(super) fn new() -> Self {
        let parse_rules = create_rules();
        Self {
            scanner: Scanner::new(""),
            parser: Parser {
                current: None,
                previous: None,
            },
            chunk: Chunk::new(),
            parse_rules,
            has_error: false,
            panic_mode: false,
        }
    }

    pub(crate) fn compile(&mut self, source: &'src str) -> Result<Chunk, CloxError> {
        self.scanner = Scanner::new(source);

        self.has_error = false;
        self.panic_mode = false;

        self.advance();
        self.expression();
        self.consume(
            TokenType::Eof,
            MessageType::Message("Expected end of expression"),
        );

        self.end_compiler();

        if self.has_error {
            Err(CloxError::CompileError)
        } else {
            Ok(mem::replace(&mut self.chunk, Chunk::new()))
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            match self.scanner.scan_token() {
                Ok(token) => {
                    self.parser.current = Some(token);
                    break;
                }
                Err(_) => {
                    let parser = &self.parser;
                    match parser.current {
                        Some(token) => {
                            self.error_at_current(MessageType::StringIndex(token.id));
                        }
                        None => unreachable!("advance"),
                    }
                }
            }
        }
    }

    fn error_at_current(&mut self, message: MessageType) {
        self.error_at(
            self.parser.current.expect("should have a token here"),
            message,
        )
    }

    fn expression(&mut self) {
        self.parse_presendence(Precedence::Assignment);
    }

    fn consume(&mut self, token_type: TokenType, message: MessageType) {
        if self.parser.current.expect("").token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at(&mut self, current: Token, message: MessageType) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        if current.token_type == TokenType::Eof {
            let msg = match message {
                MessageType::StringIndex(string_id) => self.scanner.get_str_at(string_id),
                MessageType::Message(message) => message,
            };
            trace!("[line: {}] Error at end: {}", current.line, msg);
        }

        self.has_error = true;
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        if !self.has_error {
            self.chunk.disassemble_chunk("test");
        } else {
            println!("error compiling");
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(Opcode::Return);
    }

    fn emit_bytes(&mut self, op1: Opcode, op2: Opcode) {
        self.emit_byte(op1);
        self.emit_byte(op2);
    }

    fn emit_byte(&mut self, opcode: Opcode) {
        let line = self.parser.previous.expect("").line;
        trace!("writing: {:?}", opcode);
        self.chunk.write_chunk(opcode, line)
    }

    fn number(&mut self) {
        let string_id = self.parser.previous.unwrap().id;
        let value = self.scanner.get_str_at(string_id);
        trace!("parsed number: {}", value);
        let val = CloxValue::Number(value.parse().expect("should expect a float"));
        self.emit_constant(val)
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            MessageType::Message("Expect ')' after expression"),
        );
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.expect("").token_type;
        // self.expression();
        self.parse_presendence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(Opcode::Not),
            TokenType::Minus => self.emit_byte(Opcode::Negate),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator = self.parser.previous.expect("").token_type;
        trace!("binary op: {:?}", operator);

        let precedence = self.get_rule(&operator).precedence + 1.into();
        trace!(
            "Precedence of operator rule {:?}: {:?}",
            operator,
            precedence
        );
        self.parse_presendence(precedence);

        match operator {
            TokenType::BangEqual => self.emit_bytes(Opcode::Equal, Opcode::Not),
            TokenType::EqualEqual => self.emit_byte(Opcode::Equal),
            TokenType::Greater => self.emit_byte(Opcode::Greater),
            TokenType::GreaterEqual => self.emit_bytes(Opcode::Less, Opcode::Not),
            TokenType::Less => self.emit_byte(Opcode::Less),
            TokenType::LessEqual => self.emit_bytes(Opcode::Greater, Opcode::Not),

            TokenType::Plus => self.emit_byte(Opcode::Add),
            TokenType::Minus => self.emit_byte(Opcode::Sub),
            TokenType::Star => self.emit_byte(Opcode::Mul),
            TokenType::Slash => self.emit_byte(Opcode::Div),
            _ => unreachable!(),
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.expect("").token_type {
            TokenType::False => self.emit_byte(Opcode::False),
            TokenType::Nil => self.emit_byte(Opcode::Nil),
            TokenType::True => self.emit_byte(Opcode::True),
            _ => unreachable!(),
        }
    }

    fn string(&mut self) {}

    fn emit_constant(&mut self, value: CloxValue) {
        self.emit_byte(Opcode::Constant(value));
    }

    fn parse_presendence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(prefix_rule) = self
            .get_rule(&self.parser.previous.expect("").token_type)
            .prefix
        {
            trace!(
                "Found prefix rule for {:?}: {:?}",
                self.parser.previous.expect("").token_type,
                prefix_rule
            );
            prefix_rule(self);

            while precedence
                <= self
                    .get_rule(&self.parser.current.expect("").token_type)
                    .precedence
            {
                self.advance();
                let infix_rule = self
                    .get_rule(&self.parser.previous.expect("").token_type)
                    .infix
                    .expect("No infeix rule defined");

                trace!(
                    "Found infix rule for {:?}: {:?}",
                    self.parser.previous.expect("").token_type,
                    infix_rule
                );
                infix_rule(self);
            }
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> &ParseRule<'src> {
        trace!("looking for rule for {token_type:?}");
        &self.parse_rules[token_type]
    }
}

macro_rules! add_parse_rule {
    ($m:ident, $token_type:expr => $prefix:expr, $infix:expr, $precedence:expr) => {{
        $m.insert($token_type, ParseRule::new($prefix, $infix, $precedence));
    }};
}

fn create_rules<'src>() -> HashMap<TokenType, ParseRule<'src>> {
    let mut parse_rules: HashMap<TokenType, ParseRule> = HashMap::new();

    add_parse_rule!(parse_rules, TokenType::LeftParen   => Some(Compiler::grouping),    None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::RightParen  => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::LeftBrace   => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::RightBrace  => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Comma       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Dot         => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Minus       => Some(Compiler::unary),       Some(Compiler::binary), Precedence::Term);
    add_parse_rule!(parse_rules, TokenType::Plus        => None,                        Some(Compiler::binary), Precedence::Term);
    add_parse_rule!(parse_rules, TokenType::SemiColon   => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Slash       => None,                        Some(Compiler::binary), Precedence::Factor);
    add_parse_rule!(parse_rules, TokenType::Star        => None,                        Some(Compiler::binary), Precedence::Factor);
    add_parse_rule!(parse_rules, TokenType::Bang        => Some(Compiler::unary),       None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::BangEqual   => None,                        Some(Compiler::binary), Precedence::Equality);
    add_parse_rule!(parse_rules, TokenType::Equal       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::EqualEqual  => None,                        Some(Compiler::binary), Precedence::Equality);
    add_parse_rule!(parse_rules, TokenType::Greater     => None,                        Some(Compiler::binary), Precedence::Comparison);
    add_parse_rule!(parse_rules, TokenType::GreaterEqual=> None,                        Some(Compiler::binary), Precedence::Comparison);
    add_parse_rule!(parse_rules, TokenType::Less        => None,                        Some(Compiler::binary), Precedence::Comparison);
    add_parse_rule!(parse_rules, TokenType::LessEqual   => None,                        Some(Compiler::binary), Precedence::Comparison);
    add_parse_rule!(parse_rules, TokenType::Identifier  => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::String      => Some(Compiler::string),      None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Number      => Some(Compiler::number),      None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::And         => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Class       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Else        => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::False       => Some(Compiler::literal),     None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::For         => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Fun         => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::If          => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Nil         => Some(Compiler::literal),     None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Or          => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Print       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Return      => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Super       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::This        => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::True        => Some(Compiler::literal),     None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Var         => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::While       => None,                        None,                   Precedence::None);
    add_parse_rule!(parse_rules, TokenType::Eof         => None,                        None,                   Precedence::None);
    parse_rules
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<Precedence> for u32 {
    fn from(val: Precedence) -> Self {
        match val {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 2,
            Precedence::And => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
        }
    }
}

impl From<u32> for Precedence {
    fn from(val: u32) -> Self {
        match val {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => panic!("Invalid presedence"),
        }
    }
}

impl Add<Precedence> for Precedence {
    type Output = Precedence;

    fn add(self, rhs: Precedence) -> Self::Output {
        let lhs: u32 = self.into();
        let rhs: u32 = rhs.into();
        (lhs + rhs).into()
    }
}

type ParseFn<'a> = fn(&mut Compiler<'a>) -> ();

struct ParseRule<'a> {
    prefix: Option<ParseFn<'a>>,
    infix: Option<ParseFn<'a>>,
    pub(super) precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn new(
        prefix: Option<ParseFn<'a>>,
        infix: Option<ParseFn<'a>>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}
