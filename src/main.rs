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
enum ArithmeticArg {
    Number(i32),
    Decimal(BigDecimal),
    Expression(LispExpression),
}

#[derive(Debug)]
enum ArithmeticOperator {
    Add,
    Multiply,
    Subtract,
    Divide,
    Modulus,
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
    Expression(LispExpression),
}

#[derive(Debug)]
enum ComparatorOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
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
    Modulus {
        result_type: ArithmeticResultType,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
    },
    Equals {
        result_type: ComparatorResultType,
        types: ComparatorArgType,
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    GreaterThan {
        result_type: ComparatorResultType,
        types: ComparatorArgType,
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    LessThan {
        result_type: ComparatorResultType,
        types: ComparatorArgType,
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    GreaterThanEquals {
        result_type: ComparatorResultType,
        types: ComparatorArgType,
        args: Box<(ComparatorArg, ComparatorArg, Vec<ComparatorArg>)>,
    },
    LessThanEquals {
        result_type: ComparatorResultType,
        types: ComparatorArgType,
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

// Should probably use references here, &BigDecimal and &str
// And deal with problems as they come
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
    fn eval(expr: &LispExpression) -> Result<LispExpressionResult, CustomError> {
        match expr {
            LispExpression::Add {
                result_type,
                types,
                args,
            } => match Self::calculate(result_type, types, args, ArithmeticOperator::Add) {
                Ok(v) => Ok(LispExpressionResult::ArithmeticResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::Multiply {
                result_type,
                types,
                args,
            } => match Self::calculate(result_type, types, args, ArithmeticOperator::Multiply) {
                Ok(v) => Ok(LispExpressionResult::ArithmeticResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::Subtract {
                result_type,
                types,
                args,
            } => match Self::calculate(result_type, types, args, ArithmeticOperator::Subtract) {
                Ok(v) => Ok(LispExpressionResult::ArithmeticResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::Divide {
                result_type,
                types,
                args,
            } => match Self::calculate(result_type, types, args, ArithmeticOperator::Divide) {
                Ok(v) => Ok(LispExpressionResult::ArithmeticResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::Modulus {
                result_type,
                types,
                args,
            } => match Self::calculate(result_type, types, args, ArithmeticOperator::Modulus) {
                Ok(v) => Ok(LispExpressionResult::ArithmeticResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::Equals {
                result_type,
                types,
                args,
            } => match Self::compare(result_type, types, args, ComparatorOperator::Equals) {
                Ok(v) => Ok(LispExpressionResult::ComparatorResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::GreaterThan {
                result_type,
                types,
                args,
            } => match Self::compare(result_type, types, args, ComparatorOperator::GreaterThan) {
                Ok(v) => Ok(LispExpressionResult::ComparatorResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::LessThan {
                result_type,
                types,
                args,
            } => match Self::compare(result_type, types, args, ComparatorOperator::LessThan) {
                Ok(v) => Ok(LispExpressionResult::ComparatorResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::GreaterThanEquals {
                result_type,
                types,
                args,
            } => match Self::compare(
                result_type,
                types,
                args,
                ComparatorOperator::GreaterThanEquals,
            ) {
                Ok(v) => Ok(LispExpressionResult::ComparatorResult(v)),
                Err(e) => Err(e),
            },
            LispExpression::LessThanEquals {
                result_type,
                types,
                args,
            } => {
                match Self::compare(result_type, types, args, ComparatorOperator::LessThanEquals) {
                    Ok(v) => Ok(LispExpressionResult::ComparatorResult(v)),
                    Err(e) => Err(e),
                }
            }
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

    // Write test cases and build some audio visual documentation
    // Here, the return type should be ArithmeticResult since it reduces the output subspace
    fn calculate(
        result_type: &ArithmeticResultType,
        types: &(ArithmeticArgType, Vec<ArithmeticArgType>),
        args: &Box<(ArithmeticArg, Vec<ArithmeticArg>)>,
        operator: ArithmeticOperator,
    ) -> Result<ArithmeticResult, CustomError> {
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
                let mut temp: BigDecimal = match BigDecimal::from_i32(1) {
                    Some(v) => v,
                    None => return Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                };
                let init: Result<BigDecimal, CustomError> = match &args.0 {
                    ArithmeticArg::Number(v) => match BigDecimal::from_i32(*v) {
                        Some(v1) => {
                            temp *= v1;
                            Ok(temp)
                        }
                        None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                    },
                    ArithmeticArg::Decimal(v1) => {
                        temp *= v1;
                        Ok(temp)
                    }
                    ArithmeticArg::Expression(v1) => match LispExpression::get_decimal(&v1) {
                        Ok(v2) => {
                            temp *= v2;
                            Ok(temp)
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
                                    Some(v1) => Ok(match operator {
                                        ArithmeticOperator::Add => v + v1,
                                        ArithmeticOperator::Multiply => v * v1,
                                        ArithmeticOperator::Subtract => v - v1,
                                        ArithmeticOperator::Divide => v / v1,
                                        ArithmeticOperator::Modulus => v % v1,
                                    }),
                                    None => {
                                        return Err(CustomError::Message(
                                            UNEXPECTED_ERROR.to_string(),
                                        ))
                                    }
                                },
                                ArithmeticArg::Decimal(v1) => Ok(match operator {
                                    ArithmeticOperator::Add => v + v1,
                                    ArithmeticOperator::Multiply => v * v1,
                                    ArithmeticOperator::Subtract => v - v1,
                                    ArithmeticOperator::Divide => v / v1,
                                    ArithmeticOperator::Modulus => v % v1,
                                }),
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_decimal(v1) {
                                        Ok(v2) => Ok(match operator {
                                            ArithmeticOperator::Add => v + v2,
                                            ArithmeticOperator::Multiply => v * v2,
                                            ArithmeticOperator::Subtract => v - v2,
                                            ArithmeticOperator::Divide => v / v2,
                                            ArithmeticOperator::Modulus => v % v2,
                                        }),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                            ArithmeticArgType::Decimal => match val.0 {
                                ArithmeticArg::Number(v1) => match BigDecimal::from_i32(*v1) {
                                    Some(v1) => Ok(match operator {
                                        ArithmeticOperator::Add => v + v1,
                                        ArithmeticOperator::Multiply => v * v1,
                                        ArithmeticOperator::Subtract => v - v1,
                                        ArithmeticOperator::Divide => v / v1,
                                        ArithmeticOperator::Modulus => v % v1,
                                    }),
                                    None => {
                                        return Err(CustomError::Message(
                                            UNEXPECTED_ERROR.to_string(),
                                        ))
                                    }
                                },
                                ArithmeticArg::Decimal(v1) => Ok(match operator {
                                    ArithmeticOperator::Add => v + v1,
                                    ArithmeticOperator::Multiply => v * v1,
                                    ArithmeticOperator::Subtract => v - v1,
                                    ArithmeticOperator::Divide => v / v1,
                                    ArithmeticOperator::Modulus => v % v1,
                                }),
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_decimal(v1) {
                                        Ok(v2) => Ok(match operator {
                                            ArithmeticOperator::Add => v + v2,
                                            ArithmeticOperator::Multiply => v * v2,
                                            ArithmeticOperator::Subtract => v - v2,
                                            ArithmeticOperator::Divide => v / v2,
                                            ArithmeticOperator::Modulus => v % v2,
                                        }),
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
                                ArithmeticArg::Number(v1) => Ok(match operator {
                                    ArithmeticOperator::Add => v + *v1,
                                    ArithmeticOperator::Multiply => v * *v1,
                                    ArithmeticOperator::Subtract => v - *v1,
                                    ArithmeticOperator::Divide => v / *v1,
                                    ArithmeticOperator::Modulus => v % *v1,
                                }),
                                ArithmeticArg::Decimal(v1) => match v1.to_i32() {
                                    Some(v2) => Ok(match operator {
                                        ArithmeticOperator::Add => v + v2,
                                        ArithmeticOperator::Multiply => v * v2,
                                        ArithmeticOperator::Subtract => v - v2,
                                        ArithmeticOperator::Divide => v / v2,
                                        ArithmeticOperator::Modulus => v % v2,
                                    }),
                                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_number(v1) {
                                        Ok(v2) => Ok(match operator {
                                            ArithmeticOperator::Add => v + v2,
                                            ArithmeticOperator::Multiply => v * v2,
                                            ArithmeticOperator::Subtract => v - v2,
                                            ArithmeticOperator::Divide => v / v2,
                                            ArithmeticOperator::Modulus => v % v2,
                                        }),
                                        Err(e) => Err(e),
                                    }
                                }
                            },
                            ArithmeticArgType::Decimal => match val.0 {
                                ArithmeticArg::Number(v1) => Ok(match operator {
                                    ArithmeticOperator::Add => v + *v1,
                                    ArithmeticOperator::Multiply => v * *v1,
                                    ArithmeticOperator::Subtract => v - *v1,
                                    ArithmeticOperator::Divide => v / *v1,
                                    ArithmeticOperator::Modulus => v % *v1,
                                }),
                                ArithmeticArg::Decimal(v1) => match v1.to_i32() {
                                    Some(v2) => Ok(match operator {
                                        ArithmeticOperator::Add => v + v2,
                                        ArithmeticOperator::Multiply => v * v2,
                                        ArithmeticOperator::Subtract => v - v2,
                                        ArithmeticOperator::Divide => v / v2,
                                        ArithmeticOperator::Modulus => v % v2,
                                    }),
                                    None => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                ArithmeticArg::Expression(v1) => {
                                    match LispExpression::get_number(v1) {
                                        Ok(v2) => Ok(match operator {
                                            ArithmeticOperator::Add => v + v2,
                                            ArithmeticOperator::Multiply => v * v2,
                                            ArithmeticOperator::Subtract => v - v2,
                                            ArithmeticOperator::Divide => v / v2,
                                            ArithmeticOperator::Modulus => v % v2,
                                        }),
                                        Err(e) => Err(e),
                                    }
                                }
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
    }

    fn compare(
        result_type: &ComparatorResultType,
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
                                ComparatorArg::Number(v) => {
                                    Ok(LispExpressionResult::ArithmeticResult(
                                        ArithmeticResult::Number(*v),
                                    ))
                                }
                                ComparatorArg::Expression(v) => match LispExpression::get_number(v)
                                {
                                    Ok(v1) => Ok(LispExpressionResult::ArithmeticResult(
                                        ArithmeticResult::Number(v1),
                                    )),
                                    Err(e) => Err(e),
                                },
                                _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                            })
                            .collect();
                    let result: Result<bool, CustomError> = evaluated_args
                        .iter()
                        .zip(&evaluated_args[1..])
                        .fold(init, |acc, val| match acc {
                            Ok(v) => match v {
                                true => match val {
                                    (Ok(v1), Ok(v2)) => match v1 {
                                        LispExpressionResult::ArithmeticResult(
                                            ArithmeticResult::Number(v3),
                                        ) => match v2 {
                                            LispExpressionResult::ArithmeticResult(
                                                ArithmeticResult::Number(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            LispExpressionResult::ControlFlowResult(
                                                ControlFlowResult::Number(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            _ => Err(CustomError::Message(
                                                UNEXPECTED_ERROR.to_string(),
                                            )),
                                        },
                                        LispExpressionResult::ControlFlowResult(
                                            ControlFlowResult::Number(v3),
                                        ) => match v2 {
                                            LispExpressionResult::ArithmeticResult(
                                                ArithmeticResult::Number(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            LispExpressionResult::ControlFlowResult(
                                                ControlFlowResult::Number(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            _ => Err(CustomError::Message(
                                                UNEXPECTED_ERROR.to_string(),
                                            )),
                                        },
                                        _ => {
                                            Err(CustomError::Message(UNEXPECTED_ERROR.to_string()))
                                        }
                                    },
                                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                false => Ok(false),
                            },
                            Err(e) => Err(e),
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
                            ComparatorArg::Expression(v2) => match LispExpression::get_decimal(v2) {
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
                                ComparatorArg::Decimal(v) => {
                                    Ok(LispExpressionResult::ArithmeticResult(
                                        ArithmeticResult::Decimal(v),
                                    ))
                                }
                                ComparatorArg::Expression(v) => match LispExpression::get_decimal(v)
                                {
                                    Ok(v1) => Ok(LispExpressionResult::ArithmeticResult(
                                        ArithmeticResult::Decimal(v1),
                                    )),
                                    Err(e) => Err(e),
                                },
                                _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                            })
                            .collect();
                    let result: Result<bool, CustomError> = evaluated_args
                        .iter()
                        .zip(&evaluated_args[1..])
                        .fold(init, |acc, val| match acc {
                            Ok(v) => match v {
                                true => match val {
                                    (Ok(v1), Ok(v2)) => match v1 {
                                        LispExpressionResult::ArithmeticResult(
                                            ArithmeticResult::Decimal(v3),
                                        ) => match v2 {
                                            LispExpressionResult::ArithmeticResult(
                                                ArithmeticResult::Decimal(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            LispExpressionResult::ControlFlowResult(
                                                ControlFlowResult::Decimal(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            _ => Err(CustomError::Message(
                                                UNEXPECTED_ERROR.to_string(),
                                            )),
                                        },
                                        LispExpressionResult::ControlFlowResult(
                                            ControlFlowResult::Decimal(v3),
                                        ) => match v2 {
                                            LispExpressionResult::ArithmeticResult(
                                                ArithmeticResult::Decimal(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            LispExpressionResult::ControlFlowResult(
                                                ControlFlowResult::Decimal(v4),
                                            ) => match operator {
                                                ComparatorOperator::Equals => Ok(v3 == v4),
                                                ComparatorOperator::GreaterThan => Ok(v3 < v4),
                                                ComparatorOperator::LessThan => Ok(v3 > v4),
                                                ComparatorOperator::GreaterThanEquals => {
                                                    Ok(v3 <= v4)
                                                }
                                                ComparatorOperator::LessThanEquals => Ok(v3 >= v4),
                                            },
                                            _ => Err(CustomError::Message(
                                                UNEXPECTED_ERROR.to_string(),
                                            )),
                                        },
                                        _ => {
                                            Err(CustomError::Message(UNEXPECTED_ERROR.to_string()))
                                        }
                                    },
                                    _ => Err(CustomError::Message(UNEXPECTED_ERROR.to_string())),
                                },
                                false => Ok(false),
                            },
                            Err(e) => Err(e),
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
            ComparatorArgType::Text => Ok(ComparatorResult::Boolean(true)),
        }
    }
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
        result_type: ArithmeticResultType::Number,
        types: (ArithmeticArgType::Number, vec![]),
        args: Box::new((
            ArithmeticArg::Decimal(BigDecimal::from_i32(3).unwrap()),
            vec![ArithmeticArg::Decimal(BigDecimal::from_i32(4).unwrap())],
        )),
    };
    let expr2: LispExpression = LispExpression::Multiply {
        result_type: ArithmeticResultType::Number,
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
        result_type: ComparatorResultType::Boolean,
        types: ComparatorArgType::Number,
        args: Box::new((
            ComparatorArg::Number(12),
            ComparatorArg::Number(22),
            vec![ComparatorArg::Number(22)],
        )),
    };
    println!("{:?}", LispExpression::get_boolean(&expr3).unwrap());
}

#[cfg(test)]
mod lisp_tests {
    use super::*;

    #[test]
    fn calculate() {
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
                    ArithmeticArg::Expression(expr1),
                ],
            )),
        };
        assert_eq!(LispExpression::get_number(&expr2).unwrap(), 32);
    }
}
