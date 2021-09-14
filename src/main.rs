/* Copyright (C) Logicarth (OPC) Private Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;

const UNEXPECTED_ERROR: &str = "Unexpected Error";

// Making invalid expressions syntactically invalid, probably by wrapping expressions of different result types into their own types
// 0. Prefer match over if, for consistency
// 1. Do Internationalization
// 2. Implement concat, regex, identity
// 3. Implement symbols
// 4. Implement dot operator
// 5. Write test cases and build some audio visual documentation
#[derive(Debug)]
enum CustomError {
    Message(String),
    Messages(HashMap<String, CustomError>),
}

// Arithmetic Ops

#[derive(Debug)]
enum ArithmeticResultType {
    Number,
    Decimal,
    Text,
}

#[derive(Debug)]
enum ArithmeticResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

#[derive(Debug)]
enum ArithmeticOperator {
    Add,
    Multiply,
    Subtract,
    Divide,
    Modulus,
}

// NUMBER ARITHMETIC

#[derive(Debug)]
enum NumberArithmeticArg {
    // Note. Decimal is not allowed as an arg to avoid loss of precision
    Number(i32),
    NumberArithmeticExpression(NumberArithmeticExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum NumberArithmeticExpression {
    Add(Box<(NumberArithmeticArg, Vec<NumberArithmeticArg>)>),
    Multiply(Box<(NumberArithmeticArg, Vec<NumberArithmeticArg>)>),
    Subtract(Box<(NumberArithmeticArg, Vec<NumberArithmeticArg>)>),
    Divide(Box<(NumberArithmeticArg, Vec<NumberArithmeticArg>)>),
    Modulus(Box<(NumberArithmeticArg, Vec<NumberArithmeticArg>)>),
}

impl NumberArithmeticExpression {
    fn unwrap_number(&self) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_decimal(&self) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ArithmeticResultType) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            NumberArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Add),
        };
        let init: Result<i32, CustomError> = match &args.0 {
            NumberArithmeticArg::Number(v) => Ok(*v),
            NumberArithmeticArg::NumberArithmeticExpression(v1) => v1.unwrap_number(),
        };
        let result: Result<i32, CustomError> = args.1.iter().fold(init, |acc, val| match &acc {
            Ok(v) => match val {
                NumberArithmeticArg::Number(v1) => match operator {
                    ArithmeticOperator::Add => Ok(v + *v1),
                    ArithmeticOperator::Multiply => Ok(v * *v1),
                    ArithmeticOperator::Subtract => Ok(v - *v1),
                    ArithmeticOperator::Divide => Ok(v / *v1),
                    ArithmeticOperator::Modulus => Ok(v % *v1),
                },
                NumberArithmeticArg::NumberArithmeticExpression(v1) => match v1.unwrap_number() {
                    Ok(v2) => match operator {
                        ArithmeticOperator::Add => Ok(v + v2),
                        ArithmeticOperator::Multiply => Ok(v * v2),
                        ArithmeticOperator::Subtract => Ok(v - v2),
                        ArithmeticOperator::Divide => Ok(v / v2),
                        ArithmeticOperator::Modulus => Ok(v % v2),
                    },
                    Err(e) => Err(e),
                },
            },
            Err(_) => acc,
        });
        match result {
            Ok(v) => match result_type {
                ArithmeticResultType::Number => Ok(ArithmeticResult::Number(v)),
                ArithmeticResultType::Decimal => match BigDecimal::from_i32(v) {
                    Some(v1) => Ok(ArithmeticResult::Decimal(v1)),
                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                },
                ArithmeticResultType::Text => Ok(ArithmeticResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

// DECIMAL ARITHMETIC

#[derive(Debug)]
enum DecimalArithmeticArg {
    // Note. Decimal is not allowed as an arg to avoid loss of precision
    Number(i32),
    Decimal(BigDecimal),
    NumberArithmeticExpression(NumberArithmeticExpression),
    DecimalArithmeticExpression(DecimalArithmeticExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum DecimalArithmeticExpression {
    Add(Box<(DecimalArithmeticArg, Vec<DecimalArithmeticArg>)>),
    Multiply(Box<(DecimalArithmeticArg, Vec<DecimalArithmeticArg>)>),
    Subtract(Box<(DecimalArithmeticArg, Vec<DecimalArithmeticArg>)>),
    Divide(Box<(DecimalArithmeticArg, Vec<DecimalArithmeticArg>)>),
    Modulus(Box<(DecimalArithmeticArg, Vec<DecimalArithmeticArg>)>),
}

impl DecimalArithmeticExpression {
    fn unwrap_number(&self) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_decimal(&self) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ArithmeticResultType) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            DecimalArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Add),
        };
        let mut temp: BigDecimal = match BigDecimal::from_i32(1) {
            Some(v) => v,
            None => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
        };
        let init: Result<BigDecimal, CustomError> = match &args.0 {
            DecimalArithmeticArg::Number(v) => match BigDecimal::from_i32(*v) {
                Some(v1) => {
                    temp *= v1;
                    Ok(temp)
                }
                None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
            },
            DecimalArithmeticArg::Decimal(v1) => {
                temp *= v1;
                Ok(temp)
            }
            DecimalArithmeticArg::NumberArithmeticExpression(v) => v.unwrap_decimal(),
            DecimalArithmeticArg::DecimalArithmeticExpression(v) => v.unwrap_decimal(),
        };
        let result: Result<BigDecimal, CustomError> =
            args.1.iter().fold(init, |acc, val| match &acc {
                Ok(v) => match val {
                    DecimalArithmeticArg::Number(v1) => match BigDecimal::from_i32(*v1) {
                        Some(v1) => match operator {
                            ArithmeticOperator::Add => Ok(v + v1),
                            ArithmeticOperator::Multiply => Ok(v * v1),
                            ArithmeticOperator::Subtract => Ok(v - v1),
                            ArithmeticOperator::Divide => Ok(v / v1),
                            ArithmeticOperator::Modulus => Ok(v % v1),
                        },
                        None => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    DecimalArithmeticArg::Decimal(v1) => match operator {
                        ArithmeticOperator::Add => Ok(v + v1),
                        ArithmeticOperator::Multiply => Ok(v * v1),
                        ArithmeticOperator::Subtract => Ok(v - v1),
                        ArithmeticOperator::Divide => Ok(v / v1),
                        ArithmeticOperator::Modulus => Ok(v % v1),
                    },
                    DecimalArithmeticArg::NumberArithmeticExpression(v1) => {
                        match v1.unwrap_decimal() {
                            Ok(v2) => match operator {
                                ArithmeticOperator::Add => Ok(v + v2),
                                ArithmeticOperator::Multiply => Ok(v * v2),
                                ArithmeticOperator::Subtract => Ok(v - v2),
                                ArithmeticOperator::Divide => Ok(v / v2),
                                ArithmeticOperator::Modulus => Ok(v % v2),
                            },
                            Err(e) => Err(e),
                        }
                    }
                    DecimalArithmeticArg::DecimalArithmeticExpression(v1) => {
                        match v1.unwrap_decimal() {
                            Ok(v2) => match operator {
                                ArithmeticOperator::Add => Ok(v + v2),
                                ArithmeticOperator::Multiply => Ok(v * v2),
                                ArithmeticOperator::Subtract => Ok(v - v2),
                                ArithmeticOperator::Divide => Ok(v / v2),
                                ArithmeticOperator::Modulus => Ok(v % v2),
                            },
                            Err(e) => Err(e),
                        }
                    }
                },
                Err(_) => acc,
            });
        match result_type {
            ArithmeticResultType::Number => match result {
                Ok(v) => match v.to_i32() {
                    Some(v1) => Ok(ArithmeticResult::Number(v1)),
                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                },
                Err(e) => Err(e),
            },
            ArithmeticResultType::Decimal => match result {
                Ok(v) => Ok(ArithmeticResult::Decimal(v)),
                Err(e) => Err(e),
            },
            ArithmeticResultType::Text => match result {
                Ok(v) => Ok(ArithmeticResult::Text(v.to_string())),
                Err(e) => Err(e),
            },
        }
    }
}

// COMPARATOR OPS

#[derive(Debug)]
enum ComparatorResultType {
    Boolean,
    Text,
}

#[derive(Debug)]
enum ComparatorResult {
    Boolean(bool),
    Text(String),
}

#[derive(Debug)]
enum ComparatorOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
}

// NUMBER COMPARATOR

#[derive(Debug)]
enum NumberComparatorArg {
    // Note. Decimal is not allowed as an arg to avoid loss of precision
    Number(i32),
    NumberArithmeticExpression(NumberArithmeticExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum NumberComparatorExpression {
    Equals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    GreaterThan(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    LessThan(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    GreaterThanEquals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    LessThanEquals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
}

impl NumberComparatorExpression {
    fn unwrap_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ComparatorResultType) -> Result<ComparatorResult, CustomError> {
        let (args, operator) = match self {
            NumberComparatorExpression::Equals(v) => (v, ComparatorOperator::Equals),
            NumberComparatorExpression::GreaterThan(v) => (v, ComparatorOperator::GreaterThan),
            NumberComparatorExpression::LessThan(v) => (v, ComparatorOperator::LessThan),
            NumberComparatorExpression::GreaterThanEquals(v) => {
                (v, ComparatorOperator::GreaterThanEquals)
            }
            NumberComparatorExpression::LessThanEquals(v) => {
                (v, ComparatorOperator::LessThanEquals)
            }
        };
        let arg0: Result<i32, CustomError> = match &args.0 {
            NumberComparatorArg::Number(v) => Ok(*v),
            NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
        };
        let arg1: Result<i32, CustomError> = match &args.1 {
            NumberComparatorArg::Number(v) => Ok(*v),
            NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
        };
        let init: Result<bool, CustomError> = match (arg0, arg1) {
            (Ok(v), Ok(v1)) => match operator {
                ComparatorOperator::Equals => Ok(v == v1),
                ComparatorOperator::GreaterThan => Ok(v < v1),
                ComparatorOperator::LessThan => Ok(v > v1),
                ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                ComparatorOperator::LessThanEquals => Ok(v >= v1),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        };
        match args.2.len() == 0 {
            true => match init {
                Ok(v) => match result_type {
                    ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                    ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                },
                Err(e) => Err(e),
            },
            false => {
                let evaluated_args: Vec<Result<i32, CustomError>> = std::iter::once(&args.1)
                    .chain(&args.2)
                    .map(|val| match val {
                        NumberComparatorArg::Number(v) => Ok(*v),
                        NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
                    })
                    .collect();
                let result: Result<bool, CustomError> = evaluated_args
                    .iter()
                    .zip(&evaluated_args[1..])
                    .fold(init, |acc, val| match &acc {
                        Ok(true) => match val {
                            (Ok(v1), Ok(v2)) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                ComparatorOperator::LessThan => Ok(v1 > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                            },
                            (Ok(_), Err(e)) => Err(*e),
                            (Err(e), Ok(_)) => Err(*e),
                            (Err(e), Err(_)) => Err(*e),
                        },
                        _ => acc,
                    });
                match result {
                    Ok(v) => match result_type {
                        ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                        ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}

// DECIMAL COMPARATOR

#[derive(Debug)]
enum DecimalComparatorArg {
    Number(i32),
    Decimal(BigDecimal),
    NumberArithmeticExpression(NumberArithmeticExpression),
    DecimalArithmeticExpression(DecimalArithmeticExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum DecimalComparatorExpression {
    Equals(
        Box<(
            DecimalComparatorArg,
            DecimalComparatorArg,
            Vec<DecimalComparatorArg>,
        )>,
    ),
    GreaterThan(
        Box<(
            DecimalComparatorArg,
            DecimalComparatorArg,
            Vec<DecimalComparatorArg>,
        )>,
    ),
    LessThan(
        Box<(
            DecimalComparatorArg,
            DecimalComparatorArg,
            Vec<DecimalComparatorArg>,
        )>,
    ),
    GreaterThanEquals(
        Box<(
            DecimalComparatorArg,
            DecimalComparatorArg,
            Vec<DecimalComparatorArg>,
        )>,
    ),
    LessThanEquals(
        Box<(
            DecimalComparatorArg,
            DecimalComparatorArg,
            Vec<DecimalComparatorArg>,
        )>,
    ),
}

impl DecimalComparatorExpression {
    fn unwrap_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ComparatorResultType) -> Result<ComparatorResult, CustomError> {
        let (args, operator) = match self {
            DecimalComparatorExpression::Equals(v) => (v, ComparatorOperator::Equals),
            DecimalComparatorExpression::GreaterThan(v) => (v, ComparatorOperator::GreaterThan),
            DecimalComparatorExpression::LessThan(v) => (v, ComparatorOperator::LessThan),
            DecimalComparatorExpression::GreaterThanEquals(v) => {
                (v, ComparatorOperator::GreaterThanEquals)
            }
            DecimalComparatorExpression::LessThanEquals(v) => {
                (v, ComparatorOperator::LessThanEquals)
            }
        };
        let arg0: Result<BigDecimal, CustomError> = match &args.0 {
            DecimalComparatorArg::Number(v) => match BigDecimal::from_i32(*v) {
                Some(v1) => Ok(v1),
                None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
            },
            DecimalComparatorArg::Decimal(v) => Ok(v.clone()),
            DecimalComparatorArg::NumberArithmeticExpression(v) => v.unwrap_decimal(),
            DecimalComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_decimal(),
        };
        let arg1: Result<BigDecimal, CustomError> = match &args.1 {
            DecimalComparatorArg::Number(v) => match BigDecimal::from_i32(*v) {
                Some(v1) => Ok(v1),
                None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
            },
            DecimalComparatorArg::Decimal(v) => Ok(v.clone()),
            DecimalComparatorArg::NumberArithmeticExpression(v) => v.unwrap_decimal(),
            DecimalComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_decimal(),
        };
        let init: Result<bool, CustomError> = match (arg0, arg1) {
            (Ok(v), Ok(v1)) => match operator {
                ComparatorOperator::Equals => Ok(v == v1),
                ComparatorOperator::GreaterThan => Ok(v < v1),
                ComparatorOperator::LessThan => Ok(v > v1),
                ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                ComparatorOperator::LessThanEquals => Ok(v >= v1),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        };
        match args.2.len() == 0 {
            true => match init {
                Ok(v) => match result_type {
                    ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                    ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                },
                Err(e) => Err(e),
            },
            false => {
                let evaluated_args: Vec<Result<BigDecimal, CustomError>> = std::iter::once(&args.1)
                    .chain(&args.2)
                    .map(|val| match val {
                        DecimalComparatorArg::Number(v) => match BigDecimal::from_i32(*v) {
                            Some(v1) => Ok(v1),
                            None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        DecimalComparatorArg::Decimal(v) => Ok(v.clone()),
                        DecimalComparatorArg::NumberArithmeticExpression(v) => v.unwrap_decimal(),
                        DecimalComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_decimal(),
                    })
                    .collect();
                let result: Result<bool, CustomError> = evaluated_args
                    .iter()
                    .zip(&evaluated_args[1..])
                    .fold(init, |acc, val| match &acc {
                        Ok(true) => match val {
                            (Ok(v1), Ok(v2)) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                ComparatorOperator::LessThan => Ok(v1 > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                            },
                            (Ok(_), Err(e)) => Err(*e),
                            (Err(e), Ok(_)) => Err(*e),
                            (Err(e), Err(_)) => Err(*e),
                        },
                        _ => acc,
                    });
                match result {
                    Ok(v) => match result_type {
                        ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                        ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}

// TEXT COMPARATOR

#[derive(Debug)]
enum TextComparatorArg {
    Number(i32),
    Decimal(BigDecimal),
    NumberArithmeticExpression(NumberArithmeticExpression),
    DecimalArithmeticExpression(DecimalArithmeticExpression),
    NumberComparatorExpression(NumberComparatorExpression),
    DecimalComparatorExpression(DecimalComparatorExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum TextComparatorExpression {
    Equals(
        Box<(
            TextComparatorArg,
            TextComparatorArg,
            Vec<TextComparatorArg>,
        )>,
    ),
    GreaterThan(
        Box<(
            TextComparatorArg,
            TextComparatorArg,
            Vec<TextComparatorArg>,
        )>,
    ),
    LessThan(
        Box<(
            TextComparatorArg,
            TextComparatorArg,
            Vec<TextComparatorArg>,
        )>,
    ),
    GreaterThanEquals(
        Box<(
            TextComparatorArg,
            TextComparatorArg,
            Vec<TextComparatorArg>,
        )>,
    ),
    LessThanEquals(
        Box<(
            TextComparatorArg,
            TextComparatorArg,
            Vec<TextComparatorArg>,
        )>,
    ),
}

impl TextComparatorExpression {
    fn unwrap_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ComparatorResultType) -> Result<ComparatorResult, CustomError> {
        let (args, operator) = match self {
            TextComparatorExpression::Equals(v) => (v, ComparatorOperator::Equals),
            TextComparatorExpression::GreaterThan(v) => (v, ComparatorOperator::GreaterThan),
            TextComparatorExpression::LessThan(v) => (v, ComparatorOperator::LessThan),
            TextComparatorExpression::GreaterThanEquals(v) => {
                (v, ComparatorOperator::GreaterThanEquals)
            }
            TextComparatorExpression::LessThanEquals(v) => {
                (v, ComparatorOperator::LessThanEquals)
            }
        };
        let arg0: Result<String, CustomError> = match &args.0 {
            TextComparatorArg::Number(v) => Ok(v.to_string()),
            TextComparatorArg::Decimal(v) => Ok(v.to_string()),
            TextComparatorArg::NumberArithmeticExpression(v) => v.unwrap_text(),
            TextComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_text(),
            TextComparatorArg::NumberComparatorExpression(v) => v.unwrap_text(),
            TextComparatorArg::DecimalComparatorExpression(v) => v.unwrap_text(),
        };
        let arg1: Result<String, CustomError> = match &args.1 {
            TextComparatorArg::Number(v) => Ok(v.to_string()),
            TextComparatorArg::Decimal(v) => Ok(v.to_string()),
            TextComparatorArg::NumberArithmeticExpression(v) => v.unwrap_text(),
            TextComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_text(),
            TextComparatorArg::NumberComparatorExpression(v) => v.unwrap_text(),
            TextComparatorArg::DecimalComparatorExpression(v) => v.unwrap_text(),
        };
        let init: Result<bool, CustomError> = match (arg0, arg1) {
            (Ok(v), Ok(v1)) => match operator {
                ComparatorOperator::Equals => Ok(v == v1),
                ComparatorOperator::GreaterThan => Ok(v < v1),
                ComparatorOperator::LessThan => Ok(v > v1),
                ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                ComparatorOperator::LessThanEquals => Ok(v >= v1),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        };
        match args.2.len() == 0 {
            true => match init {
                Ok(v) => match result_type {
                    ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                    ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                },
                Err(e) => Err(e),
            },
            false => {
                let evaluated_args: Vec<Result<String, CustomError>> = std::iter::once(&args.1)
                    .chain(&args.2)
                    .map(|val| match val {
                        TextComparatorArg::Number(v) => Ok(v.to_string()),
                        TextComparatorArg::Decimal(v) => Ok(v.to_string()),
                        TextComparatorArg::NumberArithmeticExpression(v) => v.unwrap_text(),
                        TextComparatorArg::DecimalArithmeticExpression(v) => v.unwrap_text(),
                        TextComparatorArg::NumberComparatorExpression(v) => v.unwrap_text(),
                        TextComparatorArg::DecimalComparatorExpression(v) => v.unwrap_text(),
                    })
                    .collect();
                let result: Result<bool, CustomError> = evaluated_args
                    .iter()
                    .zip(&evaluated_args[1..])
                    .fold(init, |acc, val| match &acc {
                        Ok(true) => match val {
                            (Ok(v1), Ok(v2)) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                ComparatorOperator::LessThan => Ok(v1 > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                            },
                            (Ok(_), Err(e)) => Err(*e),
                            (Err(e), Ok(_)) => Err(*e),
                            (Err(e), Err(_)) => Err(*e),
                        },
                        _ => acc,
                    });
                match result {
                    Ok(v) => match result_type {
                        ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                        ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}

// LOGICAL OPS

#[derive(Debug)]
enum LogicalResultType {
    Boolean,
    Text,
}

#[derive(Debug)]
enum LogicalResult {
    Boolean(bool),
    Text(String),
}

// NUMBER COMPARATOR

#[derive(Debug)]
enum LogicalOperator {
    And,
    Or
}

#[derive(Debug)]
enum NumberComparatorArg {
    // Note. Decimal is not allowed as an arg to avoid loss of precision
    Number(i32),
    NumberArithmeticExpression(NumberArithmeticExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum NumberComparatorExpression {
    Equals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    GreaterThan(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    LessThan(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    GreaterThanEquals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
    LessThanEquals(
        Box<(
            NumberComparatorArg,
            NumberComparatorArg,
            Vec<NumberComparatorArg>,
        )>,
    ),
}

impl NumberComparatorExpression {
    fn unwrap_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn unwrap_text(&self) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ComparatorResultType) -> Result<ComparatorResult, CustomError> {
        let (args, operator) = match self {
            NumberComparatorExpression::Equals(v) => (v, ComparatorOperator::Equals),
            NumberComparatorExpression::GreaterThan(v) => (v, ComparatorOperator::GreaterThan),
            NumberComparatorExpression::LessThan(v) => (v, ComparatorOperator::LessThan),
            NumberComparatorExpression::GreaterThanEquals(v) => {
                (v, ComparatorOperator::GreaterThanEquals)
            }
            NumberComparatorExpression::LessThanEquals(v) => {
                (v, ComparatorOperator::LessThanEquals)
            }
        };
        let arg0: Result<i32, CustomError> = match &args.0 {
            NumberComparatorArg::Number(v) => Ok(*v),
            NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
        };
        let arg1: Result<i32, CustomError> = match &args.1 {
            NumberComparatorArg::Number(v) => Ok(*v),
            NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
        };
        let init: Result<bool, CustomError> = match (arg0, arg1) {
            (Ok(v), Ok(v1)) => match operator {
                ComparatorOperator::Equals => Ok(v == v1),
                ComparatorOperator::GreaterThan => Ok(v < v1),
                ComparatorOperator::LessThan => Ok(v > v1),
                ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                ComparatorOperator::LessThanEquals => Ok(v >= v1),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        };
        match args.2.len() == 0 {
            true => match init {
                Ok(v) => match result_type {
                    ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                    ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                },
                Err(e) => Err(e),
            },
            false => {
                let evaluated_args: Vec<Result<i32, CustomError>> = std::iter::once(&args.1)
                    .chain(&args.2)
                    .map(|val| match val {
                        NumberComparatorArg::Number(v) => Ok(*v),
                        NumberComparatorArg::NumberArithmeticExpression(v) => v.unwrap_number(),
                    })
                    .collect();
                let result: Result<bool, CustomError> = evaluated_args
                    .iter()
                    .zip(&evaluated_args[1..])
                    .fold(init, |acc, val| match &acc {
                        Ok(true) => match val {
                            (Ok(v1), Ok(v2)) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                ComparatorOperator::LessThan => Ok(v1 > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                            },
                            (Ok(_), Err(e)) => Err(*e),
                            (Err(e), Ok(_)) => Err(*e),
                            (Err(e), Err(_)) => Err(*e),
                        },
                        _ => acc,
                    });
                match result {
                    Ok(v) => match result_type {
                        ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                        ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}

impl LispExpression {
    // fn eval(
    //     result_type: LispExpressionResultType,
    //     expr: &LispExpression,
    // ) -> Result<LispExpressionResult, CustomError> {
    // match expr {
    //     LispExpression::Add { types, args } => {
    //         match Self::arithmetic_op(
    //             match result_type {
    //                 LispExpressionResultType::Number => ArithmeticResultType::Number,
    //                 LispExpressionResultType::Decimal => ArithmeticResultType::Decimal,
    //                 LispExpressionResultType::Text => ArithmeticResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ArithmeticOperator::Add,
    //         ) {
    //             Ok(v) => match v {
    //                 ArithmeticResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //                 ArithmeticResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //                 ArithmeticResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         }
    //     }
    //         LispExpression::Multiply { types, args } => {
    //             match Self::arithmetic_op(
    //                 match result_type {
    //                     LispExpressionResultType::Number => ArithmeticResultType::Number,
    //                     LispExpressionResultType::Decimal => ArithmeticResultType::Decimal,
    //                     LispExpressionResultType::Text => ArithmeticResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 types,
    //                 args,
    //                 ArithmeticOperator::Multiply,
    //             ) {
    //                 Ok(v) => match v {
    //                     ArithmeticResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //                     ArithmeticResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //                     ArithmeticResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Subtract { types, args } => {
    //             match Self::arithmetic_op(
    //                 match result_type {
    //                     LispExpressionResultType::Number => ArithmeticResultType::Number,
    //                     LispExpressionResultType::Decimal => ArithmeticResultType::Decimal,
    //                     LispExpressionResultType::Text => ArithmeticResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 types,
    //                 args,
    //                 ArithmeticOperator::Subtract,
    //             ) {
    //                 Ok(v) => match v {
    //                     ArithmeticResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //                     ArithmeticResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //                     ArithmeticResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Divide { types, args } => {
    //             match Self::arithmetic_op(
    //                 match result_type {
    //                     LispExpressionResultType::Number => ArithmeticResultType::Number,
    //                     LispExpressionResultType::Decimal => ArithmeticResultType::Decimal,
    //                     LispExpressionResultType::Text => ArithmeticResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 types,
    //                 args,
    //                 ArithmeticOperator::Divide,
    //             ) {
    //                 Ok(v) => match v {
    //                     ArithmeticResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //                     ArithmeticResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //                     ArithmeticResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Modulus { types, args } => {
    //             match Self::arithmetic_op(
    //                 match result_type {
    //                     LispExpressionResultType::Number => ArithmeticResultType::Number,
    //                     LispExpressionResultType::Decimal => ArithmeticResultType::Decimal,
    //                     LispExpressionResultType::Text => ArithmeticResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 types,
    //                 args,
    //                 ArithmeticOperator::Modulus,
    //             ) {
    //                 Ok(v) => match v {
    //                     ArithmeticResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //                     ArithmeticResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //                     ArithmeticResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Equals { types, args } => match Self::comparator_op(
    //             match result_type {
    //                 LispExpressionResultType::Boolean => ComparatorResultType::Boolean,
    //                 LispExpressionResultType::Text => ComparatorResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ComparatorOperator::Equals,
    //         ) {
    //             Ok(v) => match v {
    //                 ComparatorResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                 ComparatorResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         },
    //         LispExpression::GreaterThan { types, args } => match Self::comparator_op(
    //             match result_type {
    //                 LispExpressionResultType::Boolean => ComparatorResultType::Boolean,
    //                 LispExpressionResultType::Text => ComparatorResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ComparatorOperator::GreaterThan,
    //         ) {
    //             Ok(v) => match v {
    //                 ComparatorResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                 ComparatorResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         },
    //         LispExpression::LessThan { types, args } => match Self::comparator_op(
    //             match result_type {
    //                 LispExpressionResultType::Boolean => ComparatorResultType::Boolean,
    //                 LispExpressionResultType::Text => ComparatorResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ComparatorOperator::LessThan,
    //         ) {
    //             Ok(v) => match v {
    //                 ComparatorResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                 ComparatorResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         },
    //         LispExpression::GreaterThanEquals { types, args } => match Self::comparator_op(
    //             match result_type {
    //                 LispExpressionResultType::Boolean => ComparatorResultType::Boolean,
    //                 LispExpressionResultType::Text => ComparatorResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ComparatorOperator::GreaterThanEquals,
    //         ) {
    //             Ok(v) => match v {
    //                 ComparatorResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                 ComparatorResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         },
    //         LispExpression::LessThanEquals { types, args } => match Self::comparator_op(
    //             match result_type {
    //                 LispExpressionResultType::Boolean => ComparatorResultType::Boolean,
    //                 LispExpressionResultType::Text => ComparatorResultType::Text,
    //                 _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //             },
    //             types,
    //             args,
    //             ComparatorOperator::LessThanEquals,
    //         ) {
    //             Ok(v) => match v {
    //                 ComparatorResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                 ComparatorResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //             },
    //             Err(e) => Err(e),
    //         },
    //         LispExpression::And { args } => {
    //             match Self::logical_op_binary(
    //                 match result_type {
    //                     LispExpressionResultType::Boolean => LogicalResultType::Boolean,
    //                     LispExpressionResultType::Text => LogicalResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 args,
    //                 LogicalOperatorBinary::And,
    //             ) {
    //                 Ok(v) => match v {
    //                     LogicalResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                     LogicalResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Or { args } => {
    //             match Self::logical_op_binary(
    //                 match result_type {
    //                     LispExpressionResultType::Boolean => LogicalResultType::Boolean,
    //                     LispExpressionResultType::Text => LogicalResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 args,
    //                 LogicalOperatorBinary::Or,
    //             ) {
    //                 Ok(v) => match v {
    //                     LogicalResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                     LogicalResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         LispExpression::Not { args } => {
    //             match Self::logical_op_unary(
    //                 match result_type {
    //                     LispExpressionResultType::Boolean => LogicalResultType::Boolean,
    //                     LispExpressionResultType::Text => LogicalResultType::Text,
    //                     _ => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 args,
    //                 LogicalOperatorUnary::Not,
    //             ) {
    //                 Ok(v) => match v {
    //                     LogicalResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //                     LogicalResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //                 },
    //                 Err(e) => Err(e),
    //             }
    //         }
    //         // LispExpression::Match { types, args } => {
    //         //     match Self::control_flow_op(
    //         //         match result_type {
    //         //             LispExpressionResultType::Number => ControlFlowResultType::Number,
    //         //             LispExpressionResultType::Decimal => ControlFlowResultType::Decimal,
    //         //             LispExpressionResultType::Boolean => ControlFlowResultType::Boolean,
    //         //             LispExpressionResultType::Text => ControlFlowResultType::Text,
    //         //         },
    //         //         types,
    //         //         args,
    //         //     ) {
    //         //         Ok(v) => match v {
    //         //             ControlFlowResult::Number(v1) => Ok(LispExpressionResult::Number(v1)),
    //         //             ControlFlowResult::Decimal(v1) => Ok(LispExpressionResult::Decimal(v1)),
    //         //             ControlFlowResult::Boolean(v1) => Ok(LispExpressionResult::Boolean(v1)),
    //         //             ControlFlowResult::Text(v1) => Ok(LispExpressionResult::Text(v1)),
    //         //         },
    //         //         Err(e) => Err(e),
    //         //     }
    //         // }
    //     }
    // }

    fn get_text(expr: &LispExpression) -> Result<String, CustomError> {
        // Maybe this is where expected result types could override the ones below
        // Or probably no use of them here, as they can be rather used while constructing lisp expressions
        match Self::eval(LispExpressionResultType::Text, expr)? {
            LispExpressionResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_number(expr: &LispExpression) -> Result<i32, CustomError> {
        match Self::eval(LispExpressionResultType::Number, expr)? {
            LispExpressionResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_decimal(expr: &LispExpression) -> Result<BigDecimal, CustomError> {
        match Self::eval(LispExpressionResultType::Decimal, expr)? {
            LispExpressionResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_boolean(expr: &LispExpression) -> Result<bool, CustomError> {
        match Self::eval(LispExpressionResultType::Boolean, expr)? {
            LispExpressionResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn comparator_op(
        result_type: ComparatorResultType,
        types: &ComparatorArgType,
        args: &Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
        operator: ComparatorOperator,
    ) -> Result<ComparatorResult, CustomError> {
        match types {
            ComparatorArgType::Number => {
                let init: Result<bool, CustomError> = match &args.0 {
                    ComparatorArg::Number(v) => match &args.1 {
                        ComparatorArg::Number(v1) => match operator {
                            ComparatorOperator::Equals => Ok(v == v1),
                            ComparatorOperator::GreaterThan => Ok(v < v1),
                            ComparatorOperator::LessThan => Ok(v > v1),
                            ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                            ComparatorOperator::LessThanEquals => Ok(v >= v1),
                        },
                        ComparatorArg::Expression(v1) => match LispExpression::get_number(v1) {
                            Ok(v2) => match operator {
                                ComparatorOperator::Equals => Ok(*v == v2),
                                ComparatorOperator::GreaterThan => Ok(*v < v2),
                                ComparatorOperator::LessThan => Ok(*v > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(*v <= v2),
                                ComparatorOperator::LessThanEquals => Ok(*v >= v2),
                            },
                            Err(e) => Err(e),
                        },
                        _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ComparatorArg::Expression(v) => match LispExpression::get_number(v) {
                        Ok(v1) => match &args.1 {
                            ComparatorArg::Number(v2) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == *v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < *v2),
                                ComparatorOperator::LessThan => Ok(v1 > *v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= *v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= *v2),
                            },
                            ComparatorArg::Expression(v2) => match LispExpression::get_number(v2) {
                                Ok(v3) => match operator {
                                    ComparatorOperator::Equals => Ok(v1 == v3),
                                    ComparatorOperator::GreaterThan => Ok(v1 < v3),
                                    ComparatorOperator::LessThan => Ok(v1 > v3),
                                    ComparatorOperator::GreaterThanEquals => Ok(v1 <= v3),
                                    ComparatorOperator::LessThanEquals => Ok(v1 >= v3),
                                },
                                Err(e) => Err(e),
                            },
                            _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        Err(e) => Err(e),
                    },
                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                };
                if args.2.len() == 0 {
                    match init {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let evaluated_args: Vec<Result<LispExpressionResult, CustomError>> =
                        std::iter::once(&args.1)
                            .chain(&args.2)
                            .map(|val| match val {
                                ComparatorArg::Number(v) => Ok(LispExpressionResult::Number(*v)),
                                ComparatorArg::Expression(v) => match LispExpression::get_number(v)
                                {
                                    Ok(v1) => Ok(LispExpressionResult::Number(v1)),
                                    Err(e) => Err(e),
                                },
                                _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                            })
                            .collect();
                    let result: Result<bool, CustomError> = evaluated_args
                        .iter()
                        .zip(&evaluated_args[1..])
                        .fold(init, |acc, val| match &acc {
                            Ok(v) => match v {
                                true => match val {
                                    (
                                        Ok(LispExpressionResult::Number(v1)),
                                        Ok(LispExpressionResult::Number(v2)),
                                    ) => match operator {
                                        ComparatorOperator::Equals => Ok(v1 == v2),
                                        ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                        ComparatorOperator::LessThan => Ok(v1 > v2),
                                        ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                        ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                                    },
                                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                false => Ok(false),
                            },
                            Err(_) => acc,
                        });
                    match result {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                }
            }
            ComparatorArgType::Decimal => {
                let init: Result<bool, CustomError> = match &args.0 {
                    ComparatorArg::Decimal(v) => match &args.1 {
                        ComparatorArg::Decimal(v1) => match operator {
                            ComparatorOperator::Equals => Ok(v == v1),
                            ComparatorOperator::GreaterThan => Ok(v < v1),
                            ComparatorOperator::LessThan => Ok(v > v1),
                            ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                            ComparatorOperator::LessThanEquals => Ok(v >= v1),
                        },
                        ComparatorArg::Expression(v1) => match LispExpression::get_decimal(v1) {
                            Ok(v2) => match operator {
                                ComparatorOperator::Equals => Ok(*v == v2),
                                ComparatorOperator::GreaterThan => Ok(*v < v2),
                                ComparatorOperator::LessThan => Ok(*v > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(*v <= v2),
                                ComparatorOperator::LessThanEquals => Ok(*v >= v2),
                            },
                            Err(e) => Err(e),
                        },
                        _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ComparatorArg::Expression(v) => match LispExpression::get_decimal(v) {
                        Ok(v1) => match &args.1 {
                            ComparatorArg::Decimal(v2) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == *v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < *v2),
                                ComparatorOperator::LessThan => Ok(v1 > *v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= *v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= *v2),
                            },
                            ComparatorArg::Expression(v2) => {
                                match LispExpression::get_decimal(v2) {
                                    Ok(v3) => match operator {
                                        ComparatorOperator::Equals => Ok(v1 == v3),
                                        ComparatorOperator::GreaterThan => Ok(v1 < v3),
                                        ComparatorOperator::LessThan => Ok(v1 > v3),
                                        ComparatorOperator::GreaterThanEquals => Ok(v1 <= v3),
                                        ComparatorOperator::LessThanEquals => Ok(v1 >= v3),
                                    },
                                    Err(e) => Err(e),
                                }
                            }
                            _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        Err(e) => Err(e),
                    },
                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                };
                if args.2.len() == 0 {
                    match init {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let evaluated_args: Vec<Result<LispExpressionResult, CustomError>> =
                        std::iter::once(&args.1)
                            .chain(&args.2)
                            .map(|val| match val {
                                ComparatorArg::Decimal(v) => {
                                    Ok(LispExpressionResult::Decimal(v.clone()))
                                }
                                ComparatorArg::Expression(v) => {
                                    match LispExpression::get_decimal(v) {
                                        Ok(v1) => Ok(LispExpressionResult::Decimal(v1)),
                                        Err(e) => Err(e),
                                    }
                                }
                                _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                            })
                            .collect();
                    let result: Result<bool, CustomError> = evaluated_args
                        .iter()
                        .zip(&evaluated_args[1..])
                        .fold(init, |acc, val| match &acc {
                            Ok(v) => match v {
                                true => match val {
                                    (
                                        Ok(LispExpressionResult::Decimal(v1)),
                                        Ok(LispExpressionResult::Decimal(v2)),
                                    ) => match operator {
                                        ComparatorOperator::Equals => Ok(v1 == v2),
                                        ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                        ComparatorOperator::LessThan => Ok(v1 > v2),
                                        ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                        ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                                    },
                                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                false => Ok(false),
                            },
                            Err(_) => acc,
                        });
                    match result {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                }
            }
            ComparatorArgType::Text => {
                let init: Result<bool, CustomError> = match &args.0 {
                    ComparatorArg::Text(v) => match &args.1 {
                        ComparatorArg::Text(v1) => match operator {
                            ComparatorOperator::Equals => Ok(v == v1),
                            ComparatorOperator::GreaterThan => Ok(v < v1),
                            ComparatorOperator::LessThan => Ok(v > v1),
                            ComparatorOperator::GreaterThanEquals => Ok(v <= v1),
                            ComparatorOperator::LessThanEquals => Ok(v >= v1),
                        },
                        ComparatorArg::Expression(v1) => match LispExpression::get_text(v1) {
                            Ok(v2) => match operator {
                                ComparatorOperator::Equals => Ok(*v == v2),
                                ComparatorOperator::GreaterThan => Ok(*v < v2),
                                ComparatorOperator::LessThan => Ok(*v > v2),
                                ComparatorOperator::GreaterThanEquals => Ok(*v <= v2),
                                ComparatorOperator::LessThanEquals => Ok(*v >= v2),
                            },
                            Err(e) => Err(e),
                        },
                        _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ComparatorArg::Expression(v) => match LispExpression::get_text(v) {
                        Ok(v1) => match &args.1 {
                            ComparatorArg::Text(v2) => match operator {
                                ComparatorOperator::Equals => Ok(v1 == *v2),
                                ComparatorOperator::GreaterThan => Ok(v1 < *v2),
                                ComparatorOperator::LessThan => Ok(v1 > *v2),
                                ComparatorOperator::GreaterThanEquals => Ok(v1 <= *v2),
                                ComparatorOperator::LessThanEquals => Ok(v1 >= *v2),
                            },
                            ComparatorArg::Expression(v2) => match LispExpression::get_text(v2) {
                                Ok(v3) => match operator {
                                    ComparatorOperator::Equals => Ok(v1 == v3),
                                    ComparatorOperator::GreaterThan => Ok(v1 < v3),
                                    ComparatorOperator::LessThan => Ok(v1 > v3),
                                    ComparatorOperator::GreaterThanEquals => Ok(v1 <= v3),
                                    ComparatorOperator::LessThanEquals => Ok(v1 >= v3),
                                },
                                Err(e) => Err(e),
                            },
                            _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        Err(e) => Err(e),
                    },
                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                };
                if args.2.len() == 0 {
                    match init {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let evaluated_args: Vec<Result<LispExpressionResult, CustomError>> =
                        std::iter::once(&args.1)
                            .chain(&args.2)
                            .map(|val| match val {
                                ComparatorArg::Text(v) => Ok(LispExpressionResult::Text(v.clone())),
                                ComparatorArg::Expression(v) => {
                                    match LispExpression::get_decimal(v) {
                                        Ok(v1) => Ok(LispExpressionResult::Decimal(v1)),
                                        Err(e) => Err(e),
                                    }
                                }
                                _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                            })
                            .collect();
                    let result: Result<bool, CustomError> = evaluated_args
                        .iter()
                        .zip(&evaluated_args[1..])
                        .fold(init, |acc, val| match &acc {
                            Ok(v) => match v {
                                true => match val {
                                    (
                                        Ok(LispExpressionResult::Text(v1)),
                                        Ok(LispExpressionResult::Text(v2)),
                                    ) => match operator {
                                        ComparatorOperator::Equals => Ok(v1 == v2),
                                        ComparatorOperator::GreaterThan => Ok(v1 < v2),
                                        ComparatorOperator::LessThan => Ok(v1 > v2),
                                        ComparatorOperator::GreaterThanEquals => Ok(v1 <= v2),
                                        ComparatorOperator::LessThanEquals => Ok(v1 >= v2),
                                    },
                                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                false => Ok(false),
                            },
                            Err(_) => acc,
                        });
                    match result {
                        Ok(v) => match result_type {
                            ComparatorResultType::Boolean => Ok(ComparatorResult::Boolean(v)),
                            ComparatorResultType::Text => Ok(ComparatorResult::Text(v.to_string())),
                        },
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }

    fn logical_op_binary(
        result_type: LogicalResultType,
        args: &Box<(LogicalArg, LogicalArg, Vec<LogicalArg>)>,
        operator: LogicalOperatorBinary,
    ) -> Result<LogicalResult, CustomError> {
        let init: Result<bool, CustomError> = match &args.0 {
            LogicalArg::Boolean(v) => match &args.1 {
                LogicalArg::Boolean(v1) => match operator {
                    LogicalOperatorBinary::And => Ok(*v && *v1),
                    LogicalOperatorBinary::Or => Ok(*v || *v1),
                },
                LogicalArg::Expression(v1) => match LispExpression::get_boolean(&v1) {
                    Ok(v2) => match operator {
                        LogicalOperatorBinary::And => Ok(*v && v2),
                        LogicalOperatorBinary::Or => Ok(*v || v2),
                    },
                    Err(e) => Err(e),
                },
            },
            LogicalArg::Expression(v) => match LispExpression::get_boolean(&v) {
                Ok(v1) => match &args.1 {
                    LogicalArg::Boolean(v2) => match operator {
                        LogicalOperatorBinary::And => Ok(v1 && *v2),
                        LogicalOperatorBinary::Or => Ok(v1 || *v2),
                    },
                    LogicalArg::Expression(v2) => match LispExpression::get_boolean(&v2) {
                        Ok(v3) => match operator {
                            LogicalOperatorBinary::And => Ok(v1 && v3),
                            LogicalOperatorBinary::Or => Ok(v1 || v3),
                        },
                        Err(e) => Err(e),
                    },
                },
                Err(e) => Err(e),
            },
        };
        let result: Result<bool, CustomError> = args.2.iter().fold(init, |acc, val| match &acc {
            Ok(v) => match val {
                LogicalArg::Boolean(v1) => match operator {
                    LogicalOperatorBinary::And => Ok(*v && *v1),
                    LogicalOperatorBinary::Or => Ok(*v || *v1),
                },
                LogicalArg::Expression(v1) => match LispExpression::get_boolean(&v1) {
                    Ok(v2) => match operator {
                        LogicalOperatorBinary::And => Ok(*v && v2),
                        LogicalOperatorBinary::Or => Ok(*v || v2),
                    },
                    Err(e) => Err(e),
                },
            },
            Err(_) => acc,
        });
        match result {
            Ok(v) => match result_type {
                LogicalResultType::Boolean => Ok(LogicalResult::Boolean(v)),
                LogicalResultType::Text => Ok(LogicalResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn logical_op_unary(
        result_type: LogicalResultType,
        args: &Box<LogicalArg>,
        operator: LogicalOperatorUnary,
    ) -> Result<LogicalResult, CustomError> {
        let result: Result<bool, CustomError> = match args.as_ref() {
            LogicalArg::Boolean(v) => match operator {
                LogicalOperatorUnary::Not => Ok(!v),
            },
            LogicalArg::Expression(v) => match LispExpression::get_boolean(&v) {
                Ok(v1) => match operator {
                    LogicalOperatorUnary::Not => Ok(!v1),
                },
                Err(e) => Err(e),
            },
        };
        match result {
            Ok(v) => match result_type {
                LogicalResultType::Boolean => Ok(LogicalResult::Boolean(v)),
                LogicalResultType::Text => Ok(LogicalResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    // fn get_result_type(arg_type: &ControlFlowArgType) -> LispExpressionResultType {
    //     match arg_type {
    //         ControlFlowArgType::Number => LispExpressionResultType::Number,
    //         ControlFlowArgType::Decimal => LispExpressionResultType::Decimal,
    //         ControlFlowArgType::Boolean => LispExpressionResultType::Boolean,
    //         ControlFlowArgType::Text => LispExpressionResultType::Text,
    //     }
    // }

    // fn control_flow_op(
    //     result_type: ControlFlowResultType,
    //     types: &(ControlFlowArgType, ControlFlowArgType),
    //     args: &Box<(
    //         LispExpression,
    //         Vec<(LispExpression, LispExpression)>,
    //         LispExpression,
    //     )>,
    // ) -> Result<ControlFlowResult, CustomError> {
    //     match args.1.is_empty() {
    //         true => match LispExpression::get_result_type(&types.1) {
    //             LispExpressionResultType::Number => match LispExpression::get_number(&args.2) {
    //                 Ok(v1) => match result_type {
    //                     ControlFlowResultType::Number => Ok(ControlFlowResult::Number(v1)),
    //                     ControlFlowResultType::Decimal => match BigDecimal::from_i32(v1) {
    //                         Some(v2) => Ok(ControlFlowResult::Decimal(v2)),
    //                         None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                     },
    //                     ControlFlowResultType::Text => Ok(ControlFlowResult::Text(v1.to_string())),
    //                     _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Decimal => match LispExpression::get_decimal(&args.2) {
    //                 Ok(v1) => match result_type {
    //                     ControlFlowResultType::Number => match v1.to_i32() {
    //                         Some(v2) => Ok(ControlFlowResult::Number(v2)),
    //                         None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                     },
    //                     ControlFlowResultType::Decimal => Ok(ControlFlowResult::Decimal(v1)),
    //                     ControlFlowResultType::Text => Ok(ControlFlowResult::Text(v1.to_string())),
    //                     _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Boolean => match LispExpression::get_boolean(&args.2) {
    //                 Ok(v1) => match result_type {
    //                     ControlFlowResultType::Boolean => Ok(ControlFlowResult::Boolean(v1)),
    //                     ControlFlowResultType::Text => Ok(ControlFlowResult::Text(v1.to_string())),
    //                     _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Text => match LispExpression::get_text(&args.2) {
    //                 Ok(v1) => match result_type {
    //                     ControlFlowResultType::Text => Ok(ControlFlowResult::Text(v1)),
    //                     _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //         },
    //         false => match LispExpression::get_result_type(&types.0) {
    //             LispExpressionResultType::Number => match LispExpression::get_number(&args.0) {
    //                 Ok(v) => match LispExpression::get_result_type(&types.1) {
    //                     LispExpressionResultType::Number => {
    //                         let init: Result<(bool, i32), CustomError> = Ok((false, 0));
    //                         let result: Result<(bool, i32), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_number(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_number(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Number(v1.1)),
    //                                 false => match LispExpression::get_number(&args.2) {
    //                                     Ok(v1) => match result_type {
    //                                         ControlFlowResultType::Number => {
    //                                             Ok(ControlFlowResult::Number(v1))
    //                                         }
    //                                         ControlFlowResultType::Decimal => {
    //                                             match BigDecimal::from_i32(v1) {
    //                                                 Some(v2) => Ok(ControlFlowResult::Decimal(v2)),
    //                                                 None => Err(CustomError::Message(
    //                                                     UNEXPECTED_ERROR.to_string(),
    //                                                 )),
    //                                             }
    //                                         }
    //                                         ControlFlowResultType::Text => {
    //                                             Ok(ControlFlowResult::Text(v1.to_string()))
    //                                         }
    //                                         _ => Err(CustomError::Message(
    //                                             UNEXPECTED_ERROR.to_string(),
    //                                         )),
    //                                     },
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Decimal => {
    //                         let init: Result<(bool, BigDecimal), CustomError> =
    //                             match BigDecimal::from_i32(0) {
    //                                 Some(v1) => Ok((false, v1)),
    //                                 None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                             };
    //                         let result: Result<(bool, BigDecimal), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_number(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_decimal(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Decimal(v1.1)),
    //                                 false => match LispExpression::get_decimal(&args.2) {
    //                                     Ok(v1) => match result_type {
    //                                         ControlFlowResultType::Number => match v1.to_i32() {
    //                                             Some(v2) => Ok(ControlFlowResult::Number(v2)),
    //                                             None => Err(CustomError::Message(
    //                                                 UNEXPECTED_ERROR.to_string(),
    //                                             )),
    //                                         },
    //                                         ControlFlowResultType::Decimal => {
    //                                             Ok(ControlFlowResult::Decimal(v1))
    //                                         }
    //                                         ControlFlowResultType::Text => {
    //                                             Ok(ControlFlowResult::Text(v1.to_string()))
    //                                         }
    //                                         _ => Err(CustomError::Message(
    //                                             UNEXPECTED_ERROR.to_string(),
    //                                         )),
    //                                     },
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Boolean => {
    //                         let init: Result<(bool, bool), CustomError> = Ok((false, false));
    //                         let result: Result<(bool, bool), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_number(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_boolean(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Boolean(v1.1)),
    //                                 false => match LispExpression::get_boolean(&args.2) {
    //                                     Ok(v1) => match result_type {
    //                                         ControlFlowResultType::Boolean => {
    //                                             Ok(ControlFlowResult::Boolean(v1))
    //                                         }
    //                                         ControlFlowResultType::Text => {
    //                                             Ok(ControlFlowResult::Text(v1.to_string()))
    //                                         }
    //                                         _ => Err(CustomError::Message(
    //                                             UNEXPECTED_ERROR.to_string(),
    //                                         )),
    //                                     },
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Text => {
    //                         let init: Result<(bool, String), CustomError> =
    //                             Ok((false, String::from("")));
    //                         let result: Result<(bool, String), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_number(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_text(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Text(v1.1)),
    //                                 false => match LispExpression::get_text(&args.2) {
    //                                     Ok(v1) => match result_type {
    //                                         ControlFlowResultType::Text => {
    //                                             Ok(ControlFlowResult::Text(v1))
    //                                         }
    //                                         _ => Err(CustomError::Message(
    //                                             UNEXPECTED_ERROR.to_string(),
    //                                         )),
    //                                     },
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Decimal => match LispExpression::get_decimal(&args.0) {
    //                 Ok(v) => match LispExpression::get_result_type(&types.1) {
    //                     LispExpressionResultType::Number => {
    //                         let init: Result<(bool, i32), CustomError> = Ok((false, 0));
    //                         let result: Result<(bool, i32), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_decimal(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_number(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Number(v1.1)),
    //                                 false => match LispExpression::get_number(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Number(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Decimal => {
    //                         let init: Result<(bool, BigDecimal), CustomError> =
    //                             match BigDecimal::from_i32(0) {
    //                                 Some(v1) => Ok((false, v1)),
    //                                 None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                             };
    //                         let result: Result<(bool, BigDecimal), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_decimal(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_decimal(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Decimal(v1.1)),
    //                                 false => match LispExpression::get_decimal(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Decimal(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Boolean => {
    //                         let init: Result<(bool, bool), CustomError> = Ok((false, false));
    //                         let result: Result<(bool, bool), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_decimal(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_boolean(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Boolean(v1.1)),
    //                                 false => match LispExpression::get_boolean(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Boolean(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Text => {
    //                         let init: Result<(bool, String), CustomError> =
    //                             Ok((false, String::from("")));
    //                         let result: Result<(bool, String), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_decimal(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_text(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Text(v1.1)),
    //                                 false => match LispExpression::get_text(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Text(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Boolean => match LispExpression::get_boolean(&args.0) {
    //                 Ok(v) => match LispExpression::get_result_type(&types.1) {
    //                     LispExpressionResultType::Number => {
    //                         let init: Result<(bool, i32), CustomError> = Ok((false, 0));
    //                         let result: Result<(bool, i32), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_boolean(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_number(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Number(v1.1)),
    //                                 false => match LispExpression::get_number(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Number(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Decimal => {
    //                         let init: Result<(bool, BigDecimal), CustomError> =
    //                             match BigDecimal::from_i32(0) {
    //                                 Some(v1) => Ok((false, v1)),
    //                                 None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                             };
    //                         let result: Result<(bool, BigDecimal), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_boolean(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_decimal(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Decimal(v1.1)),
    //                                 false => match LispExpression::get_decimal(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Decimal(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Boolean => {
    //                         let init: Result<(bool, bool), CustomError> = Ok((false, false));
    //                         let result: Result<(bool, bool), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_boolean(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_boolean(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Boolean(v1.1)),
    //                                 false => match LispExpression::get_boolean(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Boolean(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Text => {
    //                         let init: Result<(bool, String), CustomError> =
    //                             Ok((false, String::from("")));
    //                         let result: Result<(bool, String), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_boolean(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_text(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Text(v1.1)),
    //                                 false => match LispExpression::get_text(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Text(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //             LispExpressionResultType::Text => match LispExpression::get_text(&args.0) {
    //                 Ok(v) => match LispExpression::get_result_type(&types.1) {
    //                     LispExpressionResultType::Number => {
    //                         let init: Result<(bool, i32), CustomError> = Ok((false, 0));
    //                         let result: Result<(bool, i32), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_text(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_number(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Number(v1.1)),
    //                                 false => match LispExpression::get_number(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Number(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Decimal => {
    //                         let init: Result<(bool, BigDecimal), CustomError> =
    //                             match BigDecimal::from_i32(0) {
    //                                 Some(v1) => Ok((false, v1)),
    //                                 None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
    //                             };
    //                         let result: Result<(bool, BigDecimal), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_text(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_decimal(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Decimal(v1.1)),
    //                                 false => match LispExpression::get_decimal(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Decimal(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Boolean => {
    //                         let init: Result<(bool, bool), CustomError> = Ok((false, false));
    //                         let result: Result<(bool, bool), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_text(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_boolean(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Boolean(v1.1)),
    //                                 false => match LispExpression::get_boolean(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Boolean(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                     LispExpressionResultType::Text => {
    //                         let init: Result<(bool, String), CustomError> =
    //                             Ok((false, String::from("")));
    //                         let result: Result<(bool, String), CustomError> =
    //                             args.1.iter().fold(init, |acc, val| match &acc {
    //                                 Ok(v1) => match v1.0 {
    //                                     true => acc,
    //                                     false => match LispExpression::get_text(&val.0) {
    //                                         Ok(v2) => match v == v2 {
    //                                             true => match LispExpression::get_text(&val.1) {
    //                                                 Ok(v3) => Ok((true, v3)),
    //                                                 Err(e) => Err(e),
    //                                             },
    //                                             false => acc,
    //                                         },
    //                                         Err(e) => Err(e),
    //                                     },
    //                                 },
    //                                 Err(_) => acc,
    //                             });
    //                         match result {
    //                             Ok(v1) => match v1.0 {
    //                                 true => Ok(ControlFlowResult::Text(v1.1)),
    //                                 false => match LispExpression::get_text(&args.2) {
    //                                     Ok(v1) => Ok(ControlFlowResult::Text(v1)),
    //                                     Err(e) => Err(e),
    //                                 },
    //                             },
    //                             Err(e) => Err(e),
    //                         }
    //                     }
    //                 },
    //                 Err(e) => Err(e),
    //             },
    //         },
    //     }
    // }
}

// Use rust-decimal over bigdecimal
// Figure this out after implementing Lisp stuff
// And diesel stuff
fn main() {
    // let mut book_reviews = HashMap::new();
    // book_reviews.insert(
    //     "Adventures of Huckleberry Finn".to_string(),
    //     "My favorite book.".to_string(),
    // );
    let expr1: LispExpression = LispExpression::Add {
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
            vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
        )),
    };
    let expr2: LispExpression = LispExpression::Multiply {
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
            vec![
                ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
                ArithmeticArg::Expression(expr1),
            ],
        )),
    };
    println!("{:?}", LispExpression::get_number(&expr2).unwrap());

    let expr3: LispExpression = LispExpression::GreaterThanEquals {
        types: ComparatorArgType::Number,
        args: Box::new((
            ComparatorArg::Number(12),
            ComparatorArg::Number(22),
            vec![ComparatorArg::Number(22)],
        )),
    };
    let expr4: LispExpression = LispExpression::GreaterThanEquals {
        types: ComparatorArgType::Decimal,
        args: Box::new((
            ComparatorArg::Decimal(BigDecimal::from(2)),
            ComparatorArg::Decimal(BigDecimal::from(3)),
            vec![ComparatorArg::Decimal(BigDecimal::from_str("3.3").unwrap())],
        )),
    };
    println!("{:?}", LispExpression::get_boolean(&expr3).unwrap());
    println!("{:?}", LispExpression::get_boolean(&expr4).unwrap());
}

#[cfg(test)]
mod lisp_tests {
    use super::*;

    #[test]
    fn calculate() {
        let expr1: LispExpression = LispExpression::Add {
            types: (ArithmeticArgType::Number, vec![]),
            args: Box::new((
                ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
                vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
            )),
        };
        let expr2: LispExpression = LispExpression::Add {
            types: (ArithmeticArgType::Number, vec![]),
            args: Box::new((
                ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
                vec![
                    ArithmeticArg::Decimal(BigDecimal::from_i32(13).unwrap()),
                    ArithmeticArg::Expression(expr1),
                ],
            )),
        };
        assert_eq!(LispExpression::get_number(&expr2).unwrap(), 32);
    }
}
