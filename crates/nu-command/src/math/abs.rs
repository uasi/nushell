use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EvaluationContext};
use nu_protocol::{Example, ShellError, Signature, Span, Value};

pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "math abs"
    }

    fn signature(&self) -> Signature {
        Signature::build("math abs")
    }

    fn usage(&self) -> &str {
        "Returns absolute values of a list of numbers"
    }

    fn run(
        &self,
        _context: &EvaluationContext,
        call: &Call,
        input: Value,
    ) -> Result<Value, ShellError> {
        let head = call.head;
        match input {
            Value::List { vals, span } => Ok(Value::List {
                vals: vals
                    .into_iter()
                    .map(move |val| abs_helper(val, head))
                    .collect(),
                span,
            }),
            other => match abs_helper(other, head) {
                Value::Error { error } => Err(error),
                ok => Ok(ok),
            },
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Get absolute of each value in a list of numbers",
            example: "[-50 -100.0 25] | math abs",
            result: Some(Value::List {
                vals: vec![
                    Value::test_int(50),
                    Value::Float {
                        val: 100.0,
                        span: Span::unknown(),
                    },
                    Value::test_int(25),
                ],
                span: Span::unknown(),
            }),
        }]
    }
}

fn abs_helper(val: Value, head: Span) -> Value {
    match val {
        Value::Int { val, span } => Value::int(val.abs(), span),
        Value::Float { val, span } => Value::Float {
            val: val.abs(),
            span,
        },
        Value::Duration { val, span } => Value::Duration {
            val: val.abs(),
            span,
        },
        _ => Value::Error {
            error: ShellError::UnsupportedInput(
                String::from("Only numerical values are supported"),
                head,
            ),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
