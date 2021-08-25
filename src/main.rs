// Arithmetic Ops

enum ArithmeticResult {
    Number,
    Decimal,
    Text,
}

enum ArithmeticArgType {
    Number,
    Decimal,
}

enum ArithmeticArg {
    Number(i32),
    Decimal(f64),
}

// Comparator Ops

enum ComparatorResult {
    Boolean,
    Text,
}

enum ComparatorArgType {
    Number,
    Decimal,
    Text,
}

enum ComparatorArg {
    Number(i32),
    Decimal(f64),
    Text(String),
}

// Logical Ops

enum LogicalResult {
    Boolean,
    Text,
}

enum LogicalArgType {
    Boolean,
}

enum LogicalArg {
    Boolean(bool),
}

// Control Flow Ops

enum ControlFlowResult {
    Number,
    Decimal,
    Boolean,
    Text,
}

enum ControlFlowArgType {
    Number,
    Decimal,
    Boolean,
    Text,
}

enum ControlFlowArg {
    Boolean(bool, LispExpression, LispExpression),
    Expression(LispExpression, LispExpression, LispExpression),
}

// Note. In some places, tuples or slices could be used here instead of arrays

enum LispExpression {
    Plus {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Multiply {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Subtract {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Divide {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Power {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Modulus {
        result_type: ArithmeticResult,
        types: (ArithmeticArgType, Vec<ArithmeticArgType>),
        args: (ArithmeticArg, Vec<ArithmeticArg>),
    },
    Equals {
        result_type: ComparatorResult,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: (ComparatorArg, ComparatorArg, Vec<ComparatorArg>),
    },
    GreaterThan {
        result_type: ComparatorResult,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: (ComparatorArg, ComparatorArg, Vec<ComparatorArg>),
    },
    LessThan {
        result_type: ComparatorResult,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: (ComparatorArg, ComparatorArg, Vec<ComparatorArg>),
    },
    GreaterThanEquals {
        result_type: ComparatorResult,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: (ComparatorArg, ComparatorArg, Vec<ComparatorArg>),
    },
    LessThanEquals {
        result_type: ComparatorResult,
        types: (ComparatorArgType, Vec<ComparatorArgType>),
        args: (ComparatorArg, ComparatorArg, Vec<ComparatorArg>),
    },
    And {
        result_type: LogicalResult,
        args: (LogicalArg, LogicalArg, Vec<LogicalArg>),
    },
    Or {
        result_type: LogicalResult,
        args: (LogicalArg, LogicalArg, Vec<LogicalArg>),
    },
    Not {
        result_type: LogicalResult,
        args: LogicalArg,
    },
    If {
        result_type: ControlFlowResult,
        types: (ControlFlowArgType, Vec<ControlFlowArgType>),
        args: Box<ControlFlowArg>,
    },
}

fn main() {
    println!("Hello, world!");
}
