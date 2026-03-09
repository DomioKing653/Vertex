use crate::backend::{
    ast::{
        nodes::{
            BinaryOpNode, BoolNode,
            CallType::{Fn, Macro},
            FloatNode, FunctionCallNode, ImportNode, NumberNode, ProgramNode, StringNode,
            VariableAccessNode, VariableAssignNode, VariableDefineNode,
        },
        statements::{
            functions::{args_node::FunctionArgs, function_nodes::FunctionDefineNode},
            if_statement::IfStatement,
            while_statement::WhileStatement,
        },
    },
    compiler::byte_code::Compilable,
    errors::parser_errors::ParserError::{self, UnexpectedToken},
    lexer::tokens::{
        Token,
        TokenKind::{
            self, CLOSINGBRACE, COLON, COMMA, CONST, DIVIDE, ELSE, EOF, EQUAL, FALSE, FLOAT, FNC,
            GREATER, IDENTIFIER, IF, LEFTPAREN, LESS, MINUS, MODULO, NUMB, OPENINGBRACE, PLUS,
            RIGHTPAREN, SEMICOLON, STRING, TIMES, TRUE, USE, VALUE, VAR, WHILE,
        },
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    token_idx: usize,
    on_top_statement: bool,
}
impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            tokens: token_list,
            token_idx: 0,
            on_top_statement: true,
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.token_idx]
    }

    pub fn advance(&mut self) {
        self.token_idx += 1;
    }


    pub fn parse(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut program: ProgramNode = ProgramNode::new();

        while self.on_top_statement && self.current_token().token_kind != EOF {
            program.program_nodes.push(self.parse_top_statement()?);
        }

        while self.current_token().token_kind != EOF {
            program.program_nodes.push(self.parse_stmt()?)
        }
        Ok(Box::new(program))
    }
    fn parse_top_statement(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        match self.current_token().token_kind {
            USE => {
                self.advance();
                let name_to_use = self.expect(STRING)?.token_value;
                self.expect(SEMICOLON)?;
                return Ok(Box::new(ImportNode {
                    module: name_to_use,
                }));

            }
            _ => {
                self.on_top_statement = false;
                self.parse_stmt()
            }
        }
    }

    //WARN:Do not use import in other statements
    // just on top of the program for now
    fn parse_stmt(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        match &self.current_token().token_kind {
            TokenKind::EXP =>{
                self.advance();
                if self.current_token().token_kind == CONST || self.current_token().token_kind == VAR {
                    let value = self.parse_var_decl_stmt(true)?;
                    self.expect(SEMICOLON)?;
                    Ok(value)
                }
                else if self.current_token().token_kind == FNC{
                    let mut args = Vec::new();
                    self.advance(); //FN
                    let id = self.expect(IDENTIFIER)?;
                    self.expect(LEFTPAREN)?;
                    if self.current_token().token_kind != RIGHTPAREN {
                        loop {
                            let arg_name = self.expect(IDENTIFIER)?;
                            self.expect(COLON)?;
                            let arg_type = self.expect(IDENTIFIER)?;

                            args.push(FunctionArgs {
                                name: arg_name.token_value,
                                argument_type: arg_type.token_value,
                            });

                            if self.current_token().token_kind == COMMA {
                                self.advance();
                                continue;
                            }

                            break;
                        }
                    }
                    self.expect(RIGHTPAREN)?;
                    let return_type = if self.current_token().token_kind == COLON {
                        self.advance();
                        Some(self.expect(IDENTIFIER)?.token_value)
                    } else {
                        None
                    };
                    self.expect(OPENINGBRACE)?;

                    let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                    while self.current_token().token_kind != CLOSINGBRACE {
                        body.push(self.parse_stmt()?);
                    }
                    self.expect(CLOSINGBRACE)?;
                    println!("{:?}", &return_type);

                    Ok(Box::new(FunctionDefineNode {
                        id: id.token_value,
                        return_type,
                        body,
                        args,
                    }))

                }else {
                    Err(UnexpectedToken {
                        expected:VAR,
                        found:self.current_token().token_value.clone()
                    })
                }
            }
            VAR | CONST => {
                let value = self.parse_var_decl_stmt(false);
                self.expect(SEMICOLON)?;
                value
            }
            IDENTIFIER if self.peek() == EQUAL => {
                let id = self.current_token().token_value.clone();
                self.advance();
                self.expect(EQUAL)?;
                let value = self.parse_expr()?;
                self.expect(SEMICOLON)?;
                Ok(Box::new(VariableAssignNode { name: id, value }))
            }
            IF => {
                self.advance();
                self.expect(LEFTPAREN)?;
                let condition = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                            expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                if self.current_token().token_kind == ELSE {
                    self.advance();
                    self.expect(OPENINGBRACE)?;
                    let mut else_body: Vec<Box<dyn Compilable>> = Vec::new();
                    while self.current_token().token_kind != CLOSINGBRACE {
                        if self.current_token().token_kind == EOF {
                            return Err(ParserError::UnexpectedToken {
                                found: "EOF".into(),
                                expected: SEMICOLON,
                            });
                        }
                        else_body.push(self.parse_stmt()?);
                    }
                    self.expect(CLOSINGBRACE)?;
                    return Ok(Box::new(IfStatement {
                        condition,
                        then_branch: body,
                        else_branch: Some(else_body),
                    }));
                }
                return Ok(Box::new(IfStatement {
                    condition,
                    then_branch: body,
                    else_branch: None,
                }));
            }
            WHILE => {
                self.advance();
                self.expect(LEFTPAREN)?;
                let condition = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                            expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                return Ok(Box::new(WhileStatement { condition, body }));
            }
            FNC => {
                let mut args = Vec::new();
                self.advance(); //FN
                let id = self.expect(IDENTIFIER)?;
                self.expect(LEFTPAREN)?;
                if self.current_token().token_kind != RIGHTPAREN {
                    loop {
                        let arg_name = self.expect(IDENTIFIER)?;
                        self.expect(COLON)?;
                        let arg_type = self.expect(IDENTIFIER)?;

                        args.push(FunctionArgs {
                            name: arg_name.token_value,
                            argument_type: arg_type.token_value,
                        });

                        if self.current_token().token_kind == COMMA {
                            self.advance();
                            continue;
                        }

                        break;
                    }
                }
                self.expect(RIGHTPAREN)?;
                let reurn_type = if self.current_token().token_kind == COLON {
                    self.advance();
                    Some(self.expect(IDENTIFIER)?.token_value)
                } else {
                    None
                };
                self.expect(OPENINGBRACE)?;

                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                println!("{:?}", &reurn_type);

                Ok(Box::new(FunctionDefineNode {
                    id: id.token_value,
                    return_type: reurn_type,
                    body,
                    args,
                }))
            }
            _ => {
                let expr = self.parse_expr();
                self.expect(SEMICOLON)?;
                expr
            }
        }
    }

    fn parse_var_decl_stmt(&mut self,is_pub:bool) -> Result<Box<dyn Compilable>, ParserError> {
        let is_const: bool;
        if self.current_token().token_kind == CONST {
            is_const = true;
        } else {
            is_const = false;
        }
        let id: String;
        self.advance();
        id = self.expect(IDENTIFIER)?.token_value;
        let mut value_type = None;

        if self.current_token().token_kind == COLON {
            self.advance();

            value_type = Some(self.expect(IDENTIFIER)?.token_value);
        }
        let value: Option<Box<dyn Compilable>>;
        if self.current_token().token_kind == EQUAL {
            self.advance();
            value = Some(self.parse_expr()?);
        } else {
            value = None
        }
        Ok(Box::new(VariableDefineNode {
            value_type,
            value,
            var_name: id,
            is_const,
            is_public:is_pub,
        }))
    }

    fn parse_expr(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        //let mut comp:Box<dyn Compilable>;
        //if self.current_token()==MINUS {
        //  comp
        //}

        let comp = self.parse_comparison();
        comp
    }

    fn parse_comparison(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_term()?;
        while self.current_token().token_kind == GREATER || self.current_token().token_kind == LESS
        {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode {
                left: factor,
                right: self.parse_term()?,
                op_tok: operator,
            });
        }
        Ok(factor)
    }
    fn parse_term(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_factor()?;
        while self.current_token().token_kind == MINUS || self.current_token().token_kind == PLUS {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode {
                left: factor,
                right: self.parse_factor()?,
                op_tok: operator,
            });
        }
        Ok(factor)
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_unary()?;
        while self.current_token().token_kind == TIMES
            || self.current_token().token_kind == DIVIDE
            || self.current_token().token_kind == MODULO
        {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode {
                left: factor,
                right: self.parse_unary()?,
                op_tok: operator,
            });
        }
        Ok(factor)
    }

    fn parse_unary(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        if self.current_token().token_kind == FLOAT {
            let value = match self.current_token().token_value.parse::<f32>() {
                Err(_) => unreachable!(),
                Ok(numb) => numb,
            };
            self.advance();
            Ok(Box::new(FloatNode { number: value }))
        } else if self.current_token().token_kind == TRUE
            || self.current_token().token_kind == FALSE
        {
            let value = self.current_token().token_kind.clone();
            self.advance();
            Ok(Box::new(BoolNode { value }))
        } else if self.current_token().token_kind == NUMB {
            let value = match self.current_token().token_value.parse::<i64>() {
                Ok(numb) => numb,
                Err(_) => unreachable!(),
            };
            self.advance();
            Ok(Box::new(NumberNode { number: value }))
        } else if self.current_token().token_kind == IDENTIFIER {
            let value = self.current_token().token_value.clone();
            self.advance();

            if self.current_token().token_kind == LEFTPAREN {
                self.advance();
                let mut args: Vec<Box<dyn Compilable>> = Vec::new();

                if self.current_token().token_kind != RIGHTPAREN {
                    loop {
                        args.push(self.parse_expr()?);

                        if self.current_token().token_kind == COMMA {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(RIGHTPAREN)?;
                let is_macro = value.ends_with('!');
                let name = value.trim_end_matches('!').to_string();

                Ok(Box::new(FunctionCallNode {
                    args,
                    name,
                    call_type: if is_macro { Macro } else { Fn },
                    return_type:None
                }))
            } else {
                Ok(Box::new(VariableAccessNode {
                    variable_name: value,
                }))
            }
        } else if self.current_token().token_kind == LEFTPAREN {
            self.advance();
            let value = self.parse_expr()?;
            self.expect(RIGHTPAREN)?;
            Ok(value)
        } else if self.current_token().token_kind == STRING {
            let value = StringNode {
                value: self.current_token().token_value.clone(),
            };
            self.advance();
            Ok(Box::new(value))
        } else {
            Err(UnexpectedToken {
                found: self.current_token().token_value.clone(),
                expected: VALUE,
            })
        }
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token, ParserError> {
        if self.current_token().token_kind == token_kind {
            let token = self.current_token().clone();
            self.advance();
            Ok(token)
        } else {
            Err(UnexpectedToken {
                expected: token_kind,
                found: self.current_token().token_value.clone(),
            })
        }
    }

    fn peek(&self) -> TokenKind {
        let idx = self.token_idx + 1;
        self.tokens[idx].token_kind.clone()
    }
}
