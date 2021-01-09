use std::{collections::HashMap, fs::{File, read_to_string}, iter::Peekable, str::Chars};

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

    parseElement(&mut iter)
}

fn parseElement(iter: &mut Peekable<Chars>) -> Option<Value> {
    skipWhitespace(iter);
    let ret = parseValue(iter);
    skipWhitespace(iter);

    ret
}

fn parseValue(iter: &mut Peekable<Chars>) -> Option<Value> {
    match iter.peek() {
        Some('"') => Some(Value::String(parseString(iter)?)),
        Some(c) if c.is_numeric() || *c == '-' => parseNumber(iter),
        Some('{') => parseObject(iter),
        Some('[') => parseArray(iter),
        Some('t') => parseTrue(iter),
        Some('f') => parseFalse(iter),
        Some('n') => parseNull(iter),
        _ => None,
    }
}

fn parseString(iter: &mut Peekable<Chars>) -> Option<String> {
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

fn parseNumber(iter: &mut Peekable<Chars>) -> Option<Value> {
    let int = parseInteger(iter)?;
    let fraction = parseFraction(iter)?;
    let exponent = parseExponent(iter)?;

    let value = (int as f64) * 10_f64.powi(exponent);
    Some(Value::Number(value))
}

fn parseInteger(iter: &mut Peekable<Chars>) -> Option<i32> {
    let c = iter.peek()?;

    let mut sign = 1;
    if *c == '-' {
        sign = -1;
        iter.next();
    }

    let num = parseDigits(iter)?;

    Some(sign * (num as i32))
}

fn parseFraction(iter: &mut Peekable<Chars>) -> Option<u32> {
    match iter.peek() {
        Some('.') => {
            iter.next();
            parseDigits(iter)
        }
        _ => Some(0),
    }
}

fn parseDigits(iter: &mut Peekable<Chars>) -> Option<u32> {
    match iter.peek() {
        Some(c) if c.is_numeric() => {}
        _ => return None,
    }

    let mut num = 0;

    while let Some(c) = iter.peek().cloned() {
        if (c.is_numeric()) {
            iter.next();
            num = 10 * num + c.to_digit(10).unwrap();
        } else {
            break;
        }
    }

    Some(num)
}

fn parseExponent(iter: &mut Peekable<Chars>) -> Option<i32> {
    match iter.peek() {
        Some('e') | Some('E') => {
            iter.next();
            let sign = parseSign(iter)?;
            let num = parseDigits(iter)?;
            Some(sign * (num as i32))
        }
        _ => Some(0),
    }
}

fn parseSign(iter: &mut Peekable<Chars>) -> Option<i32> {
    match iter.peek() {
        Some('+') => Some(1),
        Some('-') => Some(-1),
        _ => Some(1),
    }
}

fn parseObject(iter: &mut Peekable<Chars>) -> Option<Value> {
    assert!(iter.next()? == '{');
    skipWhitespace(iter);
    let mut map = HashMap::new();

    while let Some(c) = iter.peek().cloned() {
        if c == '}' {
            iter.next();
            return Some(Value::Object(map));
        } else {
            if c == ',' {
                iter.next();
            }

            let (str, val) = parseMember(iter)?;
            map.insert(str, Box::new(val));
            skipWhitespace(iter);
        }
    }

    None
}

fn parseMember(iter: &mut Peekable<Chars>) -> Option<(String, Value)> {
    skipWhitespace(iter);
    let s = parseString(iter)?;
    skipWhitespace(iter);
    if iter.next()? != ':' {
        return None;
    }
    skipWhitespace(iter);
    let v = parseElement(iter)?;

    Some((s, v))
}

fn parseArray(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? != '[' {
        return None;
    }

    skipWhitespace(iter);

    let mut vec = Vec::new();

    while let Some(c) = iter.peek().cloned() {
        if c == ']' {
            iter.next();
            return Some(Value::Array(vec));
        } else {
            if c == ',' {
                iter.next();
            }
            vec.push(Box::new(parseElement(iter)?));
            skipWhitespace(iter);
        }
    }

    None
}

fn parseTrue(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? == 't' && iter.next()? == 'r' && iter.next()? == 'u' && iter.next()? == 'e' {
        Some(Value::True)
    } else {
        None
    }
}

fn parseFalse(iter: &mut Peekable<Chars>) -> Option<Value> {
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

fn parseNull(iter: &mut Peekable<Chars>) -> Option<Value> {
    if iter.next()? == 'n' && iter.next()? == 'u' && iter.next()? == 'l' && iter.next()? == 'l' {
        Some(Value::Null)
    } else {
        None
    }
}

fn skipWhitespace(iter: &mut Peekable<Chars>) {
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
