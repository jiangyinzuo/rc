use crate::ast::expr::*;
use crate::ast::TokenStart;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

impl Parse for Expr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        if !Self::is_token_start(cursor.next_token()?) {
            return Err("expected start token of Expr".into());
        }
        prec::parse(cursor)
    }
}

/// Expression having precedences
pub mod prec {
    use crate::ast::expr::Expr::{ArrayIndex, Assign, Call, FieldAccess, Range, Unary};
    use crate::ast::expr::UnOp::{Borrow, BorrowMut};
    use crate::ast::expr::{
        ArrayIndexExpr, AssignExpr, BinOpExpr, BinOperator, CallExpr, CallParams, Expr,
        FieldAccessExpr, FromToken, Precedence, RangeExpr, UnAryExpr, UnOp,
    };
    use crate::ast::TokenStart;
    use crate::lexer::token::Token;
    use crate::parser::expr::primitive::primitive_expr;
    use crate::parser::{Parse, ParseCursor};
    use crate::rcc::RccError;

    pub fn parse(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        self::assign_expr(cursor)
    }

    /// AssignExpr -> RangeExpr
    ///             | RangeExpr AssignOp AssignExpr
    /// (Associativity: right to left)
    fn assign_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        let mut lhs = range_expr(cursor)?;

        if let Some(assign_op) = cursor.eat_token_if_from() {
            let rhs = assign_expr(cursor)?;
            lhs = Assign(AssignExpr::new(lhs, assign_op, rhs));
        }
        Ok(lhs)
    }

    /// RangeExpr -> BinOpExpr
    ///            | BinOpExpr? RangeOp BinOpExpr?
    /// (Associativity: require parentheses)
    pub(super) fn range_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        let mut lhs_err = "".into();
        let lhs = if RangeExpr::is_token_start(cursor.next_token()?) {
            None
        } else {
            match bin_op_expr(cursor) {
                Ok(expr) => Some(expr),
                Err(e) => {
                    lhs_err = e;
                    None
                }
            }
        };

        let expr = if let Some(op) = cursor.eat_token_if_from() {
            let mut range_expr = RangeExpr::new(op);
            if let Some(expr) = lhs {
                range_expr.set_lhs(expr);
            }
            if let Ok(rhs) = bin_op_expr(cursor) {
                range_expr.set_rhs(rhs);
            }
            Range(range_expr)
        } else {
            match lhs {
                // RangeExpr -> AssignExpr
                Some(expr) => expr,
                None => return Err(lhs_err),
            }
        };

        if let Ok(tk) = cursor.next_token() {
            if tk.is_range_op() {
                return Err("range operators require parentheses".into());
            }
        }
        Ok(expr)
    }

    /// Operator Precedence Parsing
    /// as               left to right
    /// * / %            left to right
    /// + -              left to right
    /// << >>            left to right
    /// &                left to right
    /// ^                left to right
    /// |                left to right
    /// == != < > <= >=  require parentheses
    /// &&               left to right
    /// ||               left to right
    fn bin_op_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        fn reduce(bin_ops: &mut Vec<BinOperator>, exprs: &mut Vec<Expr>) {
            while let Some(last_op) = bin_ops.pop() {
                let rhs = exprs.pop().unwrap();
                let lhs = exprs.pop().unwrap();
                exprs.push(Expr::BinOp(BinOpExpr::new(lhs, last_op, rhs)));
            }
            debug_assert_eq!(exprs.len(), 1);
            debug_assert!(bin_ops.is_empty());
        };

        let mut exprs = vec![unary_expr(cursor)?];
        let mut bin_ops: Vec<BinOperator> = vec![];
        let mut next_is_op = true;

        loop {
            if next_is_op {
                if let Some(next_op) = cursor.eat_token_if_from::<BinOperator>() {
                    if let Some(last_op) = bin_ops.last() {
                        // 1 + 2 * 3   <- -
                        let prec_last = Precedence::from_bin_op(last_op);
                        let prec_next = Precedence::from_bin_op(&next_op);
                        if prec_last <= prec_next {
                            if prec_last == Precedence::Cmp && prec_last == prec_next {
                                return Err(
                                    "Chained comparison operator require parentheses".into()
                                );
                            }
                            reduce(&mut bin_ops, &mut exprs);
                        }
                    }
                    bin_ops.push(next_op);
                } else {
                    reduce(&mut bin_ops, &mut exprs);
                    return Ok(exprs.pop().unwrap());
                }
                next_is_op = false;
            } else {
                exprs.push(unary_expr(cursor)?);
                next_is_op = true;
            }
        }
    }

    /// UnAryExpr -> CallExpr
    ///            | ( `!` | `*` | `-` | `&` | `& mut` ) UnAryExpr
    fn unary_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        Ok(
            if let Some(tk) = cursor.eat_token_if_in(&[
                Token::Not,
                Token::Star,
                Token::Minus,
                Token::And,
                Token::AndAnd,
            ]) {
                if tk == &Token::AndAnd {
                    let op = if cursor.eat_token_if_eq(Token::Mut) {
                        BorrowMut
                    } else {
                        Borrow
                    };
                    let expr = Unary(UnAryExpr::new(op, unary_expr(cursor)?));
                    Unary(UnAryExpr::new(Borrow, expr))
                } else {
                    let mut op = UnOp::from_token(tk.clone()).unwrap();
                    if op == UnOp::Borrow && cursor.eat_token_if_eq(Token::Mut) {
                        op = UnOp::BorrowMut;
                    }
                    Unary(UnAryExpr::new(op, unary_expr(cursor)?))
                }
            } else {
                call_expr(cursor)?
            },
        )
    }

    /// CallExpr -> PrimitiveExpr
    ///           | CallExpr `(` CallParamsExpr `)`
    ///           | CallExpr ArrayIndexExpr
    ///           | CallExpr `.` PrimitiveExpr
    fn call_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        let mut expr = primitive_expr(cursor)?;
        while let Ok(tk) = cursor.next_token() {
            expr = match tk {
                Token::LeftParen => {
                    cursor.bump_token()?;
                    let call_params = CallParams::parse(cursor)?;
                    cursor.eat_token_eq(Token::RightParen)?;
                    Call(CallExpr::new(expr).call_params(call_params))
                }
                Token::LeftSquareBrackets => {
                    let index_expr = ArrayIndexExpr::parse_index(cursor)?;
                    ArrayIndex(ArrayIndexExpr::new(expr, index_expr))
                }
                Token::Dot => {
                    cursor.bump_token()?;
                    let rhs = primitive_expr(cursor)?;
                    FieldAccess(FieldAccessExpr::new(expr, rhs))
                }
                _ => return Ok(expr),
            }
        }
        Ok(expr)
    }

    /// CallParams ->
    ///    Expr ( , Expr )* ,?
    impl Parse for CallParams {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            let mut call_params = vec![];
            let expr = Expr::parse(cursor)?;
            call_params.push(expr);
            while cursor.eat_token_if_eq(Token::Comma) {
                call_params.push(Expr::parse(cursor)?);
            }
            Ok(call_params)
        }
    }

    /// ArrayIndexExpr -> `[` Expr `]`
    impl ArrayIndexExpr {
        fn parse_index(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
            cursor.eat_token_eq(Token::LeftSquareBrackets)?;
            let expr = Expr::parse(cursor)?;
            cursor.eat_token_eq(Token::RightSquareBrackets)?;
            Ok(expr)
        }
    }
}

/// Primitive Expressions
pub mod primitive {
    use crate::ast::expr::Expr::{Array, Block, If, Lit, LitBool, Path};
    use crate::ast::expr::*;
    use crate::ast::stmt::Stmt;
    use crate::lexer::token::LiteralKind::*;
    use crate::lexer::token::Token;
    use crate::parser::expr::prec::range_expr;
    use crate::parser::{Parse, ParseCursor};
    use crate::rcc::RccError;

    /// PrimitiveExpr -> PathExpr | LitExpr | BlockExpr
    ///                | GroupedExpr | TupleExpr | ArrayExpr
    ///                | ReturnExpr | BreakExpr
    ///                | RangeExpr(without lhs)
    pub fn primitive_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        let expr = match cursor.next_token()? {
            Token::Identifier(_) | Token::PathSep => Path(PathExpr::parse(cursor)?),
            Token::Literal { .. } => Lit(LitExpr::parse(cursor)?),
            Token::True | Token::False => LitBool(*cursor.bump_token()? == Token::True),
            Token::LeftCurlyBraces => Block(BlockExpr::parse(cursor)?),
            Token::LeftParen => parse_grouped_or_tuple_expr(cursor)?,
            Token::LeftSquareBrackets => Array(ArrayExpr::parse(cursor)?),
            Token::If => If(IfExpr::parse(cursor)?),
            Token::Return => Expr::Return(ReturnExpr::parse(cursor)?),
            Token::Break => Expr::Break(BreakExpr::parse(cursor)?),
            Token::DotDot | Token::DotDotEq => range_expr(cursor)?,
            _ => unreachable!(),
        };
        Ok(expr)
    }

    /// GroupedExpr | TupleExpr
    fn parse_grouped_or_tuple_expr(cursor: &mut ParseCursor) -> Result<Expr, RccError> {
        cursor.eat_token_eq(Token::LeftParen)?;
        let expr = Expr::parse(cursor)?;
        match cursor.next_token()? {
            Token::RightParen => {
                cursor.bump_token()?;
                Ok(Expr::Grouped(GroupedExpr::new(expr)))
            }
            Token::Comma => Ok(Expr::Tuple(TupleExpr::parse_from_second(cursor, expr)?)),
            _ => Err("expected ',' or ')'".into()),
        }
    }

    ///  TupleExpr -> `(` ( Expr , )+ Expr? `)`
    impl TupleExpr {
        fn parse_from_second(cursor: &mut ParseCursor, first_expr: Expr) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::Comma)?;
            let mut tuple_expr = TupleExpr(vec![first_expr]);
            while cursor.next_token()? != &Token::RightParen {
                tuple_expr.0.push(Expr::parse(cursor)?);
                if !cursor.eat_token_if_eq(Token::Comma) {
                    break;
                }
            }
            if cursor.eat_token_if_eq(Token::RightParen) {
                Ok(tuple_expr)
            } else {
                Err("invalid tuple expression".into())
            }
        }
    }

    /// PathExpr -> identifier (:: identifier)*
    /// # Examples
    /// `a::b::c`, `a`
    impl Parse for PathExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            #[derive(PartialEq)]
            enum State {
                Init,
                PathSep,
                Segment,
            }

            let mut path_expr = Self::new();
            let mut state = State::Init;
            while let Ok(tk) = cursor.next_token() {
                match tk {
                    Token::PathSep => {
                        if state == State::PathSep || state == State::Init {
                            return Err("invalid path".into());
                        }
                        state = State::PathSep;
                    }
                    Token::Identifier(s) => {
                        if state == State::Segment {
                            return Err("invalid path".into());
                        }
                        state = State::Segment;
                        path_expr.segments.push(s.to_string());
                    }
                    _ => break,
                }
                cursor.bump_token()?;
            }
            if state == State::Segment {
                Ok(path_expr)
            } else {
                Err("invalid path".into())
            }
        }
    }

    /// LitExpr -> literal
    impl Parse for LitExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            let (literal_kind, value) = cursor.eat_literal()?;
            Ok(LitExpr {
                ret_type: {
                    match literal_kind {
                        Char => "char",
                        String => "&str",
                        Integer { suffix } => {
                            if suffix.is_empty() {
                                Self::EMPTY_INT_TYPE
                            } else {
                                suffix
                            }
                        }
                        Float { suffix } => {
                            if suffix.is_empty() {
                                Self::EMPTY_FLOAT_TYPE
                            } else {
                                suffix
                            }
                        }
                    }
                    .into()
                },
                value,
            })
        }
    }

    /// BlockExpr -> `{` Expr* `}`
    impl Parse for BlockExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::LeftCurlyBraces)?;
            let mut block_expr = BlockExpr::new();
            while cursor.next_token()? != &Token::RightCurlyBraces {
                block_expr.stmts.push(Stmt::parse(cursor)?);
            }
            cursor.eat_token_eq(Token::RightCurlyBraces)?;
            Ok(block_expr)
        }
    }

    /// ArrayExpr -> `[` Expression ( , Expression )* ,? `]`
    ///            | `[` Expression ; Expression `]`
    impl Parse for ArrayExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::LeftSquareBrackets)?;
            let mut elems = vec![Expr::parse(cursor)?];
            let mut last_is_comma = false;
            loop {
                match cursor.next_token()? {
                    Token::RightCurlyBraces => return Ok(ArrayExpr::elems(elems)),
                    Token::Semi => {
                        cursor.bump_token()?;
                        return if elems.len() == 1 {
                            let len = Expr::parse(cursor)?;
                            let len = ConstantExpr::<usize>::expr(len);
                            cursor.eat_token_eq(Token::RightCurlyBraces)?;
                            Ok(ArrayExpr::new(elems, len))
                        } else {
                            Err("length of elems should be 1".into())
                        };
                    }
                    Token::Comma => {
                        if last_is_comma {
                            return Err("expected expr, found `,`".into());
                        }
                        last_is_comma = true;
                        cursor.bump_token()?;
                    }
                    _ => {
                        if !last_is_comma {
                            return Err("expected `,`".into());
                        }
                        last_is_comma = false;
                        elems.push(Expr::parse(cursor)?);
                    }
                }
            }
        }
    }

    /// IfExpr -> `if` Expr BlockExpr ( `else` (BlockExpr | IfExpr) )?
    impl Parse for IfExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::If)?;
            let mut if_expr = IfExpr::new();
            if_expr.add_cond(Expr::parse(cursor)?);
            if_expr.add_block(BlockExpr::parse(cursor)?);
            while cursor.eat_token_if_eq(Token::Else) {
                if cursor.eat_token_if_eq(Token::If) {
                    if_expr.add_cond(Expr::parse(cursor)?);
                }
                if_expr.add_block(BlockExpr::parse(cursor)?);
            }
            Ok(if_expr)
        }
    }

    /// ReturnExpr -> `return` Expr
    impl Parse for ReturnExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::Return)?;
            let expr = Expr::parse(cursor)?;
            Ok(ReturnExpr(Box::new(expr)))
        }
    }

    /// BreakExpr -> `break` Expr?
    impl Parse for BreakExpr {
        fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
            cursor.eat_token_eq(Token::Break)?;
            Ok(if let Ok(expr) = Expr::parse(cursor) {
                BreakExpr(Some(Box::new(expr)))
            } else {
                BreakExpr(None)
            })
        }
    }
}