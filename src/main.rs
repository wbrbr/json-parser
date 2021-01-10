use std::{collections::HashMap, fs::{read_to_string}, iter::Peekable, str::Chars};
#[derive(Debug)]
enum Value {
    Object(HashMap<String, Box<Value>>),
    Array(Vec<Box<Value>>),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

fn parse(input: String) -> Option<Value> {
    let mut iter = input.chars().peekable();

    parse_element(&mut iter)
}

fn parse_element(iter: &mut Peekable<Chars>) -> Option<Value> {
    skip_whitespace(iter);
    let ret = parse_value(iter);
    skip_whitespace(iter);

    ret
}

fn parse_value(iter: &mut Peekable<Chars>) -> Option<Value> {
    match iter.peek() {
        Some('"') => Some(Value::String(parse_string(iter)?)),
        Some(c) if c.is_numeric() || *c == '-' => parse_number(iter),
        Some('{') => parse_object(iter),
        Some('[') => parse_array(iter),
        Some('t') => parse_true(iter),
        Some('f') => parse_false(iter),
        Some('n') => parse_null(iter),
        _ => None,
    }
}

fn parse_string(iter: &mut Peekable<Chars>) -> Option<String> {
    if iter.next()? != '"' {
        return None;
    }
    let mut str = String::new();
    while let Some(c) = iter.next() {
        if c == '"' {
            return Some(str);
        } else if c == '\\' {
            match iter.next()? {
                '"' => str.push('"'),
                '\\' => str.push('\\'),
                '/' => str.push('/'),
                'b' => str.push('\x08'),
                'f' => str.push('\x12'),
                'n' => str.push('\n'),
                'r' => str.push('\r'),
                't' => str.push('\t'),
                'u' => return None, // TODO: implement this
                _ => return None
            }
        } else {
            str.push(c);
        }
    }
    None
}

fn parse_number(iter: &mut Peekable<Chars>) -> Option<Value> {
    let int = parse_integer(iter)?;
    let fraction = parse_fraction(iter)?;
    let exponent = parse_exponent(iter)?;

    let value = ((int as f64) + fraction) * 10_f64.powi(exponent);
    Some(Value::Number(value))
}

fn parse_integer(iter: &mut Peekable<Chars>) -> Option<i32> {
    let c = iter.peek()?;

    let mut sign = 1;
    if *c == '-' {
        sign = -1;
        iter.next();
    }

    let num = parse_digits(iter)?;

    Some(sign * (num as i32))
}

fn parse_fraction(iter: &mut Peekable<Chars>) -> Option<f64> {
    match iter.peek() {
        Some('.') => {
            let mut s = String::from("0.");
            iter.next();

            match iter.peek() {
                Some(c) if c.is_numeric() => {},
                _ => return None
            }

            while let Some(c) = iter.peek().cloned() {
                if c.is_numeric() {
                    iter.next();
                    s.push(c);
                } else {
                    break;
                }
            }

            // should be a valid float. otherwise this is a bug (or overflow maybe ?)
            let val: f64 = s.parse().unwrap();
            Some(val)

        }
        _ => Some(0.),
    }
}

fn parse_digits(iter: &mut Peekable<Chars>) -> Option<u32> {
    match iter.peek() {
        Some(c) if c.is_numeric() => {}
        _ => return None,
    }

    let mut num = 0;

    while let Some(c) = iter.peek().cloned() {
        if c.is_numeric() {
            iter.next();
            num = 10 * num + c.to_digit(10).unwrap();
        } else {
            break;
        }
    }

    Some(num)
}

fn parse_exponent(iter: &mut Peekable<Chars>) -> Option<i32> {
    match iter.peek() {
        Some('e') | Some('E') => {
            iter.next();
            let sign = parse_sign(iter)?;
            let num = parse_digits(iter)?;
            Some(sign * (num as i32))
        }
        _ => Some(0),
    }
}

fn parse_sign(iter: &mut Peekable<Chars>) -> Option<i32> {
    match iter.peek() {
        Some('+') => Some(1),
        Some('-') => Some(-1),
        _ => Some(1),
    }
}

fn parse_object(iter: &mut Peekable<Chars>) -> Option<Value> {
    assert!(iter.next()? == '{');
    skip_whitespace(iter);
    let mut map = HashMap::new();

    while let Some(c) = iter.peek().cloned() {
        if c == '}' {
            iter.next();
            return Some(Value::Object(map));
        } else {
            if c == ',' {
                iter.next();
            }

            let (str, val) = parse_member(iter)?;
            map.insert(str, Box::new(val));
            skip_whitespace(iter);
        }
    }

    None
}

fn parse_member(iter: &mut Peekable<Chars>) -> Option<(String, Value)> {
    skip_whitespace(iter);
    let s = parse_string(iter)?;
    skip_whitespace(iter);
    if iter.next()? != ':' {
        return None;
    }
    skip_whitespace(iter);
    let v = parse_element(iter)?;

    Some((s, v))
}

fn parse_array(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? != '[' {
        return None;
    }

    skip_whitespace(iter);

    let mut vec = Vec::new();

    while let Some(c) = iter.peek().cloned() {
        if c == ']' {
            iter.next();
            return Some(Value::Array(vec));
        } else {
            if c == ',' {
                iter.next();
            }
            vec.push(Box::new(parse_element(iter)?));
            skip_whitespace(iter);
        }
    }

    None
}

fn parse_true(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? == 't' && iter.next()? == 'r' && iter.next()? == 'u' && iter.next()? == 'e' {
        Some(Value::True)
    } else {
        None
    }
}

fn parse_false(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? == 'f'
        && iter.next()? == 'a'
        && iter.next()? == 'l'
        && iter.next()? == 's'
        && iter.next()? == 'e'
    {
        Some(Value::False)
    } else {
        None
    }
}

fn parse_null(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? == 'n' && iter.next()? == 'u' && iter.next()? == 'l' && iter.next()? == 'l' {
        Some(Value::Null)
    } else {
        None
    }
}

fn skip_whitespace(iter: &mut Peekable<Chars>) {
    while let Some(c) = iter.peek() {
        if c.is_whitespace() {
            iter.next();
        } else {
            break;
        }
    }
}

fn read_json_file(path: &str) -> Option<Value> {
    parse(read_to_string(path).ok()?)
}

fn main() {
    println!(
        "{:?}",
        read_json_file("test.json")
    );
}
