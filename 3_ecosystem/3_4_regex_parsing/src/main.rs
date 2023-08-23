use once_cell::sync::Lazy;
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

fn main() {
    let result = parse("a^+#043.8?", Mode::Pest);
    println!("{:?}", result);
}

enum Mode {
    Regex,
    Pest,
}
#[derive(Parser)]
#[grammar = "format_spec.pest"]
struct FormatSpecParser;

static FORMAT_SPEC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?<sign>[+-])?#?0?(?<width>\d*)(?:\.(?<precision>(?<precision_number>\d+)\$?|\*))?",
    )
    .unwrap()
});

fn parse(input: &str, mode: Mode) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    match mode {
        Mode::Regex => {
            let mut sign = None;
            let mut width = None;
            let mut precision = None;

            for caps in FORMAT_SPEC_REGEX.captures_iter(input) {
                if let Some(sign_match) = caps.name("sign") {
                    match sign_match.as_str() {
                        "+" => sign = Some(Sign::Plus),
                        "-" => sign = Some(Sign::Minus),
                        _ => (),
                    };
                }

                if let Some(width_match) = caps.name("width") {
                    match width_match.as_str() {
                        "" => (),
                        _ => width = Some(width_match.as_str().parse().unwrap()),
                    };
                }

                if let Some(precision_match) = caps.name("precision") {
                    match precision_match.as_str() {
                        "" => (),
                        "*" => precision = Some(Precision::Asterisk),
                        precision_match => {
                            let precision_number = caps
                                .name("precision_number")
                                .unwrap()
                                .as_str()
                                .parse()
                                .unwrap();

                            precision = if precision_match.contains('$') {
                                Some(Precision::Argument(precision_number))
                            } else {
                                Some(Precision::Integer(precision_number))
                            };
                        }
                    }
                }
            }

            (sign, width, precision)
        }
        Mode::Pest => {
            let parse = FormatSpecParser::parse(Rule::format_spec, input).unwrap();

            let mut sign = None;
            let mut width = None;
            let mut precision = None;

            for pair in parse {
                if pair.as_rule() == Rule::format_spec {
                    for rule in pair.into_inner() {
                        match rule.as_rule() {
                            Rule::sign => {
                                match rule.as_str() {
                                    "+" => sign = Some(Sign::Plus),
                                    "-" => sign = Some(Sign::Minus),
                                    _ => (),
                                };
                            }
                            Rule::width => {
                                width = Some(rule.as_str().parse().unwrap());
                            }
                            Rule::precision => {
                                let rule_str = rule.as_str();
                                if rule_str == ".*" {
                                    precision = Some(Precision::Asterisk);
                                } else {
                                    for inner_rule in rule.into_inner() {
                                        if inner_rule.as_rule() == Rule::integer {
                                            let precision_number =
                                                inner_rule.as_str().parse().unwrap();

                                            if rule_str.contains('$') {
                                                precision =
                                                    Some(Precision::Argument(precision_number));
                                            } else {
                                                precision =
                                                    Some(Precision::Integer(precision_number));
                                            }
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            (sign, width, precision)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input, Mode::Regex);
            assert_eq!(sign, expected);

            let (sign, ..) = parse(input, Mode::Pest);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input, Mode::Regex);
            assert_eq!(width, expected);

            let (_, width, _) = parse(input, Mode::Pest);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input, Mode::Regex);
            assert_eq!(precision, expected);

            let (_, _, precision) = parse(input, Mode::Pest);
            assert_eq!(precision, expected);
        }
    }
}
