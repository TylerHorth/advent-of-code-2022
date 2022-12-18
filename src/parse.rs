use ariadne::{Report, ReportKind, Fmt, Color, Label, Source};
use chumsky::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

fn print_errors_and_exit(errors: Vec<Simple<char>>, input: &str, day: usize) -> ! {
    let name = format!("day{}.txt", day);
    for e in errors {
        let report = Report::build(ReportKind::Error, &name, e.span().start);
        let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_label(
                    Label::new((&name, span.clone()))
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                )
                .with_label(
                    Label::new((&name, e.span()))
                        .with_message(format!(
                            "Must be closed before this: {}",
                            e.found().map(|c| c.to_string())
                                .unwrap_or("end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Unexpected => report
                .with_message(format!(
                    "{}, expected one of: {}",
                    if e.found().is_some() {
                        "Unexpected token"
                    } else {
                        "Unexpected end of input"
                    },
                    if e.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        e.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.escape_default().fg(Color::Green).to_string(),
                                None => "end of input".fg(Color::Green).to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ))
                .with_label(
                    Label::new((&name, e.span()))
                        .with_message(format!(
                            "Unexpected token: {}",
                            e.found().map(|c| c.escape_default().to_string())
                                .unwrap_or("end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                Label::new((&name, e.span()))
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            ),
        };

        report.finish().eprint((&name, Source::from(input))).unwrap()
    }

    std::process::exit(1)
}

pub fn parse_input<T>(parser: impl Parser<char, T, Error = Simple<char>>, input: &str, day: usize) -> T {
    match parser.parse(input) {
        Ok(result) => result,
        Err(errors) => print_errors_and_exit(errors, input, day)
    }
}

lazy_static! {
    static ref INTS_REGEX: Regex = Regex::new(r"-?\d+").unwrap();
}

pub fn ints(data: &str) -> Vec<i32> {
    INTS_REGEX.find_iter(data)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}