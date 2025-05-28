use crate::ast::Value;
use crate::intpt::Env;

pub fn load_math(env: &mut Env) {
    // Square function (sq)
    env.set(
        "sq".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("Square requires exactly one argument");
            } else {
                match &args[0] {
                    Value::Int(x) => Value::Int(x * x),
                    Value::Float(x) => Value::Float(x * x),
                    _ => panic!("Type error in square function"),
                }
            }
        }),
    );

    // Square root function (sqrt)
    env.set(
        "sqrt".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("Square root requires exactly one argument");
            } else {
                match &args[0] {
                    Value::Int(x) => {
                        if *x < 0 {
                            panic!("Cannot compute square root of negative number");
                        }
                        Value::Float((*x as f64).sqrt())
                    }
                    Value::Float(x) => {
                        if *x < 0.0 {
                            panic!("Cannot compute square root of negative number");
                        }
                        Value::Float(x.sqrt())
                    }
                    _ => panic!("Type error in square root function"),
                }
            }
        }),
    );

    // Floating-point absolute value function (fabs)
    env.set(
        "abs".to_string(),
        Value::Func(|args| {
            if args.len() != 1 {
                panic!("Absolute value requires exactly one argument");
            } else {
                match &args[0] {
                    Value::Int(x) => Value::Int(x.abs()),
                    Value::Float(x) => Value::Float(x.abs()),
                    _ => panic!("Type error in absolute value function"),
                }
            }
        }),
    );
}
