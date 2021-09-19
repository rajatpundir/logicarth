/* Copyright (C) Gokyun (OPC) Private  value: ()  value: () Limited - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Rajat Pundir <rajatpundir13@gmail.com>, August 2021
 */

// Notes.
// 1. Test serializers, implement deserializers
// 2. Modularize code
// 3. Build some audio visual documentation
// 4. Add Diesel

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use core::fmt::Debug;
use serde_json::{json, Value};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
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

#[derive(Debug, Clone)]
enum Leaf {
    Number(i32),
    Decimal(BigDecimal),
    Text(String),
    Boolean(bool),
}

#[derive(Debug)]
struct Symbol {
    value: Option<Leaf>,
    values: HashMap<String, Symbol>,
}

// Traits

trait ToNumber {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError>;
    fn serialize(&self) -> Result<Value, CustomError>;
}
// dyn_clone::clone_trait_object!(ToNumber);

impl Debug for dyn ToNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_number(&HashMap::new()))
    }
}

impl ToNumber for i32 {
    fn get_number(&self, _symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        Ok(*self)
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self))
    }
}

impl ToNumber for BigDecimal {
    fn get_number(&self, _symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
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

trait ToDecimal {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError>;
    fn serialize(&self) -> Result<Value, CustomError>;
}
// dyn_clone::clone_trait_object!(ToDecimal);

impl Debug for dyn ToDecimal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_decimal(&HashMap::new()))
    }
}

impl ToDecimal for i32 {
    fn get_decimal(&self, _symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
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

impl ToDecimal for BigDecimal {
    fn get_decimal(&self, _symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        Ok(self.clone())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        match self.to_f64() {
            Some(v) => Ok(json!(v)),
            None => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }
}

trait ToText {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError>;
    fn serialize(&self) -> Result<Value, CustomError>;
}
// dyn_clone::clone_trait_object!(ToText);

impl Debug for dyn ToText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_text(&HashMap::new()))
    }
}

impl ToText for i32 {
    fn get_text(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToText for BigDecimal {
    fn get_text(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToText for String {
    fn get_text(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

impl ToText for bool {
    fn get_text(&self, _symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        Ok(self.to_string())
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self.to_string()))
    }
}

trait ToBoolean {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError>;
    fn serialize(&self) -> Result<Value, CustomError>;
}
// dyn_clone::clone_trait_object!(ToBoolean);

impl Debug for dyn ToBoolean {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_boolean(&HashMap::new()))
    }
}

impl ToBoolean for bool {
    fn get_boolean(&self, _symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        Ok(*self)
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        Ok(json!(self))
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

#[derive(Debug, Clone)]
enum NumberArithmeticExpression {
    Add((Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    Multiply((Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    Subtract((Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    Divide((Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    Modulus((Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
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

impl ToNumber for NumberArithmeticExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToDecimal for NumberArithmeticExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToText for NumberArithmeticExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
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

#[derive(Debug, Clone)]
enum DecimalArithmeticExpression {
    Add((Rc<dyn ToDecimal>, Vec<Rc<dyn ToDecimal>>)),
    Multiply((Rc<dyn ToDecimal>, Vec<Rc<dyn ToDecimal>>)),
    Subtract((Rc<dyn ToDecimal>, Vec<Rc<dyn ToDecimal>>)),
    Divide((Rc<dyn ToDecimal>, Vec<Rc<dyn ToDecimal>>)),
    Modulus((Rc<dyn ToDecimal>, Vec<Rc<dyn ToDecimal>>)),
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

impl ToNumber for DecimalArithmeticExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(ArithmeticResultType::Number, symbols)? {
            ArithmeticResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToDecimal for DecimalArithmeticExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(ArithmeticResultType::Decimal, symbols)? {
            ArithmeticResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToText for DecimalArithmeticExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
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

#[derive(Debug, Clone)]
enum NumberComparatorExpression {
    Equals((Rc<dyn ToNumber>, Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    GreaterThan((Rc<dyn ToNumber>, Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    LessThan((Rc<dyn ToNumber>, Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    GreaterThanEquals((Rc<dyn ToNumber>, Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
    LessThanEquals((Rc<dyn ToNumber>, Rc<dyn ToNumber>, Vec<Rc<dyn ToNumber>>)),
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

impl ToText for NumberComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToBoolean for NumberComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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

#[derive(Debug, Clone)]
enum DecimalComparatorExpression {
    Equals(
        (
            Rc<dyn ToDecimal>,
            Rc<dyn ToDecimal>,
            Vec<Rc<dyn ToDecimal>>,
        ),
    ),
    GreaterThan(
        (
            Rc<dyn ToDecimal>,
            Rc<dyn ToDecimal>,
            Vec<Rc<dyn ToDecimal>>,
        ),
    ),
    LessThan(
        (
            Rc<dyn ToDecimal>,
            Rc<dyn ToDecimal>,
            Vec<Rc<dyn ToDecimal>>,
        ),
    ),
    GreaterThanEquals(
        (
            Rc<dyn ToDecimal>,
            Rc<dyn ToDecimal>,
            Vec<Rc<dyn ToDecimal>>,
        ),
    ),
    LessThanEquals(
        (
            Rc<dyn ToDecimal>,
            Rc<dyn ToDecimal>,
            Vec<Rc<dyn ToDecimal>>,
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

impl ToText for DecimalComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToBoolean for DecimalComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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

#[derive(Debug, Clone)]
enum TextComparatorExpression {
    Equals((Rc<dyn ToText>, Rc<dyn ToText>, Vec<Rc<dyn ToText>>)),
    GreaterThan((Rc<dyn ToText>, Rc<dyn ToText>, Vec<Rc<dyn ToText>>)),
    LessThan((Rc<dyn ToText>, Rc<dyn ToText>, Vec<Rc<dyn ToText>>)),
    GreaterThanEquals((Rc<dyn ToText>, Rc<dyn ToText>, Vec<Rc<dyn ToText>>)),
    LessThanEquals((Rc<dyn ToText>, Rc<dyn ToText>, Vec<Rc<dyn ToText>>)),
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

impl ToText for TextComparatorExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(ComparatorResultType::Text, symbols)? {
            ComparatorResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToBoolean for TextComparatorExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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

#[derive(Debug, Clone)]
enum LogicalBinaryExpression {
    And(
        (
            Rc<dyn ToBoolean>,
            Rc<dyn ToBoolean>,
            Vec<Rc<dyn ToBoolean>>,
        ),
    ),
    Or(
        (
            Rc<dyn ToBoolean>,
            Rc<dyn ToBoolean>,
            Vec<Rc<dyn ToBoolean>>,
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
                            "type": "Boolean",
                            "args": args
                        }))
                    }
                }
            }
        }
    }
}

impl ToText for LogicalBinaryExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToBoolean for LogicalBinaryExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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

#[derive(Debug, Clone)]
struct LogicalUnaryExpression {
    value: Rc<dyn ToBoolean>,
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

    fn serialize(&self) -> Result<Value, CustomError> {
        match self.value.serialize() {
            Ok(v) => Ok(json!({
                "op": "not",
                "type": "Boolean",
                "args": v
            })),
            Err(e) => Err(e),
        }
    }
}

impl ToText for LogicalUnaryExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(LogicalResultType::Text, symbols)? {
            LogicalResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToBoolean for LogicalUnaryExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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

#[derive(Debug, Clone)]
enum NumberMatchExpression {
    NumberConditionExpression(
        (
            Rc<dyn ToNumber>,
            Vec<(Rc<dyn ToNumber>, Rc<dyn ToNumber>)>,
            Rc<dyn ToNumber>,
        ),
    ),
    DecimalConditionExpression(
        (
            Rc<dyn ToDecimal>,
            Vec<(Rc<dyn ToDecimal>, Rc<dyn ToNumber>)>,
            Rc<dyn ToNumber>,
        ),
    ),
    TextConditionExpression(
        (
            Rc<dyn ToText>,
            Vec<(Rc<dyn ToText>, Rc<dyn ToNumber>)>,
            Rc<dyn ToNumber>,
        ),
    ),
    BooleanConditionExpression(
        (
            Rc<dyn ToBoolean>,
            Vec<(Rc<dyn ToBoolean>, Rc<dyn ToNumber>)>,
            Rc<dyn ToNumber>,
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
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                NumberMatchResultType::Text => Ok(NumberMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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

impl ToNumber for NumberMatchExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(NumberMatchResultType::Number, symbols)? {
            NumberMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        (self as &dyn ToNumber).serialize()
    }
}

impl ToDecimal for NumberMatchExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(NumberMatchResultType::Decimal, symbols)? {
            NumberMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        (self as &dyn ToNumber).serialize()
    }
}

impl ToText for NumberMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(NumberMatchResultType::Text, symbols)? {
            NumberMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        (self as &dyn ToNumber).serialize()
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

#[derive(Debug, Clone)]
enum DecimalMatchExpression {
    NumberConditionExpression(
        (
            Rc<dyn ToNumber>,
            Vec<(Rc<dyn ToNumber>, Rc<dyn ToDecimal>)>,
            Rc<dyn ToDecimal>,
        ),
    ),
    DecimalConditionExpression(
        (
            Rc<dyn ToDecimal>,
            Vec<(Rc<dyn ToDecimal>, Rc<dyn ToDecimal>)>,
            Rc<dyn ToDecimal>,
        ),
    ),
    TextConditionExpression(
        (
            Rc<dyn ToText>,
            Vec<(Rc<dyn ToText>, Rc<dyn ToDecimal>)>,
            Rc<dyn ToDecimal>,
        ),
    ),
    BooleanConditionExpression(
        (
            Rc<dyn ToBoolean>,
            Vec<(Rc<dyn ToBoolean>, Rc<dyn ToDecimal>)>,
            Rc<dyn ToDecimal>,
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
                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                },
                DecimalMatchResultType::Decimal => Ok(DecimalMatchResult::Decimal(v)),
                DecimalMatchResultType::Text => Ok(DecimalMatchResult::Text(v.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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

impl ToNumber for DecimalMatchExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        match self.eval(DecimalMatchResultType::Number, symbols)? {
            DecimalMatchResult::Number(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToDecimal for DecimalMatchExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        match self.eval(DecimalMatchResultType::Decimal, symbols)? {
            DecimalMatchResult::Decimal(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
    }
}

impl ToText for DecimalMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
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

#[derive(Debug)]
enum TextMatchResultType {
    Text,
}

#[derive(Debug)]
enum TextMatchResult {
    Text(String),
}

#[derive(Debug, Clone)]
enum TextMatchExpression {
    NumberConditionExpression(
        (
            Rc<dyn ToNumber>,
            Vec<(Rc<dyn ToNumber>, Rc<dyn ToText>)>,
            Rc<dyn ToText>,
        ),
    ),
    DecimalConditionExpression(
        (
            Rc<dyn ToDecimal>,
            Vec<(Rc<dyn ToDecimal>, Rc<dyn ToText>)>,
            Rc<dyn ToText>,
        ),
    ),
    TextConditionExpression(
        (
            Rc<dyn ToText>,
            Vec<(Rc<dyn ToText>, Rc<dyn ToText>)>,
            Rc<dyn ToText>,
        ),
    ),
    BooleanConditionExpression(
        (
            Rc<dyn ToBoolean>,
            Vec<(Rc<dyn ToBoolean>, Rc<dyn ToText>)>,
            Rc<dyn ToText>,
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

    fn serialize(&self) -> Result<Value, CustomError> {
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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

impl ToText for TextMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(TextMatchResultType::Text, symbols)? {
            TextMatchResult::Text(v) => Ok(v),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        self.serialize()
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

#[derive(Debug, Clone)]
enum BooleanMatchExpression {
    NumberConditionExpression(
        (
            Rc<dyn ToNumber>,
            Vec<(Rc<dyn ToNumber>, Rc<dyn ToBoolean>)>,
            Rc<dyn ToBoolean>,
        ),
    ),
    DecimalConditionExpression(
        (
            Rc<dyn ToDecimal>,
            Vec<(Rc<dyn ToDecimal>, Rc<dyn ToBoolean>)>,
            Rc<dyn ToBoolean>,
        ),
    ),
    TextConditionExpression(
        (
            Rc<dyn ToText>,
            Vec<(Rc<dyn ToText>, Rc<dyn ToBoolean>)>,
            Rc<dyn ToBoolean>,
        ),
    ),
    BooleanConditionExpression(
        (
            Rc<dyn ToBoolean>,
            Vec<(Rc<dyn ToBoolean>, Rc<dyn ToBoolean>)>,
            Rc<dyn ToBoolean>,
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

    fn serialize(&self) -> Result<Value, CustomError> {
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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
                                    "type": conditional_type,
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

impl ToBoolean for BooleanMatchExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        match self.eval(BooleanMatchResultType::Boolean, symbols)? {
            BooleanMatchResult::Boolean(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        (self as &dyn ToText).serialize()
    }
}

impl ToText for BooleanMatchExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self.eval(BooleanMatchResultType::Text, symbols)? {
            BooleanMatchResult::Text(v) => Ok(v),
            _ => Err(CustomError::Message(Message::ErrUnexpected)),
        }
    }

    fn serialize(&self) -> Result<Value, CustomError> {
        (self as &dyn ToText).serialize()
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

#[derive(Debug, Clone)]
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

impl ToNumber for DotExpression {
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
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

impl ToDecimal for DotExpression {
    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
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

impl ToText for DotExpression {
    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
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

impl ToBoolean for DotExpression {
    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
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
    fn get_number(&self, symbols: &HashMap<String, Symbol>) -> Result<i32, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_number(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_number(symbols),
            LispExpression::NumberComparatorExpression(v) => err,
            LispExpression::DecimalComparatorExpression(v) => err,
            LispExpression::TextComparatorExpression(v) => err,
            LispExpression::LogicalBinaryExpression(v) => err,
            LispExpression::LogicalUnaryExpression(v) => err,
            LispExpression::NumberMatchExpression(v) => v.get_number(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_number(symbols),
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => err,
            LispExpression::DotExpression(v) => v.get_number(symbols),
        }
    }

    fn get_decimal(&self, symbols: &HashMap<String, Symbol>) -> Result<BigDecimal, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_decimal(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_decimal(symbols),
            LispExpression::NumberComparatorExpression(v) => err,
            LispExpression::DecimalComparatorExpression(v) => err,
            LispExpression::TextComparatorExpression(v) => err,
            LispExpression::LogicalBinaryExpression(v) => err,
            LispExpression::LogicalUnaryExpression(v) => err,
            LispExpression::NumberMatchExpression(v) => v.get_decimal(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_decimal(symbols),
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => err,
            LispExpression::DotExpression(v) => v.get_decimal(symbols),
        }
    }

    fn get_text(&self, symbols: &HashMap<String, Symbol>) -> Result<String, CustomError> {
        match self {
            LispExpression::NumberArithmeticExpression(v) => v.get_text(symbols),
            LispExpression::DecimalArithmeticExpression(v) => v.get_text(symbols),
            LispExpression::NumberComparatorExpression(v) => v.get_text(symbols),
            LispExpression::DecimalComparatorExpression(v) => v.get_text(symbols),
            LispExpression::TextComparatorExpression(v) => v.get_text(symbols),
            LispExpression::LogicalBinaryExpression(v) => v.get_text(symbols),
            LispExpression::LogicalUnaryExpression(v) => v.get_text(symbols),
            LispExpression::NumberMatchExpression(v) => v.get_text(symbols),
            LispExpression::DecimalMatchExpression(v) => v.get_text(symbols),
            LispExpression::TextMatchExpression(v) => v.get_text(symbols),
            LispExpression::BooleanMatchExpression(v) => v.get_text(symbols),
            LispExpression::DotExpression(v) => v.get_text(symbols),
        }
    }

    fn get_boolean(&self, symbols: &HashMap<String, Symbol>) -> Result<bool, CustomError> {
        let err = Err(CustomError::Message(Message::ErrUnexpected));
        match self {
            LispExpression::NumberArithmeticExpression(v) => err,
            LispExpression::DecimalArithmeticExpression(v) => err,
            LispExpression::NumberComparatorExpression(v) => v.get_boolean(symbols),
            LispExpression::DecimalComparatorExpression(v) => v.get_boolean(symbols),
            LispExpression::TextComparatorExpression(v) => v.get_boolean(symbols),
            LispExpression::LogicalBinaryExpression(v) => v.get_boolean(symbols),
            LispExpression::LogicalUnaryExpression(v) => v.get_boolean(symbols),
            LispExpression::NumberMatchExpression(v) => err,
            LispExpression::DecimalMatchExpression(v) => err,
            LispExpression::TextMatchExpression(v) => err,
            LispExpression::BooleanMatchExpression(v) => v.get_boolean(symbols),
            LispExpression::DotExpression(v) => v.get_boolean(symbols),
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

    fn deserialize_to_number(values: &Vec<Value>) -> Result<Vec<Rc<dyn ToNumber>>, CustomError> {
        let init: Vec<Result<Rc<dyn ToNumber>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Number(v) => match v.is_f64() {
                        true => match v.as_f64() {
                            Some(v1) => match BigDecimal::from_f64(v1) {
                                Some(v2) => Ok(Rc::new(v2)),
                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        false => match v.as_i64() {
                            Some(v1) => Ok(Rc::new(v1 as i32)),
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                    },
                    Value::String(v) => match v.parse::<i32>() {
                        Ok(v1) => Ok(Rc::new(v1)),
                        Err(_) => match v.parse::<f64>() {
                            Ok(v2) => match BigDecimal::from_f64(v2) {
                                Some(v3) => Ok(Rc::new(v3)),
                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                    },
                    Value::Object(_) => match Self::deserialize(val.clone()) {
                        Ok(v) => match v {
                            LispExpression::NumberArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::NumberMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DotExpression(v1) => Ok(Rc::new(v1)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_decimal(values: &Vec<Value>) -> Result<Vec<Rc<dyn ToDecimal>>, CustomError> {
        let init: Vec<Result<Rc<dyn ToDecimal>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Number(v) => match v.is_f64() {
                        true => match v.as_f64() {
                            Some(v1) => match BigDecimal::from_f64(v1) {
                                Some(v2) => Ok(Rc::new(v2)),
                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        false => match v.as_i64() {
                            Some(v1) => Ok(Rc::new(v1 as i32)),
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                    },
                    Value::String(v) => match v.parse::<i32>() {
                        Ok(v1) => Ok(Rc::new(v1)),
                        Err(_) => match v.parse::<f64>() {
                            Ok(v2) => match BigDecimal::from_f64(v2) {
                                Some(v3) => Ok(Rc::new(v3)),
                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                    },
                    Value::Object(_) => match Self::deserialize(val.clone()) {
                        Ok(v) => match v {
                            LispExpression::NumberArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::NumberMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DotExpression(v1) => Ok(Rc::new(v1)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_text(values: &Vec<Value>) -> Result<Vec<Rc<dyn ToText>>, CustomError> {
        let init: Vec<Result<Rc<dyn ToText>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::Number(v) => match v.is_f64() {
                        true => match v.as_f64() {
                            Some(v1) => match BigDecimal::from_f64(v1) {
                                Some(v2) => Ok(Rc::new(v2)),
                                None => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        false => match v.as_i64() {
                            Some(v1) => Ok(Rc::new(v1 as i32)),
                            None => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                    },
                    Value::String(v) => Ok(Rc::new(v.to_string())),
                    Value::Bool(v) => Ok(Rc::new(v.to_string())),
                    Value::Object(_) => match Self::deserialize(val.clone()) {
                        Ok(v) => match v {
                            LispExpression::NumberArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalArithmeticExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::NumberComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::TextComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::LogicalBinaryExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::LogicalUnaryExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::NumberMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::TextMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::BooleanMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DotExpression(v1) => Ok(Rc::new(v1)),
                        },
                        Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                });
                acc
            })
            .into_iter()
            .collect()
    }

    fn deserialize_to_boolean(values: &Vec<Value>) -> Result<Vec<Rc<dyn ToBoolean>>, CustomError> {
        let init: Vec<Result<Rc<dyn ToBoolean>, CustomError>> = vec![];
        values
            .iter()
            .fold(init, |mut acc, val| {
                acc.push(match val {
                    Value::String(v) => match v.parse::<bool>() {
                        Ok(v1) => Ok(Rc::new(v1)),
                        Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
                    },
                    Value::Bool(v) => Ok(Rc::new(*v)),
                    Value::Object(_) => match Self::deserialize(val.clone()) {
                        Ok(v) => match v {
                            LispExpression::NumberComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DecimalComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::TextComparatorExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::LogicalBinaryExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::LogicalUnaryExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::BooleanMatchExpression(v1) => Ok(Rc::new(v1)),
                            LispExpression::DotExpression(v1) => Ok(Rc::new(v1)),
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        },
                        Err(_) => Err(CustomError::Message(Message::ErrUnexpected)),
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
                                            Value::Array(v6) => match Self::deserialize_to_number(v6) {
                                                Ok(v7) => match v7.first() {
                                                    Some(v8) => match v4.as_str() {
                                                        "+" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Add((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "*" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Multiply((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "-" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Subtract((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "/" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Divide((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "%" => {
                                                            Ok(LispExpression::NumberArithmeticExpression(NumberArithmeticExpression::Modulus((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                Err(e) => Err(e),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        "Decimal" => match v3 {
                                            Value::Array(v6) => match Self::deserialize_to_decimal(v6) {
                                                Ok(v7) => match v7.first() {
                                                    Some(v8) => match v4.as_str() {
                                                        "+" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Add((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "*" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Multiply((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "-" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Subtract((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "/" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Divide((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        "%" => {
                                                            Ok(LispExpression::DecimalArithmeticExpression(DecimalArithmeticExpression::Modulus((v8.clone(), v7[1..].to_vec()))))
                                                        },
                                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                                    },
                                                    None => Err(CustomError::Message(Message::ErrUnexpected)),
                                                },
                                                Err(e) => Err(e),
                                            },
                                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                        },
                                        _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                    },
                                    _ => Err(CustomError::Message(Message::ErrUnexpected)),
                                },
                                "==" | ">=" | "<=" | ">" | "<" => {
                                    todo!()
                                }
                                "and" | "or" => {
                                    todo!()
                                }
                                "not" => {
                                    todo!()
                                }
                                "match" => {
                                    todo!()
                                }
                                // "." => {
                                //     todo!()
                                // }
                                _ => Err(CustomError::Message(Message::ErrUnexpected)),
                            },
                            _ => Err(CustomError::Message(Message::ErrUnexpected)),
                        }
                    }
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
        let expr = DecimalArithmeticExpression::Add((
            Rc::new(2),
            vec![Rc::new(BigDecimal::from_str("2.3").unwrap()), Rc::new(7)],
        ));
        assert_eq!(11, (&expr).get_number(&symbols).unwrap());
        println!("{}", (&expr as &dyn ToDecimal).serialize().unwrap());
    }

    #[test]
    fn test_decimal_arithmetic_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            BigDecimal::from_str("4.3").unwrap(),
            DecimalArithmeticExpression::Add((
                Rc::new(2),
                vec![Rc::new(BigDecimal::from_str("2.3").unwrap())]
            ))
            .get_decimal(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_number_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            true,
            NumberComparatorExpression::Equals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(2)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            NumberComparatorExpression::GreaterThanEquals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("3.3").unwrap()),
                vec![Rc::new(4)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            NumberComparatorExpression::LessThan((
                Rc::new(5),
                Rc::new(BigDecimal::from_str("3.3").unwrap()),
                vec![Rc::new(2)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            NumberComparatorExpression::GreaterThanEquals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(2)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            NumberComparatorExpression::LessThanEquals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(2)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_decimal_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            true,
            DecimalComparatorExpression::Equals((
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(BigDecimal::from_str("2.3").unwrap())]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            DecimalComparatorExpression::GreaterThanEquals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("3.3").unwrap()),
                vec![Rc::new(4)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            DecimalComparatorExpression::LessThan((
                Rc::new(5),
                Rc::new(BigDecimal::from_str("3.3").unwrap()),
                vec![Rc::new(2)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            DecimalComparatorExpression::GreaterThanEquals((
                Rc::new(2),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(3)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            true,
            DecimalComparatorExpression::LessThanEquals((
                Rc::new(4),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(1)]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_text_comparator_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            true,
            TextComparatorExpression::Equals((
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                Rc::new(BigDecimal::from_str("2.3").unwrap()),
                vec![Rc::new(BigDecimal::from_str("2.3").unwrap())]
            ))
            .get_boolean(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_logical_binary_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            true,
            LogicalBinaryExpression::And((Rc::new(true), Rc::new(true), vec![]))
                .get_boolean(&symbols)
                .unwrap()
        );
        assert_eq!(
            false,
            LogicalBinaryExpression::And((Rc::new(true), Rc::new(false), vec![]))
                .get_boolean(&symbols)
                .unwrap()
        );
        assert_eq!(
            true,
            LogicalBinaryExpression::Or((Rc::new(true), Rc::new(false), vec![]))
                .get_boolean(&symbols)
                .unwrap()
        );
        assert_eq!(
            false,
            LogicalBinaryExpression::Or((Rc::new(false), Rc::new(false), vec![]))
                .get_boolean(&symbols)
                .unwrap()
        );
    }

    #[test]
    fn test_logical_unary_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            true,
            LogicalUnaryExpression {
                value: Rc::new(false)
            }
            .get_boolean(&symbols)
            .unwrap()
        );
        assert_eq!(
            false,
            LogicalUnaryExpression {
                value: Rc::new(true)
            }
            .get_boolean(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_number_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            7,
            NumberMatchExpression::NumberConditionExpression((Rc::new(2), vec![], Rc::new(7)))
                .get_number(&symbols)
                .unwrap()
        );
        assert_eq!(
            11,
            NumberMatchExpression::NumberConditionExpression((
                Rc::new(2),
                vec![
                    (Rc::new(5), Rc::new(8)),
                    (Rc::new(2), Rc::new(11)),
                    (Rc::new(3), Rc::new(13))
                ],
                Rc::new(7)
            ))
            .get_number(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_decimal_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            BigDecimal::from_str("2.3").unwrap(),
            DecimalMatchExpression::NumberConditionExpression((
                Rc::new(2),
                vec![],
                Rc::new(BigDecimal::from_str("2.3").unwrap())
            ))
            .get_decimal(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_text_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            "2.3".to_string(),
            TextMatchExpression::NumberConditionExpression((
                Rc::new(2),
                vec![],
                Rc::new(BigDecimal::from_str("2.3").unwrap())
            ))
            .get_text(&symbols)
            .unwrap()
        );
    }

    #[test]
    fn test_boolean_match_expression() {
        let symbols: HashMap<String, Symbol> = HashMap::new();
        assert_eq!(
            false,
            BooleanMatchExpression::NumberConditionExpression((
                Rc::new(2),
                vec![],
                Rc::new(false)
            ))
            .get_boolean(&symbols)
            .unwrap()
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
        assert_eq!(
            11,
            DecimalArithmeticExpression::Add((
                Rc::new(DotExpression {
                    path: vec![String::from("x")]
                }),
                vec![
                    Rc::new(DotExpression {
                        path: vec![String::from("y")]
                    }),
                    Rc::new(DotExpression {
                        path: vec![String::from("z"), String::from("z")]
                    })
                ]
            ))
            .get_number(&symbols)
            .unwrap()
        );
    }
}
