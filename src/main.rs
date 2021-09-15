/* Copyright (C) Gokyun (OPC) Private Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

// Notes.
// 1. Write test cases
// 2. Add serde
// 3. Build some audio visual documentation
// 4. Add Diesel

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use core::fmt::Debug;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec;

#[derive(Debug)]
enum Language {
    English,
}

#[derive(Debug, Clone)]
enum Strings {
    UnexpectedError,
    MissingSymbol,
}

impl Strings {
    fn to_string(&self, lang: Language) -> String {
        let result: &str = match self {
            Strings::UnexpectedError => match lang {
                Language::English => "Unexpected Error",
            },
            Strings::MissingSymbol => match lang {
                Language::English => "Symbol not found",
            },
        };
        result.to_string()
    }
}

#[derive(Debug, Clone)]
enum CustomError {
    Message(Strings),
    Messages(HashMap<String, CustomError>),
}

// Symbols

enum Leaf {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
    Boolean(bool),
}

enum Symbol {
    Leaf(Leaf),
    Symbols(HashMap<String, Symbol>),
}

// Traits

trait ToNumber {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError>;
}

impl Debug for dyn ToNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_number(&HashMap::new()))
    }
}

impl ToNumber for i32 {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        Ok(*self)
    }
}

impl ToNumber for BigDecimal {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.to_i32() {
            Some(v) => Ok(v),
            None => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

trait ToDecimal {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError>;
}

impl Debug for dyn ToDecimal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_decimal(&HashMap::new()))
    }
}

impl ToDecimal for i32 {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match BigDecimal::from_i32(*self) {
            Some(v) => Ok(v),
            None => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for BigDecimal {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        Ok(self.clone())
    }
}

trait ToText {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError>;
}

impl Debug for dyn ToText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_text(&HashMap::new()))
    }
}

impl ToText for i32 {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }
}

impl ToText for BigDecimal {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }
}

impl ToText for String {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }
}

impl ToText for bool {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }
}

trait ToBoolean {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError>;
}

impl Debug for dyn ToBoolean {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_boolean(&HashMap::new()))
    }
}

impl ToBoolean for bool {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        Ok(*self)
    }
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
enum NumberArithmeticExpression {
    Add((Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    Multiply((Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    Subtract((Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    Divide((Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    Modulus((Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
}

impl NumberArithmeticExpression {
    fn eval(
        &self,
        result_type: ArithmeticResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            NumberArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            NumberArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Multiply),
            NumberArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Subtract),
            NumberArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Divide),
            NumberArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Modulus),
        };
        let init: Result<i32, CustomError> = args.0.get_number(symbols);
        let result: Result<i32, CustomError> = args.1.iter().fold(init, |acc, val| match &acc {
            Ok(v) => match val.get_number(symbols) {
                Ok(v1) => match operator {
                    ArithmeticOperator::Add => Ok(v + v1),
                    ArithmeticOperator::Multiply => Ok(v * v1),
                    ArithmeticOperator::Subtract => Ok(v - v1),
                    ArithmeticOperator::Divide => Ok(v / v1),
                    ArithmeticOperator::Modulus => Ok(v % v1),
                },
                Err(e) => Err(e),
            },
            Err(_) => acc,
        });
        match result {
            Ok(v) => match result_type {
                ArithmeticResultType::Number => Ok(ArithmeticResult::Number(v)),
                ArithmeticResultType::Decimal => match BigDecimal::from_i32(v) {
                    Some(v1) => Ok(ArithmeticResult::Decimal(v1)),
                    None => Err(CustomError::Message(Strings::UnexpectedError)),
                },
                ArithmeticResultType::Text => Ok(ArithmeticResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToNumber for NumberArithmeticExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for NumberArithmeticExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for NumberArithmeticExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text, symbols)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// DECIMAL ARITHMETIC

#[derive(Debug)]
enum DecimalArithmeticExpression {
    Add((Box<dyn ToDecimal>, Vec<Box<dyn ToDecimal>>)),
    Multiply((Box<dyn ToDecimal>, Vec<Box<dyn ToDecimal>>)),
    Subtract((Box<dyn ToDecimal>, Vec<Box<dyn ToDecimal>>)),
    Divide((Box<dyn ToDecimal>, Vec<Box<dyn ToDecimal>>)),
    Modulus((Box<dyn ToDecimal>, Vec<Box<dyn ToDecimal>>)),
}

impl DecimalArithmeticExpression {
    fn eval(
        &self,
        result_type: ArithmeticResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<ArithmeticResult, CustomError> {
        let (args, operator) = match self {
            DecimalArithmeticExpression::Add(v) => (v, ArithmeticOperator::Add),
            DecimalArithmeticExpression::Multiply(v) => (v, ArithmeticOperator::Multiply),
            DecimalArithmeticExpression::Subtract(v) => (v, ArithmeticOperator::Subtract),
            DecimalArithmeticExpression::Divide(v) => (v, ArithmeticOperator::Divide),
            DecimalArithmeticExpression::Modulus(v) => (v, ArithmeticOperator::Modulus),
        };
        let init: Result<BigDecimal, CustomError> = args.0.get_decimal(symbols);
        let result: Result<BigDecimal, CustomError> =
            args.1.iter().fold(init, |acc, val| match &acc {
                Ok(v) => match val.get_decimal(symbols) {
                    Ok(v1) => match operator {
                        ArithmeticOperator::Add => Ok(v + v1),
                        ArithmeticOperator::Multiply => Ok(v * v1),
                        ArithmeticOperator::Subtract => Ok(v - v1),
                        ArithmeticOperator::Divide => Ok(v / v1),
                        ArithmeticOperator::Modulus => Ok(v % v1),
                    },
                    Err(e) => Err(e),
                },
                Err(_) => acc,
            });
        match result_type {
            ArithmeticResultType::Number => match result {
                Ok(v) => match v.to_i32() {
                    Some(v1) => Ok(ArithmeticResult::Number(v1)),
                    None => Err(CustomError::Message(Strings::UnexpectedError)),
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

impl ToNumber for DecimalArithmeticExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for DecimalArithmeticExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for DecimalArithmeticExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text, symbols)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
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
enum NumberComparatorExpression {
    Equals((Box<dyn ToNumber>, Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    GreaterThan((Box<dyn ToNumber>, Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    LessThan((Box<dyn ToNumber>, Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    GreaterThanEquals((Box<dyn ToNumber>, Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
    LessThanEquals((Box<dyn ToNumber>, Box<dyn ToNumber>, Vec<Box<dyn ToNumber>>)),
}

impl NumberComparatorExpression {
    fn eval(
        &self,
        result_type: ComparatorResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<ComparatorResult, CustomError> {
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
        let init: Result<bool, CustomError> =
            match (args.0.get_number(symbols), args.1.get_number(symbols)) {
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
                    .map(|val| val.get_number(symbols))
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

impl ToText for NumberComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for NumberComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// DECIMAL COMPARATOR

#[derive(Debug)]
enum DecimalComparatorExpression {
    Equals(
        (
            Box<dyn ToDecimal>,
            Box<dyn ToDecimal>,
            Vec<Box<dyn ToDecimal>>,
        ),
    ),
    GreaterThan(
        (
            Box<dyn ToDecimal>,
            Box<dyn ToDecimal>,
            Vec<Box<dyn ToDecimal>>,
        ),
    ),
    LessThan(
        (
            Box<dyn ToDecimal>,
            Box<dyn ToDecimal>,
            Vec<Box<dyn ToDecimal>>,
        ),
    ),
    GreaterThanEquals(
        (
            Box<dyn ToDecimal>,
            Box<dyn ToDecimal>,
            Vec<Box<dyn ToDecimal>>,
        ),
    ),
    LessThanEquals(
        (
            Box<dyn ToDecimal>,
            Box<dyn ToDecimal>,
            Vec<Box<dyn ToDecimal>>,
        ),
    ),
}

impl DecimalComparatorExpression {
    fn eval(
        &self,
        result_type: ComparatorResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<ComparatorResult, CustomError> {
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
        let init: Result<bool, CustomError> =
            match (args.0.get_decimal(symbols), args.1.get_decimal(symbols)) {
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
                    .map(|val| val.get_decimal(symbols))
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

impl ToText for DecimalComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for DecimalComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// TEXT COMPARATOR

#[derive(Debug)]
enum TextComparatorExpression {
    Equals((Box<dyn ToText>, Box<dyn ToText>, Vec<Box<dyn ToText>>)),
    GreaterThan((Box<dyn ToText>, Box<dyn ToText>, Vec<Box<dyn ToText>>)),
    LessThan((Box<dyn ToText>, Box<dyn ToText>, Vec<Box<dyn ToText>>)),
    GreaterThanEquals((Box<dyn ToText>, Box<dyn ToText>, Vec<Box<dyn ToText>>)),
    LessThanEquals((Box<dyn ToText>, Box<dyn ToText>, Vec<Box<dyn ToText>>)),
}

impl TextComparatorExpression {
    fn eval(
        &self,
        result_type: ComparatorResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<ComparatorResult, CustomError> {
        let (args, operator) = match self {
            TextComparatorExpression::Equals(v) => (v, ComparatorOperator::Equals),
            TextComparatorExpression::GreaterThan(v) => (v, ComparatorOperator::GreaterThan),
            TextComparatorExpression::LessThan(v) => (v, ComparatorOperator::LessThan),
            TextComparatorExpression::GreaterThanEquals(v) => {
                (v, ComparatorOperator::GreaterThanEquals)
            }
            TextComparatorExpression::LessThanEquals(v) => (v, ComparatorOperator::LessThanEquals),
        };
        let init: Result<bool, CustomError> =
            match (args.0.get_text(symbols), args.1.get_text(symbols)) {
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
                    .map(|val| val.get_text(symbols))
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

impl ToText for TextComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for TextComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
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

// BINARY LOGICAL

#[derive(Debug)]
enum LogicalBinaryOperator {
    And,
    Or,
}

#[derive(Debug)]
enum LogicalBinaryExpression {
    And(
        (
            Box<dyn ToBoolean>,
            Box<dyn ToBoolean>,
            Vec<Box<dyn ToBoolean>>,
        ),
    ),
    Or(
        (
            Box<dyn ToBoolean>,
            Box<dyn ToBoolean>,
            Vec<Box<dyn ToBoolean>>,
        ),
    ),
}

impl LogicalBinaryExpression {
    fn eval(
        &self,
        result_type: LogicalResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<LogicalResult, CustomError> {
        let (args, operator) = match self {
            LogicalBinaryExpression::And(v) => (v, LogicalBinaryOperator::And),
            LogicalBinaryExpression::Or(v) => (v, LogicalBinaryOperator::Or),
        };
        let init: Result<bool, CustomError> =
            match (args.0.get_boolean(symbols), args.1.get_boolean(symbols)) {
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
                    .map(|val| val.get_boolean(symbols))
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

impl ToText for LogicalBinaryExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for LogicalBinaryExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean, symbols)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// UNARY LOGICAL

#[derive(Debug)]
enum LogicalUnaryOperator {
    Not,
}

#[derive(Debug)]
struct LogicalUnaryExpression {
    value: Box<dyn ToBoolean>,
}

impl LogicalUnaryExpression {
    fn eval(
        &self,
        result_type: LogicalResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<LogicalResult, CustomError> {
        let result: Result<bool, CustomError> = match self.value.get_boolean(symbols) {
            Ok(v) => Ok(!v),
            Err(e) => Err(e),
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

impl ToText for LogicalUnaryExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for LogicalUnaryExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean, symbols)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// NUMBER MATCH

#[derive(Debug)]
enum NumberMatchResultType {
    Number,
    Decimal,
    Text,
}

#[derive(Debug)]
enum NumberMatchResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

enum NumberMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToNumber>,
            Vec<(Box<dyn ToNumber>, Box<dyn ToNumber>)>,
            Box<dyn ToNumber>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToDecimal>,
            Vec<(Box<dyn ToDecimal>, Box<dyn ToNumber>)>,
            Box<dyn ToNumber>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToText>,
            Vec<(Box<dyn ToText>, Box<dyn ToNumber>)>,
            Box<dyn ToNumber>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToBoolean>,
            Vec<(Box<dyn ToBoolean>, Box<dyn ToNumber>)>,
            Box<dyn ToNumber>,
        ),
    ),
}

impl NumberMatchExpression {
    fn eval(
        &self,
        result_type: NumberMatchResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<NumberMatchResult, CustomError> {
        let result: Result<i32, CustomError> = match self {
            NumberMatchExpression::NumberConditionExpression((condition, guards, otherwise)) => {
                match condition.get_number(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_number(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_number(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_decimal(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_decimal(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_number(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_text(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_text(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_number(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_boolean(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_boolean(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_number(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                NumberMatchResultType::Number => Ok(NumberMatchResult::Number(v)),
                NumberMatchResultType::Decimal => match BigDecimal::from_i32(v) {
                    Some(v1) => Ok(NumberMatchResult::Decimal(v1)),
                    None => Err(CustomError::Message(Strings::UnexpectedError)),
                },
                NumberMatchResultType::Text => Ok(NumberMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToNumber for NumberMatchExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(NumberMatchResultType::Number, symbols)? {
            NumberMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for NumberMatchExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(NumberMatchResultType::Decimal, symbols)? {
            NumberMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for NumberMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(NumberMatchResultType::Text, symbols)? {
            NumberMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// DECIMAL MATCH

#[derive(Debug)]
enum DecimalMatchResultType {
    Number,
    Decimal,
    Text,
}

#[derive(Debug)]
enum DecimalMatchResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

enum DecimalMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToNumber>,
            Vec<(Box<dyn ToNumber>, Box<dyn ToDecimal>)>,
            Box<dyn ToDecimal>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToDecimal>,
            Vec<(Box<dyn ToDecimal>, Box<dyn ToDecimal>)>,
            Box<dyn ToDecimal>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToText>,
            Vec<(Box<dyn ToText>, Box<dyn ToDecimal>)>,
            Box<dyn ToDecimal>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToBoolean>,
            Vec<(Box<dyn ToBoolean>, Box<dyn ToDecimal>)>,
            Box<dyn ToDecimal>,
        ),
    ),
}

impl DecimalMatchExpression {
    fn eval(
        &self,
        result_type: DecimalMatchResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<DecimalMatchResult, CustomError> {
        let result: Result<BigDecimal, CustomError> = match self {
            DecimalMatchExpression::NumberConditionExpression((condition, guards, otherwise)) => {
                match condition.get_number(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_number(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_decimal(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_decimal(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_decimal(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_decimal(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_text(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_text(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_decimal(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_boolean(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_boolean(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_decimal(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                DecimalMatchResultType::Number => match v.to_i32() {
                    Some(v1) => Ok(DecimalMatchResult::Number(v1)),
                    None => Err(CustomError::Message(Strings::UnexpectedError)),
                },
                DecimalMatchResultType::Decimal => Ok(DecimalMatchResult::Decimal(v)),
                DecimalMatchResultType::Text => Ok(DecimalMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToNumber for DecimalMatchExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(DecimalMatchResultType::Number, symbols)? {
            DecimalMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for DecimalMatchExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(DecimalMatchResultType::Decimal, symbols)? {
            DecimalMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for DecimalMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(DecimalMatchResultType::Text, symbols)? {
            DecimalMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// TEXT MATCH

#[derive(Debug)]
enum TextMatchResultType {
    Text,
}

#[derive(Debug)]
enum TextMatchResult {
    Text(String),
}

enum TextMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToNumber>,
            Vec<(Box<dyn ToNumber>, Box<dyn ToText>)>,
            Box<dyn ToText>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToDecimal>,
            Vec<(Box<dyn ToDecimal>, Box<dyn ToText>)>,
            Box<dyn ToText>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToText>,
            Vec<(Box<dyn ToText>, Box<dyn ToText>)>,
            Box<dyn ToText>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToBoolean>,
            Vec<(Box<dyn ToBoolean>, Box<dyn ToText>)>,
            Box<dyn ToText>,
        ),
    ),
}

impl TextMatchExpression {
    fn eval(
        &self,
        result_type: TextMatchResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<TextMatchResult, CustomError> {
        let result: Result<String, CustomError> = match self {
            TextMatchExpression::NumberConditionExpression((condition, guards, otherwise)) => {
                match condition.get_number(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_number(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_text(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_decimal(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_decimal(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_text(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_text(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_text(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_text(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_boolean(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_boolean(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_text(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                TextMatchResultType::Text => Ok(TextMatchResult::Text(v)),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToText for TextMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(TextMatchResultType::Text, symbols)? {
            TextMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// BOOLEAN MATCH

#[derive(Debug)]
enum BooleanMatchResultType {
    Boolean,
    Text,
}

#[derive(Debug)]
enum BooleanMatchResult {
    Boolean(bool),
    Text(String),
}

enum BooleanMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToNumber>,
            Vec<(Box<dyn ToNumber>, Box<dyn ToBoolean>)>,
            Box<dyn ToBoolean>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToDecimal>,
            Vec<(Box<dyn ToDecimal>, Box<dyn ToBoolean>)>,
            Box<dyn ToBoolean>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToText>,
            Vec<(Box<dyn ToText>, Box<dyn ToBoolean>)>,
            Box<dyn ToBoolean>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToBoolean>,
            Vec<(Box<dyn ToBoolean>, Box<dyn ToBoolean>)>,
            Box<dyn ToBoolean>,
        ),
    ),
}

impl BooleanMatchExpression {
    fn eval(
        &self,
        result_type: BooleanMatchResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<BooleanMatchResult, CustomError> {
        let result: Result<bool, CustomError> = match self {
            BooleanMatchExpression::NumberConditionExpression((condition, guards, otherwise)) => {
                match condition.get_number(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_number(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_boolean(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_decimal(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_decimal(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_boolean(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_text(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_text(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_boolean(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_boolean(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_boolean(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_boolean(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                BooleanMatchResultType::Boolean => Ok(BooleanMatchResult::Boolean(v)),
                BooleanMatchResultType::Text => Ok(BooleanMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToBoolean for BooleanMatchExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(BooleanMatchResultType::Text, symbols)? {
            BooleanMatchResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for BooleanMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(BooleanMatchResultType::Text, symbols)? {
            BooleanMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

// DOT OPERATOR

#[derive(Debug)]
enum DotResult {
    Number(i32),
    Decimal(BigDecimal),
    Boolean(bool),
    Text(String),
}

struct DotExpression {
    path: Vec<String>,
}

impl DotExpression {
    fn eval(&self, symbols: &HashMap<String, Symbol>) -> Result<DotResult, CustomError> {
        let init: (Result<&Leaf, CustomError>, &HashMap<String, Symbol>) =
            (Err(CustomError::Message(Strings::MissingSymbol)), &symbols);
        let result: (Result<&Leaf, CustomError>, &HashMap<String, Symbol>) =
            self.path
                .iter()
                .fold(init, |acc, val| match &acc.1.get(val) {
                    Some(v) => match v {
                        Symbol::Leaf(v1) => (Ok(v1), &symbols),
                        Symbol::Symbols(v1) => {
                            (Err(CustomError::Message(Strings::MissingSymbol)), v1)
                        }
                    },
                    None => (Err(CustomError::Message(Strings::MissingSymbol)), &symbols),
                });
        match result.0 {
            Ok(v) => match v {
                Leaf::Number(v1) => Ok(DotResult::Number(*v1)),
                Leaf::Decimal(v1) => Ok(DotResult::Decimal(v1.clone())),
                Leaf::Text(v1) => Ok(DotResult::Text(v1.to_string())),
                Leaf::Boolean(v1) => Ok(DotResult::Boolean(*v1)),
            },
            Err(e) => Err(e),
        }
    }
}

impl ToNumber for DotExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(symbols)? {
            DotResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToDecimal for DotExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(symbols)? {
            DotResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToText for DotExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(symbols)? {
            DotResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

impl ToBoolean for DotExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(symbols)? {
            DotResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Strings::UnexpectedError)),
        }
    }
}

fn main() {
    // let mut book_reviews = HashMap::new();
    // book_reviews.insert(
    //     "Adventures of Huckleberry Finn".to_string(),
    //     "My favorite book.".to_string(),
    // );

    let symbols: HashMap<String, Symbol> = HashMap::new();

    let expr1 = DecimalArithmeticExpression::Add((
        Box::new(BigDecimal::from_i32(3).unwrap()),
        vec![Box::new(BigDecimal::from_i32(141).unwrap())],
    ));
    let expr2 = DecimalArithmeticExpression::Multiply((
        Box::new(BigDecimal::from_i32(12).unwrap()),
        vec![Box::new(BigDecimal::from_i32(12).unwrap())],
    ));

    println!("expr1: {:?}", expr1.get_decimal(&symbols).unwrap());
    println!("expr2: {:?}", expr2.get_decimal(&symbols).unwrap());

    let expr3 = NumberComparatorExpression::GreaterThanEquals((
        Box::new(12),
        Box::new(22),
        vec![Box::new(22)],
    ));

    let expr4 = DecimalComparatorExpression::GreaterThanEquals((
        Box::new(BigDecimal::from(2)),
        Box::new(BigDecimal::from(3)),
        vec![Box::new(BigDecimal::from_str("3.3").unwrap())],
    ));

    println!("expr3: {:?}", &expr3.get_boolean(&symbols).unwrap());
    println!("expr4 {:?}", &expr4.get_boolean(&symbols).unwrap());

    let expr5 = DecimalArithmeticExpression::Add((
        Box::new(BigDecimal::from_i32(3).unwrap()),
        vec![Box::new(BigDecimal::from_i32(4).unwrap())],
    ));

    println!("expr5: {:?}", &expr5.get_text(&symbols).unwrap());

    let expr6 = TextMatchExpression::NumberConditionExpression((
        Box::new(expr1),
        vec![(Box::new(expr2), Box::new(expr4))],
        Box::new(expr5),
    ));
    println!("expr6: {:?}", &expr6.get_text(&symbols).unwrap());
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
