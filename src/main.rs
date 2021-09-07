/* Copyright (C) Logicarth (OPC) Private Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;

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
enum ArithmeticArg {
    Number(i32),
    Decimal(BigDecimal),
    // Expression(LispExpression),
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
enum ControlFlowArg {
    Boolean(bool, LispExpression, LispExpression),
    Expression(LispExpression, LispExpression, LispExpression),
}

// Note. In some places, tuples or slices could be used here instead of arrays
#[derive(Debug)]
enum LispExpression {
    Add {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Multiply {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Subtract {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Divide {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Power {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Modulus {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
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
        args: Box<ControlFlowArg>,
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

impl LispExpression {
    fn eval(expr: LispExpression) -> Result<LispExpressionResult, CustomError> {
        match expr {
            LispExpression::Add {
                result_type,
                types,
                args,
            } => Ok(Self::add(result_type, types, (args.0, args.1))),
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

    fn get_text(expr: LispExpression) -> Result<String, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Text(x)) => Ok(x),
            LispExpressionResult::ComparatorResult(ComparatorResult::Text(x)) => Ok(x),
            LispExpressionResult::LogicalResult(LogicalResult::Text(x)) => Ok(x),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Text(x)) => Ok(x),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_number(expr: LispExpression) -> Result<i32, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Number(x)) => Ok(x),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Number(x)) => Ok(x),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_decimal(expr: LispExpression) -> Result<BigDecimal, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ArithmeticResult(ArithmeticResult::Decimal(x)) => Ok(x),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Decimal(x)) => Ok(x),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn get_boolean(expr: LispExpression) -> Result<bool, CustomError> {
        match Self::eval(expr)? {
            LispExpressionResult::ComparatorResult(ComparatorResult::Boolean(x)) => Ok(x),
            LispExpressionResult::LogicalResult(LogicalResult::Boolean(x)) => Ok(x),
            LispExpressionResult::ControlFlowResult(ControlFlowResult::Boolean(x)) => Ok(x),
            _ => Err(CustomError::Message("Unexpected Result".to_string())),
        }
    }

    fn add(
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    ) -> LispExpressionResult {
        let last_type = match types.1.is_empty() {
            true => &types.0,
            false => types.1.last().unwrap(),
        };
        let contains_decimal_type: bool = if let ArithmeticArgType::Decimal = types.0 {
            true
        } else {
            types.1.iter().fold(false, |acc, val| {
                acc || if let ArithmeticArgType::Decimal = val {
                    true
                } else {
                    false
                }
            })
        };
        match contains_decimal_type {
            true => {
                let init: BigDecimal = match types.0 {
                    ArithmeticArgType::Number => match args.0 {
                        ArithmeticArg::Number(x) => BigDecimal::from_i32(x).unwrap(),
                        ArithmeticArg::Decimal(x) => x,
                    },
                    ArithmeticArgType::Decimal => match args.0 {
                        ArithmeticArg::Number(x) => BigDecimal::from_i32(x).unwrap(),
                        ArithmeticArg::Decimal(x) => x,
                    },
                };
                let result: BigDecimal = args
                    .1
                    .iter()
                    .zip(types.1.iter().chain(repeat(last_type)))
                    .fold(init, |acc, val| match val.1 {
                        ArithmeticArgType::Number => match val.0 {
                            ArithmeticArg::Number(x) => acc + BigDecimal::from_i32(*x).unwrap(),
                            ArithmeticArg::Decimal(x) => acc + x,
                        },
                        ArithmeticArgType::Decimal => match val.0 {
                            ArithmeticArg::Number(x) => acc + BigDecimal::from_i32(*x).unwrap(),
                            ArithmeticArg::Decimal(x) => acc + x,
                        },
                    });
                LispExpressionResult::ArithmeticResult(match result_type {
                    ArithmeticResultType::Number => {
                        ArithmeticResult::Number(result.to_i32().unwrap())
                    }
                    ArithmeticResultType::Decimal => ArithmeticResult::Decimal(result),
                    ArithmeticResultType::Text => ArithmeticResult::Text(result.to_string()),
                })
            }
            false => {
                let init: i32 = match types.0 {
                    ArithmeticArgType::Number => match args.0 {
                        ArithmeticArg::Number(x) => x,
                        ArithmeticArg::Decimal(x) => x.to_i32().unwrap(),
                    },
                    ArithmeticArgType::Decimal => match args.0 {
                        ArithmeticArg::Number(x) => x,
                        ArithmeticArg::Decimal(x) => x.to_i32().unwrap(),
                    },
                };
                let result: i32 = args
                    .1
                    .iter()
                    .zip(types.1.iter().chain(repeat(last_type)))
                    .fold(init, |acc, val| {
                        acc + match val.1 {
                            ArithmeticArgType::Number => match val.0 {
                                ArithmeticArg::Number(x) => *x,
                                ArithmeticArg::Decimal(x) => x.to_i32().unwrap(),
                            },
                            ArithmeticArgType::Decimal => match val.0 {
                                ArithmeticArg::Number(x) => *x,
                                ArithmeticArg::Decimal(x) => x.to_i32().unwrap(),
                            },
                        }
                    });
                LispExpressionResult::ArithmeticResult(match result_type {
                    ArithmeticResultType::Number => ArithmeticResult::Number(result),
                    ArithmeticResultType::Decimal => {
                        ArithmeticResult::Decimal(BigDecimal::from_i32(result).unwrap())
                    }
                    ArithmeticResultType::Text => ArithmeticResult::Text(result.to_string()),
                })
            }
        }
    }
}

fn main() {
    let x: LispExpression = LispExpression::Add {
        result_type: ArithmeticResultType::Number,
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
            vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())]
        )),
    };
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    println!("{:?}", x);
    println!("{:?}", LispExpression::eval(x).unwrap());
    println!("{:?}", book_reviews);
    let input = "0.8";
    let dec = BigDecimal::from_str(&input).unwrap();
    let float = f32::from_str(&input).unwrap();

    println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);

    println!(
        "{:?}",
        LispExpression::add(
            ArithmeticResultType::Number,
            (ArithmeticArgType::Number, vec![]),
            (
                ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
                vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())]
            )
        )
    );

    let x: LispExpression = LispExpression::Add {
        result_type: ArithmeticResultType::Number,
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(38).unwrap()),
            vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())]
        )),
    };

    println!("{:?}", LispExpression::get_number(x).unwrap())
}
