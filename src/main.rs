/* Copyright (C) Logicarth (OPC) Private Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::str::FromStr;

const UNEXPECTED_ERROR: &str = "Unexpected Error";

// Making invalid expressions syntactically invalid, probably by wrapping expressions of different result types into their own types
// 0. Prefer match over if, for consistency
// 1. Do Internationalization
// 2. Implement concat, regex, identity
// 3. Implement symbols
// 4. Implement dot operator
// 5. Write test cases and build some audio visual documentation
#[derive(Debug, Clone)]
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
    fn get_number(&self) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_decimal(&self) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ArithmeticResultType) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            NumberArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Multiply),
            NumberArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Subtract),
            NumberArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Divide),
            NumberArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Modulus),
        };
        let init: Result<i32, CustomError> = match &args.0 {
            NumberArithmeticArg::Number(v) => Ok(*v),
            NumberArithmeticArg::NumberArithmeticExpression(v1) => v1.get_number(),
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
                NumberArithmeticArg::NumberArithmeticExpression(v1) => match v1.get_number() {
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
    fn get_number(&self) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_decimal(&self) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: ArithmeticResultType) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            DecimalArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Multiply),
            DecimalArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Subtract),
            DecimalArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Divide),
            DecimalArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Modulus),
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
            DecimalArithmeticArg::NumberArithmeticExpression(v) => v.get_decimal(),
            DecimalArithmeticArg::DecimalArithmeticExpression(v) => v.get_decimal(),
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
                        match v1.get_decimal() {
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
                        match v1.get_decimal() {
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
    fn get_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
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
            NumberComparatorArg::NumberArithmeticExpression(v) => v.get_number(),
        };
        let arg1: Result<i32, CustomError> = match &args.1 {
            NumberComparatorArg::Number(v) => Ok(*v),
            NumberComparatorArg::NumberArithmeticExpression(v) => v.get_number(),
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
                        NumberComparatorArg::NumberArithmeticExpression(v) => v.get_number(),
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
                            (Ok(_), Err(e)) => Err(e.clone()),
                            (Err(e), Ok(_)) => Err(e.clone()),
                            (Err(e), Err(_)) => Err(e.clone()),
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
    fn get_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
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
            DecimalComparatorArg::NumberArithmeticExpression(v) => v.get_decimal(),
            DecimalComparatorArg::DecimalArithmeticExpression(v) => v.get_decimal(),
        };
        let arg1: Result<BigDecimal, CustomError> = match &args.1 {
            DecimalComparatorArg::Number(v) => match BigDecimal::from_i32(*v) {
                Some(v1) => Ok(v1),
                None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
            },
            DecimalComparatorArg::Decimal(v) => Ok(v.clone()),
            DecimalComparatorArg::NumberArithmeticExpression(v) => v.get_decimal(),
            DecimalComparatorArg::DecimalArithmeticExpression(v) => v.get_decimal(),
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
                        DecimalComparatorArg::NumberArithmeticExpression(v) => v.get_decimal(),
                        DecimalComparatorArg::DecimalArithmeticExpression(v) => v.get_decimal(),
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
                            (Ok(_), Err(e)) => Err(e.clone()),
                            (Err(e), Ok(_)) => Err(e.clone()),
                            (Err(e), Err(_)) => Err(e.clone()),
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
    LogicalBinaryExpression(LogicalBinaryExpression),
    LogicalUnaryExpression(LogicalUnaryExpression),
    // Expression(ArithmeticControlFlowExpression),
}

#[derive(Debug)]
enum TextComparatorExpression {
    Equals(Box<(TextComparatorArg, TextComparatorArg, Vec<TextComparatorArg>)>),
    GreaterThan(Box<(TextComparatorArg, TextComparatorArg, Vec<TextComparatorArg>)>),
    LessThan(Box<(TextComparatorArg, TextComparatorArg, Vec<TextComparatorArg>)>),
    GreaterThanEquals(Box<(TextComparatorArg, TextComparatorArg, Vec<TextComparatorArg>)>),
    LessThanEquals(Box<(TextComparatorArg, TextComparatorArg, Vec<TextComparatorArg>)>),
}

impl TextComparatorExpression {
    fn get_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
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
            TextComparatorExpression::LessThanEquals(v) => (v, ComparatorOperator::LessThanEquals),
        };
        let arg0: Result<String, CustomError> = match &args.0 {
            TextComparatorArg::Number(v) => Ok(v.to_string()),
            TextComparatorArg::Decimal(v) => Ok(v.to_string()),
            TextComparatorArg::NumberArithmeticExpression(v) => v.get_text(),
            TextComparatorArg::DecimalArithmeticExpression(v) => v.get_text(),
            TextComparatorArg::NumberComparatorExpression(v) => v.get_text(),
            TextComparatorArg::DecimalComparatorExpression(v) => v.get_text(),
            TextComparatorArg::LogicalBinaryExpression(v) => v.get_text(),
            TextComparatorArg::LogicalUnaryExpression(v) => v.get_text(),
        };
        let arg1: Result<String, CustomError> = match &args.1 {
            TextComparatorArg::Number(v) => Ok(v.to_string()),
            TextComparatorArg::Decimal(v) => Ok(v.to_string()),
            TextComparatorArg::NumberArithmeticExpression(v) => v.get_text(),
            TextComparatorArg::DecimalArithmeticExpression(v) => v.get_text(),
            TextComparatorArg::NumberComparatorExpression(v) => v.get_text(),
            TextComparatorArg::DecimalComparatorExpression(v) => v.get_text(),
            TextComparatorArg::LogicalBinaryExpression(v) => v.get_text(),
            TextComparatorArg::LogicalUnaryExpression(v) => v.get_text(),
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
                        TextComparatorArg::NumberArithmeticExpression(v) => v.get_text(),
                        TextComparatorArg::DecimalArithmeticExpression(v) => v.get_text(),
                        TextComparatorArg::NumberComparatorExpression(v) => v.get_text(),
                        TextComparatorArg::DecimalComparatorExpression(v) => v.get_text(),
                        TextComparatorArg::LogicalBinaryExpression(v) => v.get_text(),
                        TextComparatorArg::LogicalUnaryExpression(v) => v.get_text(),
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
                            (Ok(_), Err(e)) => Err(e.clone()),
                            (Err(e), Ok(_)) => Err(e.clone()),
                            (Err(e), Err(_)) => Err(e.clone()),
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

#[derive(Debug)]
enum LogicalOperatorArg {
    Boolean(bool),
    NumberComparatorExpression(NumberComparatorExpression),
    DecimalComparatorExpression(DecimalComparatorExpression),
    TextComparatorExpression(TextComparatorExpression), // Expression(ArithmeticControlFlowExpression),
}

// BINARY LOGICAL

#[derive(Debug)]
enum LogicalBinaryOperator {
    And,
    Or,
}

#[derive(Debug)]
enum LogicalBinaryExpression {
    And(
        Box<(
            LogicalOperatorArg,
            LogicalOperatorArg,
            Vec<LogicalOperatorArg>,
        )>,
    ),
    Or(
        Box<(
            LogicalOperatorArg,
            LogicalOperatorArg,
            Vec<LogicalOperatorArg>,
        )>,
    ),
}

impl LogicalBinaryExpression {
    fn get_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: LogicalResultType) -> Result<LogicalResult, CustomError> {
        let (args, operator) = match self {
            LogicalBinaryExpression::And(v) => (v, LogicalBinaryOperator::And),
            LogicalBinaryExpression::Or(v) => (v, LogicalBinaryOperator::Or),
        };
        let arg0: Result<bool, CustomError> = match &args.0 {
            LogicalOperatorArg::Boolean(v) => Ok(*v),
            LogicalOperatorArg::NumberComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::DecimalComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::TextComparatorExpression(v) => v.get_boolean(),
        };
        let arg1: Result<bool, CustomError> = match &args.1 {
            LogicalOperatorArg::Boolean(v) => Ok(*v),
            LogicalOperatorArg::NumberComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::DecimalComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::TextComparatorExpression(v) => v.get_boolean(),
        };
        let init: Result<bool, CustomError> = match (arg0, arg1) {
            (Ok(v), Ok(v1)) => match operator {
                LogicalBinaryOperator::And => Ok(v && v1),
                LogicalBinaryOperator::Or => Ok(v || v1),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        };
        match args.2.len() == 0 {
            true => match init {
                Ok(v) => match result_type {
                    LogicalResultType::Boolean => Ok(LogicalResult::Boolean(v)),
                    LogicalResultType::Text => Ok(LogicalResult::Text(v.to_string())),
                },
                Err(e) => Err(e),
            },
            false => {
                let evaluated_args: Vec<Result<bool, CustomError>> = std::iter::once(&args.1)
                    .chain(&args.2)
                    .map(|val| match val {
                        LogicalOperatorArg::Boolean(v) => Ok(*v),
                        LogicalOperatorArg::NumberComparatorExpression(v) => v.get_boolean(),
                        LogicalOperatorArg::DecimalComparatorExpression(v) => v.get_boolean(),
                        LogicalOperatorArg::TextComparatorExpression(v) => v.get_boolean(),
                    })
                    .collect();
                let result: Result<bool, CustomError> =
                    evaluated_args.iter().fold(init, |acc, val| match &acc {
                        Ok(v) => match val {
                            Ok(v1) => match operator {
                                LogicalBinaryOperator::And => Ok(*v && *v1),
                                LogicalBinaryOperator::Or => Ok(*v || *v1),
                            },
                            Err(e) => Err(e.clone()),
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
        }
    }
}

// UNARY LOGICAL

#[derive(Debug)]
enum LogicalUnaryOperator {
    Not,
}

#[derive(Debug)]
enum LogicalUnaryExpression {
    Not(Box<LogicalOperatorArg>),
}

impl LogicalUnaryExpression {
    fn get_boolean(&self) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_text(&self) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn eval(&self, result_type: LogicalResultType) -> Result<LogicalResult, CustomError> {
        let (args, operator) = match self {
            LogicalUnaryExpression::Not(v) => (v, LogicalUnaryOperator::Not),
        };
        let result: Result<bool, CustomError> = match args.as_ref() {
            LogicalOperatorArg::Boolean(v) => Ok(*v),
            LogicalOperatorArg::NumberComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::DecimalComparatorExpression(v) => v.get_boolean(),
            LogicalOperatorArg::TextComparatorExpression(v) => v.get_boolean(),
        };
        match result {
            Ok(v) => match result_type {
                LogicalResultType::Boolean => Ok(LogicalResult::Boolean(v)),
                LogicalResultType::Text => Ok(LogicalResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

// impl LispExpression {
// fn eval(
//     result_type: LispExpressionResultType,
//     expr: &LispExpression,
// ) -> Result<LispExpressionResult, CustomError> {
// match expr {
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
// }

// And diesel stuff
fn main() {
    // let mut book_reviews = HashMap::new();
    // book_reviews.insert(
    //     "Adventures of Huckleberry Finn".to_string(),
    //     "My favorite book.".to_string(),
    // );
    let expr1 = DecimalArithmeticExpression::Add(Box::new((
        DecimalArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
        vec![DecimalArithmeticArg::Decimal(
            BigDecimal::from_i32(4).unwrap(),
        )],
    )));
    let expr2 = DecimalArithmeticExpression::Multiply(Box::new((
        DecimalArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
        vec![
            DecimalArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
            DecimalArithmeticArg::DecimalArithmeticExpression(expr1),
        ],
    )));

    println!("{:?}", expr2.get_decimal().unwrap());

    let expr3 = NumberComparatorExpression::GreaterThanEquals(Box::new((
        NumberComparatorArg::Number(12),
        NumberComparatorArg::Number(22),
        vec![NumberComparatorArg::Number(22)],
    )));

    let expr4 = DecimalComparatorExpression::GreaterThanEquals(Box::new((
        DecimalComparatorArg::Decimal(BigDecimal::from(2)),
        DecimalComparatorArg::Decimal(BigDecimal::from(3)),
        vec![DecimalComparatorArg::Decimal(BigDecimal::from_str("3.3").unwrap()),],
    )));

    println!("{:?}", &expr3.get_boolean().unwrap());
    println!("{:?}", &expr4.get_boolean().unwrap());
}

// #[cfg(test)]
// mod lisp_tests {
//     use super::*;

//     #[test]
//     fn calculate() {
//         let expr1: LispExpression = LispExpression::Add {
//             types: (ArithmeticArgType::Number, vec![]),
//             args: Box::new((
//                 ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
//                 vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
//             )),
//         };
//         let expr2: LispExpression = LispExpression::Add {
//             types: (ArithmeticArgType::Number, vec![]),
//             args: Box::new((
//                 ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
//                 vec![
//                     ArithmeticArg::Decimal(BigDecimal::from_i32(13).unwrap()),
//                     ArithmeticArg::Expression(expr1),
//                 ],
//             )),
//         };
//         assert_eq!(LispExpression::get_number(&expr2).unwrap(), 32);
//     }
// }
