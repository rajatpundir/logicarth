/* Copyright (C) Gokyun (OPC) Private  value: ()  value: () Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

// Notes for the future.
// 1. Build some docs and audio visual documentation for quick understanding
// 2. transmute() can be used to cast Box<ToValue<i32>> to Box<ToValue<V>> in deserializing match expressions
// 3. Match expression can be generalized further with generics, MatchExp<T, U> (T, [(T, U), U])

// TODO
// 1. Add Diesel
// 2. Modularize code

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use core::fmt::Debug;
use serde_json::{json, Value};
use std::collections::HashMap;

enum Language {
    English,
}

#[derive(Debug, Clone)]
enum Message {
    ErrUnexpected,
    ErrMissingSymbol,
    ErrSerialization,
    ErrDeserialization,
    SymbolType,
    SymbolValue,
    SymbolTypeNumber,
    SymbolTypeDecimal,
    SymbolTypeText,
    SymbolTypeBoolean,
}

impl Message {
    fn to_string(&self, lang: &Language) -> String {
        let result: &str = match lang {
            Language::English => match self {
                Message::ErrUnexpected => "Unexpected Error",
                Message::ErrMissingSymbol => "Symbol not found",
                Message::ErrSerialization => "Unable to serialize",
                Message::ErrDeserialization => "Unable to deserialize",
                Message::SymbolType => "type",
                Message::SymbolValue => "value",
                Message::SymbolTypeNumber => "Number",
                Message::SymbolTypeDecimal => "Decimal",
                Message::SymbolTypeText => "Text",
                Message::SymbolTypeBoolean => "Boolean",
            },
        };
        result.to_string()
    }

    fn serialize(&self) -> Value {
        json!(self.to_string(&Language::English))
    }
}

#[derive(Debug, Clone)]
enum CustomError {
    Message(Message),
    Messages(HashMap<String, CustomError>),
}

impl CustomError {
    fn serialize(self, lang: &Language) -> Value {
        match self {
            CustomError::Message(v) => v.serialize(),
            CustomError::Messages(v) => Value::Object(
                v.into_iter()
                    .map(|(key, val)| (key, val.serialize(lang)))
                    .collect(),
            ),
        }
    }
}

// Symbols

#[derive(Clone)]
enum Leaf {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
    Boolean(bool),
}

struct Symbol {
    value: Option<Leaf>,
    values: HashMap<String, Symbol>,
}

// Traits

trait ToValue<T> {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<T, CustomError>;
    fn serialize(&self) -> Result<Value, CustomError>;
}

impl ToValue<i32> for i32 {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        Ok(*self)
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self))
    }
}

impl ToValue<i32> for BigDecimal {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.to_i32() {
            Some(v) => Ok(v),
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        match self.to_i32() {
            Some(v) => Ok(json!(v)),
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }
}

impl ToValue<BigDecimal> for i32 {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match BigDecimal::from_i32(*self) {
            Some(v) => Ok(v),
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        match BigDecimal::from_i32(*self) {
            Some(v) => match v.to_f64() {
                Some(v1) => Ok(json!(v1)),
                None => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }
}

impl ToValue<BigDecimal> for BigDecimal {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        Ok(self.clone())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        match self.to_f64() {
            Some(v) => Ok(json!(v)),
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }
}

impl ToValue<String> for i32 {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToValue<String> for BigDecimal {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToValue<String> for String {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToValue<String> for bool {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToValue<bool> for bool {
    fn get_value(&self, _symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        Ok(*self)
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self))
    }
}

// Arithmetic Ops

enum ArithmeticResultType {
    Number,
    Decimal,
    Text,
}

enum ArithmeticResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

enum ArithmeticOperator {
    Add,
    Multiply,
    Subtract,
    Divide,
    Modulus,
}

// NUMBER ARITHMETIC

enum NumberArithmeticExpression {
    Add((Box<dyn ToValue<i32>>, Vec<Box<dyn ToValue<i32>>>)),
    Multiply((Box<dyn ToValue<i32>>, Vec<Box<dyn ToValue<i32>>>)),
    Subtract((Box<dyn ToValue<i32>>, Vec<Box<dyn ToValue<i32>>>)),
    Divide((Box<dyn ToValue<i32>>, Vec<Box<dyn ToValue<i32>>>)),
    Modulus((Box<dyn ToValue<i32>>, Vec<Box<dyn ToValue<i32>>>)),
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
        let init: Result<i32, CustomError> = args.0.get_value(symbols);
        let result: Result<i32, CustomError> = args.1.iter().fold(init, |acc, val| match &acc {
            Ok(v) => match val.get_value(symbols) {
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
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                ArithmeticResultType::Text => Ok(ArithmeticResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            NumberArithmeticExpression::Add(_) => "+",
            NumberArithmeticExpression::Multiply(_) => "*",
            NumberArithmeticExpression::Subtract(_) => "-",
            NumberArithmeticExpression::Divide(_) => "/",
            NumberArithmeticExpression::Modulus(_) => "%",
        };
        match self {
            NumberArithmeticExpression::Add(v)
            | NumberArithmeticExpression::Multiply(v)
            | NumberArithmeticExpression::Subtract(v)
            | NumberArithmeticExpression::Divide(v)
            | NumberArithmeticExpression::Modulus(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(&v.1)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "type": "Number",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<i32> for NumberArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<BigDecimal> for NumberArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for NumberArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text, symbols)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// DECIMAL ARITHMETIC

enum DecimalArithmeticExpression {
    Add(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    Multiply(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    Subtract(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    Divide(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    Modulus(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
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
        let init: Result<BigDecimal, CustomError> = args.0.get_value(symbols);
        let result: Result<BigDecimal, CustomError> =
            args.1.iter().fold(init, |acc, val| match &acc {
                Ok(v) => match val.get_value(symbols) {
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
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            DecimalArithmeticExpression::Add(_) => "+",
            DecimalArithmeticExpression::Multiply(_) => "*",
            DecimalArithmeticExpression::Subtract(_) => "-",
            DecimalArithmeticExpression::Divide(_) => "/",
            DecimalArithmeticExpression::Modulus(_) => "%",
        };
        match self {
            DecimalArithmeticExpression::Add(v)
            | DecimalArithmeticExpression::Multiply(v)
            | DecimalArithmeticExpression::Subtract(v)
            | DecimalArithmeticExpression::Divide(v)
            | DecimalArithmeticExpression::Modulus(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(&v.1)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "type": "Decimal",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<i32> for DecimalArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<BigDecimal> for DecimalArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for DecimalArithmeticExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ArithmeticResultType::Text, symbols)? {
            ArithmeticResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// COMPARATOR OPS

enum ComparatorResultType {
    Boolean,
    Text,
}

enum ComparatorResult {
    Boolean(bool),
    Text(String),
}

enum ComparatorOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
}

// NUMBER COMPARATOR

enum NumberComparatorExpression {
    Equals(
        (
            Box<dyn ToValue<i32>>,
            Box<dyn ToValue<i32>>,
            Vec<Box<dyn ToValue<i32>>>,
        ),
    ),
    GreaterThan(
        (
            Box<dyn ToValue<i32>>,
            Box<dyn ToValue<i32>>,
            Vec<Box<dyn ToValue<i32>>>,
        ),
    ),
    LessThan(
        (
            Box<dyn ToValue<i32>>,
            Box<dyn ToValue<i32>>,
            Vec<Box<dyn ToValue<i32>>>,
        ),
    ),
    GreaterThanEquals(
        (
            Box<dyn ToValue<i32>>,
            Box<dyn ToValue<i32>>,
            Vec<Box<dyn ToValue<i32>>>,
        ),
    ),
    LessThanEquals(
        (
            Box<dyn ToValue<i32>>,
            Box<dyn ToValue<i32>>,
            Vec<Box<dyn ToValue<i32>>>,
        ),
    ),
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
            match (args.0.get_value(symbols), args.1.get_value(symbols)) {
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
                    .map(|val| val.get_value(symbols))
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
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            NumberComparatorExpression::Equals(_) => "==",
            NumberComparatorExpression::GreaterThanEquals(_) => ">=",
            NumberComparatorExpression::LessThanEquals(_) => "<=",
            NumberComparatorExpression::GreaterThan(_) => ">",
            NumberComparatorExpression::LessThan(_) => "<",
        };
        match self {
            NumberComparatorExpression::Equals(v)
            | NumberComparatorExpression::GreaterThanEquals(v)
            | NumberComparatorExpression::LessThanEquals(v)
            | NumberComparatorExpression::GreaterThan(v)
            | NumberComparatorExpression::LessThan(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(std::iter::once(&v.1))
                    .chain(&v.2)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "type": "Number",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<String> for NumberComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for NumberComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// DECIMAL COMPARATOR

enum DecimalComparatorExpression {
    Equals(
        (
            Box<dyn ToValue<BigDecimal>>,
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    GreaterThan(
        (
            Box<dyn ToValue<BigDecimal>>,
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    LessThan(
        (
            Box<dyn ToValue<BigDecimal>>,
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    GreaterThanEquals(
        (
            Box<dyn ToValue<BigDecimal>>,
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
        ),
    ),
    LessThanEquals(
        (
            Box<dyn ToValue<BigDecimal>>,
            Box<dyn ToValue<BigDecimal>>,
            Vec<Box<dyn ToValue<BigDecimal>>>,
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
            match (args.0.get_value(symbols), args.1.get_value(symbols)) {
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
                    .map(|val| val.get_value(symbols))
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
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            DecimalComparatorExpression::Equals(_) => "==",
            DecimalComparatorExpression::GreaterThanEquals(_) => ">=",
            DecimalComparatorExpression::LessThanEquals(_) => "<=",
            DecimalComparatorExpression::GreaterThan(_) => ">",
            DecimalComparatorExpression::LessThan(_) => "<",
        };
        match self {
            DecimalComparatorExpression::Equals(v)
            | DecimalComparatorExpression::GreaterThanEquals(v)
            | DecimalComparatorExpression::LessThanEquals(v)
            | DecimalComparatorExpression::GreaterThan(v)
            | DecimalComparatorExpression::LessThan(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(std::iter::once(&v.1))
                    .chain(&v.2)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "type": "Decimal",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<String> for DecimalComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for DecimalComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// TEXT COMPARATOR

enum TextComparatorExpression {
    Equals(
        (
            Box<dyn ToValue<String>>,
            Box<dyn ToValue<String>>,
            Vec<Box<dyn ToValue<String>>>,
        ),
    ),
    GreaterThan(
        (
            Box<dyn ToValue<String>>,
            Box<dyn ToValue<String>>,
            Vec<Box<dyn ToValue<String>>>,
        ),
    ),
    LessThan(
        (
            Box<dyn ToValue<String>>,
            Box<dyn ToValue<String>>,
            Vec<Box<dyn ToValue<String>>>,
        ),
    ),
    GreaterThanEquals(
        (
            Box<dyn ToValue<String>>,
            Box<dyn ToValue<String>>,
            Vec<Box<dyn ToValue<String>>>,
        ),
    ),
    LessThanEquals(
        (
            Box<dyn ToValue<String>>,
            Box<dyn ToValue<String>>,
            Vec<Box<dyn ToValue<String>>>,
        ),
    ),
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
            match (args.0.get_value(symbols), args.1.get_value(symbols)) {
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
                    .map(|val| val.get_value(symbols))
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
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            TextComparatorExpression::Equals(_) => "==",
            TextComparatorExpression::GreaterThanEquals(_) => ">=",
            TextComparatorExpression::LessThanEquals(_) => "<=",
            TextComparatorExpression::GreaterThan(_) => ">",
            TextComparatorExpression::LessThan(_) => "<",
        };
        match self {
            TextComparatorExpression::Equals(v)
            | TextComparatorExpression::GreaterThanEquals(v)
            | TextComparatorExpression::LessThanEquals(v)
            | TextComparatorExpression::GreaterThan(v)
            | TextComparatorExpression::LessThan(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(std::iter::once(&v.1))
                    .chain(&v.2)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "type": "Text",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<String> for TextComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for TextComparatorExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(ComparatorResultType::Boolean, symbols)? {
            ComparatorResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// LOGICAL OPS

enum LogicalResultType {
    Boolean,
    Text,
}

enum LogicalResult {
    Boolean(bool),
    Text(String),
}

// BINARY LOGICAL

enum LogicalBinaryOperator {
    And,
    Or,
}

enum LogicalBinaryExpression {
    And(
        (
            Box<dyn ToValue<bool>>,
            Box<dyn ToValue<bool>>,
            Vec<Box<dyn ToValue<bool>>>,
        ),
    ),
    Or(
        (
            Box<dyn ToValue<bool>>,
            Box<dyn ToValue<bool>>,
            Vec<Box<dyn ToValue<bool>>>,
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
            match (args.0.get_value(symbols), args.1.get_value(symbols)) {
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
                    .map(|val| val.get_value(symbols))
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let operator: &str = match self {
            LogicalBinaryExpression::And(_) => "and",
            LogicalBinaryExpression::Or(_) => "or",
        };
        match self {
            LogicalBinaryExpression::And(v) | LogicalBinaryExpression::Or(v) => {
                let mut err: Option<CustomError> = None;
                let result: Vec<Result<Value, CustomError>> = std::iter::once(&v.0)
                    .chain(std::iter::once(&v.1))
                    .chain(&v.2)
                    .map(|val| match val.serialize() {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            err = Some(e.clone());
                            Err(e)
                        }
                    })
                    .collect();
                match err {
                    Some(e) => Err(e),
                    None => {
                        let args: Vec<Value> = result
                            .iter()
                            .map(|val| match val {
                                Ok(v) => v.clone(),
                                Err(_) => panic!(),
                            })
                            .collect();
                        Ok(json!({
                            "op": operator,
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToValue<String> for LogicalBinaryExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for LogicalBinaryExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean, symbols)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// UNARY LOGICAL

struct LogicalUnaryExpression {
    value: Box<dyn ToValue<bool>>,
}

impl LogicalUnaryExpression {
    fn eval(
        &self,
        result_type: LogicalResultType,
        symbols: &HashMap<String, Symbol>,
    ) -> Result<LogicalResult, CustomError> {
        let result: Result<bool, CustomError> = match self.value.get_value(symbols) {
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

    fn serialize(&self) -> Result<Value, CustomError> {
        match self.value.serialize() {
            Ok(v) => Ok(json!({
                "op": "not",
                "args": json!([v])
            })),
            Err(e) => Err(e),
        }
    }
}

impl ToValue<String> for LogicalUnaryExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for LogicalUnaryExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(LogicalResultType::Boolean, symbols)? {
            LogicalResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// NUMBER MATCH

enum NumberMatchResultType {
    Number,
    Decimal,
    Text,
}

enum NumberMatchResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

enum NumberMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToValue<i32>>,
            Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<i32>>)>,
            Box<dyn ToValue<i32>>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<i32>>)>,
            Box<dyn ToValue<i32>>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToValue<String>>,
            Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<i32>>)>,
            Box<dyn ToValue<i32>>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToValue<bool>>,
            Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<i32>>)>,
            Box<dyn ToValue<i32>>,
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
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            NumberMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                NumberMatchResultType::Number => Ok(NumberMatchResult::Number(v)),
                NumberMatchResultType::Decimal => match BigDecimal::from_i32(v) {
                    Some(v1) => Ok(NumberMatchResult::Decimal(v1)),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                NumberMatchResultType::Text => Ok(NumberMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        let return_type: &str = "Number";
        let conditional_type: &str = match self {
            NumberMatchExpression::NumberConditionExpression(_) => "Number",
            NumberMatchExpression::DecimalConditionExpression(_) => "Decimal",
            NumberMatchExpression::TextConditionExpression(_) => "Text",
            NumberMatchExpression::BooleanConditionExpression(_) => "Boolean",
        };
        match self {
            NumberMatchExpression::NumberConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            NumberMatchExpression::DecimalConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            NumberMatchExpression::TextConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            NumberMatchExpression::BooleanConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        }
    }
}

impl ToValue<i32> for NumberMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(NumberMatchResultType::Number, symbols)? {
            NumberMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<BigDecimal> for NumberMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(NumberMatchResultType::Decimal, symbols)? {
            NumberMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for NumberMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(NumberMatchResultType::Text, symbols)? {
            NumberMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// DECIMAL MATCH

enum DecimalMatchResultType {
    Number,
    Decimal,
    Text,
}

enum DecimalMatchResult {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
}

enum DecimalMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToValue<i32>>,
            Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<BigDecimal>>)>,
            Box<dyn ToValue<BigDecimal>>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<BigDecimal>>)>,
            Box<dyn ToValue<BigDecimal>>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToValue<String>>,
            Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<BigDecimal>>)>,
            Box<dyn ToValue<BigDecimal>>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToValue<bool>>,
            Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<BigDecimal>>)>,
            Box<dyn ToValue<BigDecimal>>,
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
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            DecimalMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
        };
        match result {
            Ok(v) => match result_type {
                DecimalMatchResultType::Number => match v.to_i32() {
                    Some(v1) => Ok(DecimalMatchResult::Number(v1)),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                DecimalMatchResultType::Decimal => Ok(DecimalMatchResult::Decimal(v)),
                DecimalMatchResultType::Text => Ok(DecimalMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        let return_type: &str = "Decimal";
        let conditional_type: &str = match self {
            DecimalMatchExpression::NumberConditionExpression(_) => "Number",
            DecimalMatchExpression::DecimalConditionExpression(_) => "Decimal",
            DecimalMatchExpression::TextConditionExpression(_) => "Text",
            DecimalMatchExpression::BooleanConditionExpression(_) => "Boolean",
        };
        match self {
            DecimalMatchExpression::NumberConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            DecimalMatchExpression::DecimalConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            DecimalMatchExpression::TextConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            DecimalMatchExpression::BooleanConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        }
    }
}

impl ToValue<i32> for DecimalMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(DecimalMatchResultType::Number, symbols)? {
            DecimalMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<BigDecimal> for DecimalMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(DecimalMatchResultType::Decimal, symbols)? {
            DecimalMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for DecimalMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(DecimalMatchResultType::Text, symbols)? {
            DecimalMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// TEXT MATCH

enum TextMatchResultType {
    Text,
}

enum TextMatchResult {
    Text(String),
}

enum TextMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToValue<i32>>,
            Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<String>>)>,
            Box<dyn ToValue<String>>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<String>>)>,
            Box<dyn ToValue<String>>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToValue<String>>,
            Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<String>>)>,
            Box<dyn ToValue<String>>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToValue<bool>>,
            Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<String>>)>,
            Box<dyn ToValue<String>>,
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
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            TextMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let return_type: &str = "Text";
        let conditional_type: &str = match self {
            TextMatchExpression::NumberConditionExpression(_) => "Number",
            TextMatchExpression::DecimalConditionExpression(_) => "Decimal",
            TextMatchExpression::TextConditionExpression(_) => "Text",
            TextMatchExpression::BooleanConditionExpression(_) => "Boolean",
        };
        match self {
            TextMatchExpression::NumberConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            TextMatchExpression::DecimalConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            TextMatchExpression::TextConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            TextMatchExpression::BooleanConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        }
    }
}

impl ToValue<String> for TextMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(TextMatchResultType::Text, symbols)? {
            TextMatchResult::Text(v) => Ok(v),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// BOOLEAN MATCH

enum BooleanMatchResultType {
    Boolean,
    Text,
}

enum BooleanMatchResult {
    Boolean(bool),
    Text(String),
}

enum BooleanMatchExpression {
    NumberConditionExpression(
        (
            Box<dyn ToValue<i32>>,
            Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<bool>>)>,
            Box<dyn ToValue<bool>>,
        ),
    ),
    DecimalConditionExpression(
        (
            Box<dyn ToValue<BigDecimal>>,
            Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<bool>>)>,
            Box<dyn ToValue<bool>>,
        ),
    ),
    TextConditionExpression(
        (
            Box<dyn ToValue<String>>,
            Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<bool>>)>,
            Box<dyn ToValue<bool>>,
        ),
    ),
    BooleanConditionExpression(
        (
            Box<dyn ToValue<bool>>,
            Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<bool>>)>,
            Box<dyn ToValue<bool>>,
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
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::DecimalConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::TextConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
                    Err(e) => Err(e),
                }
            }
            BooleanMatchExpression::BooleanConditionExpression((condition, guards, otherwise)) => {
                match condition.get_value(symbols) {
                    Ok(v) => guards
                        .iter()
                        .fold(otherwise, |acc, val| match val.0.get_value(symbols) {
                            Ok(v1) => match v == v1 {
                                true => &val.1,
                                false => acc,
                            },
                            Err(_) => acc,
                        })
                        .get_value(symbols),
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

    fn serialize(&self) -> Result<Value, CustomError> {
        let return_type: &str = "Boolean";
        let conditional_type: &str = match self {
            BooleanMatchExpression::NumberConditionExpression(_) => "Number",
            BooleanMatchExpression::DecimalConditionExpression(_) => "Decimal",
            BooleanMatchExpression::TextConditionExpression(_) => "Text",
            BooleanMatchExpression::BooleanConditionExpression(_) => "Boolean",
        };
        match self {
            BooleanMatchExpression::NumberConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            BooleanMatchExpression::DecimalConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            BooleanMatchExpression::TextConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            BooleanMatchExpression::BooleanConditionExpression(v) => match v.0.serialize() {
                Ok(v1) => match v.2.serialize() {
                    Ok(v2) => {
                        let mut err: Option<CustomError> = None;
                        let result: Vec<Result<Vec<Value>, CustomError>> =
                            v.1.iter()
                                .map(|val| match (val.0.serialize(), val.1.serialize()) {
                                    (Ok(v3), Ok(v4)) => Ok(vec![v3, v4]),
                                    (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => {
                                        err = Some(e.clone());
                                        Err(e)
                                    }
                                })
                                .collect();
                        match err {
                            Some(e) => Err(e),
                            None => {
                                let args: Vec<Vec<Value>> = result
                                    .iter()
                                    .map(|val| match val {
                                        Ok(v) => v.clone(),
                                        Err(_) => panic!(),
                                    })
                                    .collect();
                                Ok(json!({
                                    "op": "match",
                                    "type": [return_type, conditional_type],
                                    "args": Value::Array(vec![v1, json!(args),v2])
                                }))
                            }
                        }
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        }
    }
}

impl ToValue<bool> for BooleanMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(BooleanMatchResultType::Boolean, symbols)? {
            BooleanMatchResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for BooleanMatchExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(BooleanMatchResultType::Text, symbols)? {
            BooleanMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

// DOT OPERATOR

enum DotResult {
    Number(i32),
    Decimal(BigDecimal),
    Boolean(bool),
    Text(String),
}

#[derive(Clone)]
struct DotExpression {
    path: Vec<String>,
}

impl DotExpression {
    fn eval(&self, symbols: &HashMap<String, Symbol>) -> Result<DotResult, CustomError> {
        let result = Self::get_leaf(&self.path, symbols);
        match result {
            Ok(v) => match v {
                Leaf::Number(v1) => Ok(DotResult::Number(v1)),
                Leaf::Decimal(v1) => Ok(DotResult::Decimal(v1)),
                Leaf::Text(v1) => Ok(DotResult::Text(v1)),
                Leaf::Boolean(v1) => Ok(DotResult::Boolean(v1)),
            },
            Err(e) => Err(e),
        }
    }

    fn get_leaf(path: &[String], symbols: &HashMap<String, Symbol>) -> Result<Leaf, CustomError> {
        match path.first() {
            Some(v) => match symbols.get(v) {
                Some(v1) => match path.len() {
                    1 => match &v1.value {
                        Some(v2) => Ok(v2.clone()),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => DotExpression::get_leaf(&path[1..], &v1.values),
                },
                None => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!({
            "op": ".",
            "args": json!(self.path)
        }))
    }
}

impl ToValue<i32> for DotExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(symbols)? {
            DotResult::Number(v) => Ok(v),
            DotResult::Decimal(v) => match v.to_i32() {
                Some(v1) => Ok(v1),
                None => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<BigDecimal> for DotExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(symbols)? {
            DotResult::Number(v) => match BigDecimal::from_i32(v) {
                Some(v1) => Ok(v1),
                None => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            DotResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<String> for DotExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(symbols)? {
            DotResult::Number(v) => Ok(v.to_string()),
            DotResult::Decimal(v) => Ok(v.to_string()),
            DotResult::Text(v) => Ok(v),
            DotResult::Boolean(v) => Ok(v.to_string()),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToValue<bool> for DotExpression {
    fn get_value(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(symbols)? {
            DotResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

enum LispExpression {
    NumberArithmeticExpression(NumberArithmeticExpression),
    DecimalArithmeticExpression(DecimalArithmeticExpression),
    NumberComparatorExpression(NumberComparatorExpression),
    DecimalComparatorExpression(DecimalComparatorExpression),
    TextComparatorExpression(TextComparatorExpression),
    LogicalBinaryExpression(LogicalBinaryExpression),
    LogicalUnaryExpression(LogicalUnaryExpression),
    NumberMatchExpression(NumberMatchExpression),
    DecimalMatchExpression(DecimalMatchExpression),
    TextMatchExpression(TextMatchExpression),
    BooleanMatchExpression(BooleanMatchExpression),
    DotExpression(DotExpression),
}

impl LispExpression {
    fn as_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::NumberComparatorExpression(v) => err,
            LispExpression::DecimalComparatorExpression(v) => err,
            LispExpression::TextComparatorExpression(v) => err,
            LispExpression::LogicalBinaryExpression(v) => err,
            LispExpression::LogicalUnaryExpression(v) => err,
            LispExpression::NumberMatchExpression(v) => v.get_value(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_value(symbols),
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => err,
            LispExpression::DotExpression(v) => v.get_value(symbols),
        }
    }

    fn as_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::NumberComparatorExpression(v) => err,
            LispExpression::DecimalComparatorExpression(v) => err,
            LispExpression::TextComparatorExpression(v) => err,
            LispExpression::LogicalBinaryExpression(v) => err,
            LispExpression::LogicalUnaryExpression(v) => err,
            LispExpression::NumberMatchExpression(v) => v.get_value(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_value(symbols),
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => err,
            LispExpression::DotExpression(v) => v.get_value(symbols),
        }
    }

    fn as_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_value(symbols),
            LispExpression::NumberComparatorExpression(v) => v.get_value(symbols),
            LispExpression::DecimalComparatorExpression(v) => v.get_value(symbols),
            LispExpression::TextComparatorExpression(v) => v.get_value(symbols),
            LispExpression::LogicalBinaryExpression(v) => v.get_value(symbols),
            LispExpression::LogicalUnaryExpression(v) => v.get_value(symbols),
            LispExpression::NumberMatchExpression(v) => v.get_value(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_value(symbols),
            LispExpression::TextMatchExpression(v) => v.get_value(symbols),
            LispExpression::BooleanMatchExpression(v) => v.get_value(symbols),
            LispExpression::DotExpression(v) => v.get_value(symbols),
        }
    }

    fn as_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => err,
            LispExpression::DecimalArithmeticExpression(v) => err,
            LispExpression::NumberComparatorExpression(v) => v.get_value(symbols),
            LispExpression::DecimalComparatorExpression(v) => v.get_value(symbols),
            LispExpression::TextComparatorExpression(v) => v.get_value(symbols),
            LispExpression::LogicalBinaryExpression(v) => v.get_value(symbols),
            LispExpression::LogicalUnaryExpression(v) => v.get_value(symbols),
            LispExpression::NumberMatchExpression(v) => err,
            LispExpression::DecimalMatchExpression(v) => err,
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => v.get_value(symbols),
            LispExpression::DotExpression(v) => v.get_value(symbols),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.serialize(),
            LispExpression::DecimalArithmeticExpression(v) => v.serialize(),
            LispExpression::NumberComparatorExpression(v) => v.serialize(),
            LispExpression::DecimalComparatorExpression(v) => v.serialize(),
            LispExpression::TextComparatorExpression(v) => v.serialize(),
            LispExpression::LogicalBinaryExpression(v) => v.serialize(),
            LispExpression::LogicalUnaryExpression(v) => v.serialize(),
            LispExpression::NumberMatchExpression(v) => v.serialize(),
            LispExpression::DecimalMatchExpression(v) => v.serialize(),
            LispExpression::TextMatchExpression(v) => v.serialize(),
            LispExpression::BooleanMatchExpression(v) => v.serialize(),
            LispExpression::DotExpression(v) => v.serialize(),
        }
    }

    fn deserialize_to_number(val: &Value) -> Result<Box<dyn ToValue<i32>>, CustomError> {
        match val {
            Value::Number(v) => match v.is_f64() {
                true => match v.as_f64() {
                    Some(v1) => match BigDecimal::from_f64(v1) {
                        Some(v2) => Ok(Box::new(v2)),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                false => match v.as_i64() {
                    Some(v1) => Ok(Box::new(v1 as i32)),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::String(v) => match v.parse::<i32>() {
                Ok(v1) => Ok(Box::new(v1)),
                Err(_) => match v.parse::<f64>() {
                    Ok(v2) => match BigDecimal::from_f64(v2) {
                        Some(v3) => Ok(Box::new(v3)),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::Object(_) => match Self::deserialize(val.clone()) {
                Ok(v) => match v {
                    LispExpression::NumberArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::NumberMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DotExpression(v1) => Ok(Box::new(v1)),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn deserialize_to_vec_number(
        values: &Vec<Value>,
    ) -> Result<Vec<Box<dyn ToValue<i32>>>, CustomError> {
        let init: Vec<Result<Box<dyn ToValue<i32>>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(Self::deserialize_to_number(val));
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_decimal(val: &Value) -> Result<Box<dyn ToValue<BigDecimal>>, CustomError> {
        match val {
            Value::Number(v) => match v.is_f64() {
                true => match v.as_f64() {
                    Some(v1) => match BigDecimal::from_f64(v1) {
                        Some(v2) => Ok(Box::new(v2)),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                false => match v.as_i64() {
                    Some(v1) => Ok(Box::new(v1 as i32)),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::String(v) => match v.parse::<i32>() {
                Ok(v1) => Ok(Box::new(v1)),
                Err(_) => match v.parse::<f64>() {
                    Ok(v2) => match BigDecimal::from_f64(v2) {
                        Some(v3) => Ok(Box::new(v3)),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::Object(_) => match Self::deserialize(val.clone()) {
                Ok(v) => match v {
                    LispExpression::NumberArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::NumberMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DotExpression(v1) => Ok(Box::new(v1)),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn deserialize_to_vec_decimal(
        values: &Vec<Value>,
    ) -> Result<Vec<Box<dyn ToValue<BigDecimal>>>, CustomError> {
        let init: Vec<Result<Box<dyn ToValue<BigDecimal>>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(Self::deserialize_to_decimal(val));
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_text(val: &Value) -> Result<Box<dyn ToValue<String>>, CustomError> {
        match val {
            Value::Number(v) => match v.is_f64() {
                true => match v.as_f64() {
                    Some(v1) => match BigDecimal::from_f64(v1) {
                        Some(v2) => Ok(Box::new(v2)),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                false => match v.as_i64() {
                    Some(v1) => Ok(Box::new(v1 as i32)),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::String(v) => Ok(Box::new(v.to_string())),
            Value::Bool(v) => Ok(Box::new(v.to_string())),
            Value::Object(_) => match Self::deserialize(val.clone()) {
                Ok(v) => match v {
                    LispExpression::NumberArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalArithmeticExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::NumberComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::TextComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::LogicalBinaryExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::LogicalUnaryExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::NumberMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::TextMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::BooleanMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DotExpression(v1) => Ok(Box::new(v1)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn deserialize_to_vec_text(
        values: &Vec<Value>,
    ) -> Result<Vec<Box<dyn ToValue<String>>>, CustomError> {
        let init: Vec<Result<Box<dyn ToValue<String>>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(Self::deserialize_to_text(val));
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_boolean(val: &Value) -> Result<Box<dyn ToValue<bool>>, CustomError> {
        match val {
            Value::String(v) => match v.parse::<bool>() {
                Ok(v1) => Ok(Box::new(v1)),
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            Value::Bool(v) => Ok(Box::new(*v)),
            Value::Object(_) => match Self::deserialize(val.clone()) {
                Ok(v) => match v {
                    LispExpression::NumberComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DecimalComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::TextComparatorExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::LogicalBinaryExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::LogicalUnaryExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::BooleanMatchExpression(v1) => Ok(Box::new(v1)),
                    LispExpression::DotExpression(v1) => Ok(Box::new(v1)),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            },
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn deserialize_to_vec_boolean(
        values: &Vec<Value>,
    ) -> Result<Vec<Box<dyn ToValue<bool>>>, CustomError> {
        let init: Vec<Result<Box<dyn ToValue<bool>>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(Self::deserialize_to_boolean(val));
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_string(val: &Value) -> Result<String, CustomError> {
        match val {
            Value::Number(v) => match v.is_f64() {
                true => match v.as_f64() {
                    Some(v1) => match BigDecimal::from_f64(v1) {
                        Some(v2) => Ok(v2.to_string()),
                        None => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                false => match v.as_i64() {
                    Some(v1) => Ok((v1 as i32).to_string()),
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
            },
            Value::String(v) => Ok(v.to_string()),
            Value::Bool(v) => Ok(v.to_string()),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn deserialize_to_vec_string(values: &Vec<Value>) -> Result<Vec<String>, CustomError> {
        let init: Vec<Result<String, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(Self::deserialize_to_string(val));
                acc
            })
            .into_iter()
            .collect()
    }

    // NUMBER MATCH

    fn deserialize_to_number_match_number(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<i32>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_number(v1),
                            Self::deserialize_to_number(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_number_match_decimal(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<i32>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_decimal(v1),
                            Self::deserialize_to_number(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_number_match_text(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<i32>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_text(v1),
                            Self::deserialize_to_number(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_number_match_boolean(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<i32>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_boolean(v1),
                            Self::deserialize_to_number(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    // DECIMAL MATCH

    fn deserialize_to_decimal_match_number(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<BigDecimal>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_number(v1),
                            Self::deserialize_to_decimal(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_decimal_match_decimal(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<BigDecimal>>)>, CustomError>
    {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_decimal(v1),
                            Self::deserialize_to_decimal(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_decimal_match_text(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<BigDecimal>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_text(v1),
                            Self::deserialize_to_decimal(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_decimal_match_boolean(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<BigDecimal>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_boolean(v1),
                            Self::deserialize_to_decimal(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    // TEXT MATCH

    fn deserialize_to_text_match_number(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<String>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_number(v1),
                            Self::deserialize_to_text(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_text_match_decimal(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<String>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_decimal(v1),
                            Self::deserialize_to_text(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_text_match_text(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<String>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => {
                            match (Self::deserialize_to_text(v1), Self::deserialize_to_text(v2)) {
                                (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            }
                        }
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_text_match_boolean(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<String>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_boolean(v1),
                            Self::deserialize_to_text(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    // BOOLEAN MATCH

    fn deserialize_to_boolean_match_number(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<i32>>, Box<dyn ToValue<bool>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_number(v1),
                            Self::deserialize_to_boolean(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_boolean_match_decimal(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<BigDecimal>>, Box<dyn ToValue<bool>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_decimal(v1),
                            Self::deserialize_to_boolean(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_boolean_match_text(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<String>>, Box<dyn ToValue<bool>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_text(v1),
                            Self::deserialize_to_boolean(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_boolean_match_boolean(
        values: &Vec<Value>,
    ) -> Result<Vec<(Box<dyn ToValue<bool>>, Box<dyn ToValue<bool>>)>, CustomError> {
        let init = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Array(v) => match (v.first(), v.get(1)) {
                        (Some(v1), Some(v2)) => match (
                            Self::deserialize_to_boolean(v1),
                            Self::deserialize_to_boolean(v2),
                        ) {
                            (Ok(v3), Ok(v4)) => Ok((v3, v4)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize(json: Value) -> Result<LispExpression, CustomError> {
        match json {
            Value::Object(v) => {
                match (v.get("op"), v.get("type"), v.get("args")) {
                    (Some(v1), Some(v2), Some(v3)) => {
                        match v1 {
                            Value::String(v4) => match v4.as_str() {
                                "+" | "*" | "-" | "/" | "%" => match v2 {
                                    Value::String(v5) => match v5.as_str() {
                                        "Number" => match v3 {
                                            Value::Array(v6) => match v6.first() {
                                                Some(v7) => match (Self::deserialize_to_number(v7), Self::deserialize_to_vec_number(&v6[1..].to_vec())) {
                                                    (Ok(v8), Ok(v9)) => match v4.as_str() {
                                                        "+" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Add((v8, v9))))
                                                        },
                                                        "*" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Multiply((v8, v9))))
                                                        },
                                                        "-" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Subtract((v8, v9))))
                                                        },
                                                        "/" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Divide((v8, v9))))
                                                        },
                                                        "%" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Modulus((v8, v9))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        "Decimal" => match v3 {
                                            Value::Array(v6) => match v6.first() {
                                                Some(v7) => match (Self::deserialize_to_decimal(v7), Self::deserialize_to_vec_decimal(&v6[1..].to_vec())) {
                                                    (Ok(v8), Ok(v9)) => match v4.as_str() {
                                                        "+" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Add((v8, v9))))
                                                        },
                                                        "*" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Multiply((v8, v9))))
                                                        },
                                                        "-" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Subtract((v8, v9))))
                                                        },
                                                        "/" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Divide((v8, v9))))
                                                        },
                                                        "%" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Modulus((v8, v9))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                    },
                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                },
                                "==" | ">=" | "<=" | ">" | "<" => match v2 {
                                    Value::String(v5) => match v5.as_str() {
                                        "Number" => match v3 {
                                            Value::Array(v6) => match (v6.first(), v6.get(0)) {
                                                (Some(v7), Some(v8)) => match (Self::deserialize_to_number(v7), Self::deserialize_to_number(v8), Self::deserialize_to_vec_number(&v6[2..].to_vec())) {
                                                    (Ok(v9), Ok(v10), Ok(v11)) => match v4.as_str() {
                                                        "==" => {
                                                            Ok(LispExpression::NumberComparatorExpression(NumberComparatorExpression::Equals((v9, v10, v11))))
                                                        },
                                                        ">=" => {
                                                            Ok(LispExpression::NumberComparatorExpression(NumberComparatorExpression::GreaterThanEquals((v9, v10, v11))))
                                                        },
                                                        "<=" => {
                                                            Ok(LispExpression::NumberComparatorExpression(NumberComparatorExpression::LessThanEquals((v9, v10, v11))))
                                                        },
                                                        ">" => {
                                                            Ok(LispExpression::NumberComparatorExpression(NumberComparatorExpression::GreaterThan((v9, v10, v11))))
                                                        },
                                                        "<" => {
                                                            Ok(LispExpression::NumberComparatorExpression(NumberComparatorExpression::LessThan((v9, v10, v11))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        "Decimal" => match v3 {
                                            Value::Array(v6) => match (v6.first(), v6.get(0)) {
                                                (Some(v7), Some(v8)) => match (Self::deserialize_to_decimal(v7), Self::deserialize_to_decimal(v8), Self::deserialize_to_vec_decimal(&v6[2..].to_vec())) {
                                                    (Ok(v9), Ok(v10), Ok(v11)) => match v4.as_str() {
                                                        "==" => {
                                                            Ok(LispExpression::DecimalComparatorExpression(DecimalComparatorExpression::Equals((v9, v10, v11))))
                                                        },
                                                        ">=" => {
                                                            Ok(LispExpression::DecimalComparatorExpression(DecimalComparatorExpression::GreaterThanEquals((v9, v10, v11))))
                                                        },
                                                        "<=" => {
                                                            Ok(LispExpression::DecimalComparatorExpression(DecimalComparatorExpression::LessThanEquals((v9, v10, v11))))
                                                        },
                                                        ">" => {
                                                            Ok(LispExpression::DecimalComparatorExpression(DecimalComparatorExpression::GreaterThan((v9, v10, v11))))
                                                        },
                                                        "<" => {
                                                            Ok(LispExpression::DecimalComparatorExpression(DecimalComparatorExpression::LessThan((v9, v10, v11))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        "Text" => match v3 {
                                            Value::Array(v6) => match (v6.first(), v6.get(0)) {
                                                (Some(v7), Some(v8)) => match (Self::deserialize_to_text(v7), Self::deserialize_to_text(v8), Self::deserialize_to_vec_text(&v6[2..].to_vec())) {
                                                    (Ok(v9), Ok(v10), Ok(v11)) => match v4.as_str() {
                                                        "==" => {
                                                            Ok(LispExpression::TextComparatorExpression(TextComparatorExpression::Equals((v9, v10, v11))))
                                                        },
                                                        ">=" => {
                                                            Ok(LispExpression::TextComparatorExpression(TextComparatorExpression::GreaterThanEquals((v9, v10, v11))))
                                                        },
                                                        "<=" => {
                                                            Ok(LispExpression::TextComparatorExpression(TextComparatorExpression::LessThanEquals((v9, v10, v11))))
                                                        },
                                                        ">" => {
                                                            Ok(LispExpression::TextComparatorExpression(TextComparatorExpression::GreaterThan((v9, v10, v11))))
                                                        },
                                                        "<" => {
                                                            Ok(LispExpression::TextComparatorExpression(TextComparatorExpression::LessThan((v9, v10, v11))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                    },
                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                }
                                "match" => match v2 {
                                    Value::Array(v5) => match (v5.first(), v5.get(1)) {
                                        (Some(v6), Some(v7)) => match (Self::deserialize_to_string(v6), Self::deserialize_to_string(v7)) {
                                            (Ok(v8), Ok(v9)) => match v8.as_str() {
                                                "Number" => match v9.as_str() {
                                                    "Number" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_number(v10), v11, Self::deserialize_to_number(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_number_match_number(v14) {
                                                                    Ok(v16) => Ok(LispExpression::NumberMatchExpression(NumberMatchExpression::NumberConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Decimal" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_decimal(v10), v11, Self::deserialize_to_number(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_number_match_decimal(v14) {
                                                                    Ok(v16) => Ok(LispExpression::NumberMatchExpression(NumberMatchExpression::DecimalConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Text" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_text(v10), v11, Self::deserialize_to_number(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_number_match_text(v14) {
                                                                    Ok(v16) => Ok(LispExpression::NumberMatchExpression(NumberMatchExpression::TextConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Boolean" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_boolean(v10), v11, Self::deserialize_to_number(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_number_match_boolean(v14) {
                                                                    Ok(v16) => Ok(LispExpression::NumberMatchExpression(NumberMatchExpression::BooleanConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                "Decimal" => match v9.as_str() {
                                                    "Number" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_number(v10), v11, Self::deserialize_to_decimal(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_decimal_match_number(v14) {
                                                                    Ok(v16) => Ok(LispExpression::DecimalMatchExpression(DecimalMatchExpression::NumberConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Decimal" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_decimal(v10), v11, Self::deserialize_to_decimal(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_decimal_match_decimal(v14) {
                                                                    Ok(v16) => Ok(LispExpression::DecimalMatchExpression(DecimalMatchExpression::DecimalConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Text" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_text(v10), v11, Self::deserialize_to_decimal(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_decimal_match_text(v14) {
                                                                    Ok(v16) => Ok(LispExpression::DecimalMatchExpression(DecimalMatchExpression::TextConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Boolean" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_boolean(v10), v11, Self::deserialize_to_decimal(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_decimal_match_boolean(v14) {
                                                                    Ok(v16) => Ok(LispExpression::DecimalMatchExpression(DecimalMatchExpression::BooleanConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                "Text" => match v9.as_str() {
                                                    "Number" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_number(v10), v11, Self::deserialize_to_text(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_text_match_number(v14) {
                                                                    Ok(v16) => Ok(LispExpression::TextMatchExpression(TextMatchExpression::NumberConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Decimal" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_decimal(v10), v11, Self::deserialize_to_text(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_text_match_decimal(v14) {
                                                                    Ok(v16) => Ok(LispExpression::TextMatchExpression(TextMatchExpression::DecimalConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Text" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_text(v10), v11, Self::deserialize_to_text(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_text_match_text(v14) {
                                                                    Ok(v16) => Ok(LispExpression::TextMatchExpression(TextMatchExpression::TextConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Boolean" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_boolean(v10), v11, Self::deserialize_to_text(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_text_match_boolean(v14) {
                                                                    Ok(v16) => Ok(LispExpression::TextMatchExpression(TextMatchExpression::BooleanConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                "Boolean" => match v9.as_str() {
                                                    "Number" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_number(v10), v11, Self::deserialize_to_boolean(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_boolean_match_number(v14) {
                                                                    Ok(v16) => Ok(LispExpression::BooleanMatchExpression(BooleanMatchExpression::NumberConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Decimal" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_decimal(v10), v11, Self::deserialize_to_boolean(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_boolean_match_decimal(v14) {
                                                                    Ok(v16) => Ok(LispExpression::BooleanMatchExpression(BooleanMatchExpression::DecimalConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Text" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_text(v10), v11, Self::deserialize_to_boolean(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_boolean_match_text(v14) {
                                                                    Ok(v16) => Ok(LispExpression::BooleanMatchExpression(BooleanMatchExpression::TextConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    "Boolean" => match v3 {
                                                        Value::Array(v10) => match (v10.first(), v10.get(1), v10.get(2)) {
                                                            (Some(v10), Some(v11), Some(v12)) => match (Self::deserialize_to_boolean(v10), v11, Self::deserialize_to_boolean(v12)) {
                                                                (Ok(v13), Value::Array(v14), Ok(v15)) => match Self::deserialize_to_boolean_match_boolean(v14) {
                                                                    Ok(v16) => Ok(LispExpression::BooleanMatchExpression(BooleanMatchExpression::BooleanConditionExpression((v13, v16, v15)))),
                                                                    Err(e) => Err(e),
                                                                },
                                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                            },
                                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                    },
                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                }
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        }
                    }
                    (Some(v1), None, Some(v2)) => match v1 {
                        Value::String(v3) => match v3.as_str() {
                            "and" | "or" => match v2 {
                                Value::Array(v4) => match (v4.first(), v4.get(1)) {
                                    (Some(v5), Some(v6)) => match (Self::deserialize_to_boolean(v5), Self::deserialize_to_boolean(v6), Self::deserialize_to_vec_boolean(&v4[2..].to_vec())) {
                                        (Ok(v7), Ok(v8), Ok(v9)) => match v3.as_str() {
                                            "and" => {
                                                Ok(LispExpression::LogicalBinaryExpression(
                                                    LogicalBinaryExpression::And((v7, v8, v9,)),
                                                ))
                                            }
                                            "or" => {
                                                Ok(LispExpression::LogicalBinaryExpression(
                                                    LogicalBinaryExpression::Or((v7, v8, v9,)),
                                                ))
                                            }
                                            _ => Err(CustomError::Message(
                                                Message::ErrUnexpected,
                                            )),
                                        },
                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                    },
                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                },
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            "not" => match v2 {
                                Value::Array(v4) => match v4.first() {
                                    Some(v5) => match Self::deserialize_to_boolean(v5) {
                                        Ok(v6) => Ok(LispExpression::LogicalUnaryExpression(LogicalUnaryExpression { value: v6 })),
                                        Err(e) => Err(e),
                                     }
                                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                                }
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            "." => match v2 {
                                Value::Array(v4) => match Self::deserialize_to_vec_string(v4) {
                                    Ok(v5) => Ok(LispExpression::DotExpression(DotExpression { path: v5 })),
                                    Err(e) => Err(e),
                                }
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                }
            }
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod lisp_tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_number_arithmetic_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = NumberArithmeticExpression::Add((
            Box::new(2),
            vec![Box::new(BigDecimal::from_str("2.3").unwrap()), Box::new(7)],
        ));
        let res: i32 = (&expr).get_value(&symbols).unwrap();
        assert_eq!(11, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize((&expr as &dyn ToValue<i32>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::NumberArithmeticExpression(v) =>
                        (&v as &dyn ToValue<i32>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<i32>).serialize().unwrap(),
            match LispExpression::deserialize(
                (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::NumberArithmeticExpression(v) =>
                        (&v as &dyn ToValue<i32>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }

    #[test]
    fn test_decimal_arithmetic_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = DecimalArithmeticExpression::Add((
            Box::new(2),
            vec![Box::new(BigDecimal::from_str("2.3").unwrap())],
        ));
        let res: BigDecimal = (&expr).get_value(&symbols).unwrap();
        assert_eq!(BigDecimal::from_str("4.3").unwrap(), res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::DecimalArithmeticExpression(v) =>
                        (&v as &dyn ToValue<BigDecimal>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap(),
            match LispExpression::deserialize(
                (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::DecimalArithmeticExpression(v) =>
                        (&v as &dyn ToValue<BigDecimal>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }

    #[test]
    fn test_number_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = NumberComparatorExpression::Equals((
            Box::new(2),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(2)],
        ));
        let res: bool = (&expr).get_value(&symbols).unwrap();
        assert_eq!(true, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<bool>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::NumberComparatorExpression(v) =>
                        (&v as &dyn ToValue<bool>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<bool>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<bool>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::NumberComparatorExpression(v) =>
                        (&v as &dyn ToValue<bool>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
        let res: bool = NumberComparatorExpression::GreaterThanEquals((
            Box::new(2),
            Box::new(BigDecimal::from_str("3.3").unwrap()),
            vec![Box::new(4)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = NumberComparatorExpression::LessThan((
            Box::new(5),
            Box::new(BigDecimal::from_str("3.3").unwrap()),
            vec![Box::new(2)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = NumberComparatorExpression::GreaterThanEquals((
            Box::new(2),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(2)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = NumberComparatorExpression::LessThanEquals((
            Box::new(2),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(2)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_decimal_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = DecimalComparatorExpression::Equals((
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(BigDecimal::from_str("2.3").unwrap())],
        ));
        let res: bool = (&expr).get_value(&symbols).unwrap();
        assert_eq!(true, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<bool>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::DecimalComparatorExpression(v) =>
                        (&v as &dyn ToValue<bool>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<bool>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<bool>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::DecimalComparatorExpression(v) =>
                        (&v as &dyn ToValue<bool>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
        let res: bool = DecimalComparatorExpression::GreaterThanEquals((
            Box::new(2),
            Box::new(BigDecimal::from_str("3.3").unwrap()),
            vec![Box::new(4)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = DecimalComparatorExpression::LessThan((
            Box::new(5),
            Box::new(BigDecimal::from_str("3.3").unwrap()),
            vec![Box::new(2)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = DecimalComparatorExpression::GreaterThanEquals((
            Box::new(2),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(3)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
        let res: bool = DecimalComparatorExpression::LessThanEquals((
            Box::new(4),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(1)],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_text_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let res: bool = TextComparatorExpression::Equals((
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            Box::new(BigDecimal::from_str("2.3").unwrap()),
            vec![Box::new(BigDecimal::from_str("2.3").unwrap())],
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(true, res);
    }

    #[test]
    fn test_logical_binary_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = LogicalBinaryExpression::And((Box::new(true), Box::new(true), vec![]));
        let res: bool = (&expr).get_value(&symbols).unwrap();
        assert_eq!(true, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<bool>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::LogicalBinaryExpression(v) =>
                        (&v as &dyn ToValue<bool>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<bool>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<bool>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::LogicalBinaryExpression(v) =>
                        (&v as &dyn ToValue<bool>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
        let res: bool = LogicalBinaryExpression::And((Box::new(true), Box::new(false), vec![]))
            .get_value(&symbols)
            .unwrap();
        assert_eq!(false, res);
        let res: bool = LogicalBinaryExpression::Or((Box::new(true), Box::new(false), vec![]))
            .get_value(&symbols)
            .unwrap();
        assert_eq!(true, res);
        let res: bool = LogicalBinaryExpression::Or((Box::new(false), Box::new(false), vec![]))
            .get_value(&symbols)
            .unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_logical_unary_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = LogicalUnaryExpression {
            value: Box::new(false),
        };
        let res: bool = (&expr).get_value(&symbols).unwrap();
        assert_eq!(true, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<bool>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::LogicalUnaryExpression(v) =>
                        (&v as &dyn ToValue<bool>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<bool>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<bool>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::LogicalUnaryExpression(v) =>
                        (&v as &dyn ToValue<bool>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
        let res: bool = LogicalUnaryExpression {
            value: Box::new(true),
        }
        .get_value(&symbols)
        .unwrap();
        assert_eq!(false, res);
    }

    #[test]
    fn test_number_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr =
            NumberMatchExpression::NumberConditionExpression((Box::new(2), vec![], Box::new(7)));
        let res: i32 = (&expr).get_value(&symbols).unwrap();
        assert_eq!(7, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize((&expr as &dyn ToValue<i32>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::NumberMatchExpression(v) =>
                        (&v as &dyn ToValue<i32>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<i32>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<i32>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::NumberMatchExpression(v) =>
                        (&v as &dyn ToValue<i32>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
        let res: i32 = NumberMatchExpression::NumberConditionExpression((
            Box::new(2),
            vec![
                (Box::new(5), Box::new(8)),
                (Box::new(2), Box::new(11)),
                (Box::new(3), Box::new(13)),
            ],
            Box::new(7),
        ))
        .get_value(&symbols)
        .unwrap();
        assert_eq!(11, res);
    }

    #[test]
    fn test_decimal_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = DecimalMatchExpression::NumberConditionExpression((
            Box::new(2),
            vec![],
            Box::new(BigDecimal::from_str("2.3").unwrap()),
        ));
        let res: BigDecimal = (&expr).get_value(&symbols).unwrap();
        assert_eq!(BigDecimal::from_str("2.3").unwrap(), res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::DecimalMatchExpression(v) =>
                        (&v as &dyn ToValue<BigDecimal>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap(),
            match LispExpression::deserialize(
                (&expr as &dyn ToValue<BigDecimal>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::DecimalMatchExpression(v) =>
                        (&v as &dyn ToValue<BigDecimal>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }

    #[test]
    fn test_text_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = TextMatchExpression::NumberConditionExpression((
            Box::new(2),
            vec![],
            Box::new(BigDecimal::from_str("2.3").unwrap()),
        ));
        let res: String = (&expr).get_value(&symbols).unwrap();
        assert_eq!("2.3".to_string(), res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<String>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::TextMatchExpression(v) =>
                        (&v as &dyn ToValue<String>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<String>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<String>).serialize().unwrap())
            {
                Ok(v) => match v {
                    LispExpression::TextMatchExpression(v) =>
                        (&v as &dyn ToValue<String>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }

    #[test]
    fn test_boolean_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        let expr = BooleanMatchExpression::NumberConditionExpression((
            Box::new(2),
            vec![],
            Box::new(false),
        ));
        let res: bool = (&expr).get_value(&symbols).unwrap();
        assert_eq!(false, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize(
                (&expr as &dyn ToValue<bool>).serialize().unwrap()
            ) {
                Ok(v) => match v {
                    LispExpression::BooleanMatchExpression(v) =>
                        (&v as &dyn ToValue<bool>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<bool>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<bool>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::BooleanMatchExpression(v) =>
                        (&v as &dyn ToValue<bool>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }

    #[test]
    fn test_dot_expression() {
        let symbols: HashMap<String, Symbol> = vec![
            (
                "x".to_string(),
                Symbol {
                    value: Some(Leaf::Number(2)),
                    values: HashMap::new(),
                },
            ),
            (
                "y".to_string(),
                Symbol {
                    value: Some(Leaf::Number(3)),
                    values: HashMap::new(),
                },
            ),
            (
                "z".to_string(),
                Symbol {
                    value: None,
                    values: vec![(
                        "z".to_string(),
                        Symbol {
                            value: Some(Leaf::Number(6)),
                            values: HashMap::new(),
                        },
                    )]
                    .into_iter()
                    .collect(),
                },
            ),
        ]
        .into_iter()
        .collect();
        let expr = DecimalArithmeticExpression::Add((
            Box::new(DotExpression {
                path: vec![String::from("x")],
            }),
            vec![
                Box::new(DotExpression {
                    path: vec![String::from("y")],
                }),
                Box::new(DotExpression {
                    path: vec![String::from("z"), String::from("z")],
                }),
            ],
        ));
        let res: i32 = (&expr).get_value(&symbols).unwrap();
        assert_eq!(11, res);
        // eval == serialize.deserialize.eval
        assert_eq!(
            res,
            (match LispExpression::deserialize((&expr as &dyn ToValue<i32>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::DecimalArithmeticExpression(v) =>
                        (&v as &dyn ToValue<i32>).get_value(&symbols),
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
            })
            .unwrap()
        );
        // serialize == serialize.deserialize.serialize
        assert_eq!(
            (&expr as &dyn ToValue<i32>).serialize().unwrap(),
            match LispExpression::deserialize((&expr as &dyn ToValue<i32>).serialize().unwrap()) {
                Ok(v) => match v {
                    LispExpression::DecimalArithmeticExpression(v) =>
                        (&v as &dyn ToValue<i32>).serialize().unwrap(),
                    _ => Value::Null,
                },
                Err(_) => Value::Null,
            }
        );
    }
}
