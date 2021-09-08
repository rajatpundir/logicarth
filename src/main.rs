/* Copyright (C) Logicarth (OPC) Private Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::iter::repeat;

const UNEXPECTED_ERROR: &str = "Unexpected Error";

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

#[derive(Debug, Clone)]
enum ArithmeticArgType {
    Number,
    Decimal,
}

#[derive(Debug)]
enum ArithmeticArg<'a> {
    Number(i32),
    Decimal(BigDecimal),
    Expression(&'a LispExpression<'a>),
}

// Comparator Ops

#[derive(Debug)]
enum ComparatorResultType {
    Boolean,
    Text,
}

#[derive(Debug)]
enum ComparatorArgType {
    Number,
    Decimal,
    Text,
}

#[derive(Debug)]
enum ComparatorArg {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
    // Expression(LispExpression),
}

// Logical Ops

#[derive(Debug)]
enum LogicalResultType {
    Boolean,
    Text,
}

#[derive(Debug)]
enum LogicalArgType {
    Boolean,
}

#[derive(Debug)]
enum LogicalArg {
    Boolean(bool),
    // Expression(LispExpression),
}

// Control Flow Ops

#[derive(Debug)]
enum ControlFlowResultType {
    Number,
    Decimal,
    Boolean,
    Text,
}

#[derive(Debug)]
enum ControlFlowArgType {
    Number,
    Decimal,
    Boolean,
    Text,
}

#[derive(Debug)]
enum ControlFlowArg<'a> {
    Boolean(bool, LispExpression<'a>, LispExpression<'a>),
    Expression(LispExpression<'a>, LispExpression<'a>, LispExpression<'a>),
}

// Note. In some places, tuples or slices could be used here instead of arrays
#[derive(Debug)]
enum LispExpression<'a> {
    Add {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Multiply {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Subtract {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Divide {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Power {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Modulus {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg<'a>, Vec<ArithmeticArg<'a>>)>,
    },
    Equals {
        result_type: ComparatorResultType,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    GreaterThan {
        result_type: ComparatorResultType,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    LessThan {
        result_type: ComparatorResultType,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    GreaterThanEquals {
        result_type: ComparatorResultType,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    LessThanEquals {
        result_type: ComparatorResultType,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    And {
        result_type: LogicalResultType,
        args: Box<(LogicalArg, LogicalArg, Vec<LogicalArg>)>,
    },
    Or {
        result_type: LogicalResultType,
        args: Box<(LogicalArg, LogicalArg, Vec<LogicalArg>)>,
    },
    Not {
        result_type: LogicalResultType,
        args: Box<LogicalArg>,
    },
    If {
        result_type: ControlFlowResultType,
        types: (ControlFlowArgType, Vec<ControlFlowArgType>),
        args: Box<ControlFlowArg<'a>>,
    },
}

#[derive(Debug)]
enum ArithmeticResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

#[derive(Debug)]
enum ComparatorResult {
    Boolean(bool),
    Text(String),
}

#[derive(Debug)]
enum LogicalResult {
    Boolean(bool),
    Text(String),
}

#[derive(Debug)]
enum ControlFlowResult {
    Number(i32),
    Decimal(BigDecimal),
    Boolean(bool),
    Text(String),
}

#[derive(Debug)]
enum LispExpressionResult {
    ArithmeticResult(ArithmeticResult),
    ComparatorResult(ComparatorResult),
    LogicalResult(LogicalResult),
    ControlFlowResult(ControlFlowResult),
}

impl LispExpression<'static> {
    fn eval(expr: &LispExpression) -> Result<LispExpressionResult, CustomError> {
        match expr {
            LispExpression::Add {
                result_type,
                types,
                args,
            } => Self::add(result_type, types, args),
            LispExpression::Multiply { .. } => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
            LispExpression::Subtract { .. } => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
            LispExpression::Divide { .. } => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
            LispExpression::Power { .. } => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
            LispExpression::Modulus { .. } => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
            LispExpression::Equals { .. } => Ok(LispExpressionResult::ComparatorResult(
                ComparatorResult::Boolean(true),
            )),
            LispExpression::GreaterThan { .. } => Ok(LispExpressionResult::ComparatorResult(
                ComparatorResult::Boolean(true),
            )),
            LispExpression::LessThan { .. } => Ok(LispExpressionResult::ComparatorResult(
                ComparatorResult::Boolean(true),
            )),
            LispExpression::GreaterThanEquals { .. } => Ok(LispExpressionResult::ComparatorResult(
                ComparatorResult::Boolean(true),
            )),
            LispExpression::LessThanEquals { .. } => Ok(LispExpressionResult::ComparatorResult(
                ComparatorResult::Boolean(true),
            )),
            LispExpression::And { .. } => Ok(LispExpressionResult::LogicalResult(
                LogicalResult::Boolean(true),
            )),
            LispExpression::Or { .. } => Ok(LispExpressionResult::LogicalResult(
                LogicalResult::Boolean(true),
            )),
            LispExpression::Not { .. } => Ok(LispExpressionResult::LogicalResult(
                LogicalResult::Boolean(true),
            )),
            _ => Ok(LispExpressionResult::ArithmeticResult(
                ArithmeticResult::Number(2),
            )),
        }
    }

    fn get_text(expr: &LispExpression) -> Result<String, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Text(v)) => Ok(v),
            LispExpressionResult::ComparatorResult(ComparatorResult::Text(v)) => Ok(v),
            LispExpressionResult::LogicalResult(LogicalResult::Text(v)) => Ok(v),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Text(v)) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_number(expr: &LispExpression) -> Result<i32, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Number(v)) => Ok(v),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Number(v)) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_decimal(expr: &LispExpression) -> Result<BigDecimal, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Decimal(v)) => Ok(v),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Decimal(v)) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_boolean(expr: &LispExpression) -> Result<bool, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ComparatorResult(ComparatorResult::Boolean(v)) => Ok(v),
            LispExpressionResult::LogicalResult(LogicalResult::Boolean(v)) => Ok(v),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Boolean(v)) => Ok(v),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    // Write some docs(to make understanding trivial at first glance) and test cases
    // Integrate expressions
    fn add(
        result_type: &ArithmeticResultType,
        types: &(ArithmeticArgType, Vec<ArithmeticArgType>),
        args: &Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    ) -> Result<LispExpressionResult, CustomError> {
        let last_type = match types.1.is_empty() {
            true => &types.0,
            false => match types.1.last() {
                Some(v) => v,
                None => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
            },
        };
        let contains_decimal_type: bool = match types.0 {
            ArithmeticArgType::Decimal => true,
            _ => types.1.iter().any(|val| match val {
                ArithmeticArgType::Decimal => true,
                _ => false,
            }),
        };
        match contains_decimal_type {
            true => {
                let mut init_val: BigDecimal = match BigDecimal::from_i32(0) {
                    Some(v) => v,
                    None => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                };
                let init: Result<BigDecimal, CustomError> = match &args.0 {
                    ArithmeticArg::Number(v) => match BigDecimal::from_i32(*v) {
                        Some(v1) => {
                            init_val += v1;
                            Ok(init_val)
                        }
                        None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ArithmeticArg::Decimal(v1) => {
                        init_val += v1;
                        Ok(init_val)
                    }
                    ArithmeticArg::Expression(v1) => match LispExpression::get_decimal(&v1) {
                        Ok(v2) => {
                            init_val += v2;
                            Ok(init_val)
                        }
                        Err(e) => Err(e),
                    },
                };
                let result: Result<BigDecimal, CustomError> = args
                    .1
                    .iter()
                    .zip(types.1.iter().chain(repeat(last_type)))
                    .fold(init, |acc, val| match acc {
                        Ok(v) => match val.1 {
                            ArithmeticArgType::Number => match val.0 {
                                ArithmeticArg::Number(v1) => match BigDecimal::from_i32(*v1) {
                                    Some(v1) => Ok(v + v1),
                                    None => {
                                        return Err(CustomError::Message(
                                            UNEXPECTED_ERROR.to_string(),
                                        ))
                                    }
                                },
                                ArithmeticArg::Decimal(v1) => Ok(v + v1),
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_decimal(v1) {
                                        Ok(v2) => Ok(v + v2),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                            ArithmeticArgType::Decimal => match val.0 {
                                ArithmeticArg::Number(v1) => match BigDecimal::from_i32(*v1) {
                                    Some(v1) => Ok(v + v1),
                                    None => {
                                        return Err(CustomError::Message(
                                            UNEXPECTED_ERROR.to_string(),
                                        ))
                                    }
                                },
                                ArithmeticArg::Decimal(v1) => Ok(v + v1),
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_decimal(v1) {
                                        Ok(v2) => Ok(v + v2),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                        },
                        Err(_) => acc,
                    });
                match result_type {
                    ArithmeticResultType::Number => match result {
                        Ok(v) => match v.to_i32() {
                            Some(v1) => Ok(LispExpressionResult::ArithmeticResult(
                                ArithmeticResult::Number(v1),
                            )),
                            None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        Err(e) => Err(e),
                    },
                    ArithmeticResultType::Decimal => match result {
                        Ok(v) => Ok(LispExpressionResult::ArithmeticResult(
                            ArithmeticResult::Decimal(v),
                        )),
                        Err(e) => Err(e),
                    },
                    ArithmeticResultType::Text => match result {
                        Ok(v) => Ok(LispExpressionResult::ArithmeticResult(
                            ArithmeticResult::Text(v.to_string()),
                        )),
                        Err(e) => Err(e),
                    },
                }
            }
            false => {
                let init: Result<i32, CustomError> = match &args.0 {
                    ArithmeticArg::Number(v) => Ok(*v),
                    ArithmeticArg::Decimal(v) => match v.to_i32() {
                        Some(v1) => Ok(v1),
                        None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ArithmeticArg::Expression(v1) => match LispExpression::get_number(v1) {
                        Ok(v2) => Ok(v2),
                        Err(e) => Err(e),
                    },
                };
                let result: Result<i32, CustomError> = args
                    .1
                    .iter()
                    .zip(types.1.iter().chain(repeat(last_type)))
                    .fold(init, |acc, val| match acc {
                        Ok(v) => match val.1 {
                            ArithmeticArgType::Number => match val.0 {
                                ArithmeticArg::Number(v1) => Ok(v + *v1),
                                ArithmeticArg::Decimal(v1) => match v1.to_i32() {
                                    Some(v2) => Ok(v + v2),
                                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_number(v1) {
                                        Ok(v2) => Ok(v + v2),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                            ArithmeticArgType::Decimal => match val.0 {
                                ArithmeticArg::Number(v1) => Ok(v + *v1),
                                ArithmeticArg::Decimal(v1) => match v1.to_i32() {
                                    Some(v2) => Ok(v + v2),
                                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_number(v1) {
                                        Ok(v2) => Ok(v + v2),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                        },
                        Err(_) => acc,
                    });
                match result_type {
                    ArithmeticResultType::Number => match result {
                        Ok(v) => Ok(LispExpressionResult::ArithmeticResult(
                            ArithmeticResult::Number(v),
                        )),
                        Err(e) => Err(e),
                    },
                    ArithmeticResultType::Decimal => match result {
                        Ok(v) => match BigDecimal::from_i32(v) {
                            Some(v1) => Ok(LispExpressionResult::ArithmeticResult(
                                ArithmeticResult::Decimal(v1),
                            )),
                            None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                        },
                        Err(e) => Err(e),
                    },
                    ArithmeticResultType::Text => match result {
                        Ok(v) => Ok(LispExpressionResult::ArithmeticResult(
                            ArithmeticResult::Text(v.to_string()),
                        )),
                        Err(e) => Err(e),
                    },
                }
            }
        }
    }
}

fn main() {
    let y: LispExpression = LispExpression::Add {
        result_type: ArithmeticResultType::Number,
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
            vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
        )),
    };
    // let mut book_reviews = HashMap::new();

    // // Review some books.
    // book_reviews.insert(
    //     "Adventures of Huckleberry Finn".to_string(),
    //     "My favorite book.".to_string(),
    // );
    // println!("{:?}", x);
    // println!("{:?}", LispExpression::eval(x).unwrap());
    // println!("{:?}", book_reviews);
    // let input = "0.8";
    // let dec = BigDecimal::from_str(&input).unwrap();
    // let float = f32::from_str(&input).unwrap();

    // println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);

    // println!(
    //     "{:?}",
    //     LispExpression::add(
    //         ArithmeticResultType::Number,
    //         (ArithmeticArgType::Number, vec![]),
    //         (
    //             ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
    //             vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())]
    //         )
    //     )
    // );

    let x: LispExpression = LispExpression::Add {
        result_type: ArithmeticResultType::Number,
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
            vec![
                ArithmeticArg::Decimal(BigDecimal::from_i32(13).unwrap()),
                ArithmeticArg::Expression(&y),
            ],
        )),
    };

    println!("{:?}", LispExpression::get_number(&x).unwrap());
}

#[cfg(test)]
mod lisp_tests {
    use super::*;

    #[test]
    fn add() {
        let expr1: LispExpression = LispExpression::Add {
            result_type: ArithmeticResultType::Number,
            types: (ArithmeticArgType::Number, vec![]),
            args: Box::new((
                ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
                vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
            )),
        };
        let expr2: LispExpression = LispExpression::Add {
            result_type: ArithmeticResultType::Number,
            types: (ArithmeticArgType::Number, vec![]),
            args: Box::new((
                ArithmeticArg::Decimal(BigDecimal::from_i32(12).unwrap()),
                vec![
                    ArithmeticArg::Decimal(BigDecimal::from_i32(13).unwrap()),
                    ArithmeticArg::Expression(&expr1),
                ],
            )),
        };
        assert_eq!(LispExpression::get_number(&expr2).unwrap(), 32);
    }
}
