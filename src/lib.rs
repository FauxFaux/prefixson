use anyhow::{anyhow, ensure, Result};
use std::collections::HashMap;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until1, take_while, take_while1};
use nom::combinator::{map, opt};
use nom::multi::count;
use nom::IResult;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Simple {
    Str(String),
    Int(i64),
}

#[derive(Debug)]
pub enum Obj {
    Hash(HashMap<Simple, Obj>),
    Simple(Simple),
}

fn drop_ws(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    Ok((input, ()))
}

fn parse_len(input: &str) -> IResult<&str, usize> {
    let (input, chars) = take_until1(":")(input)?;
    let len = usize::from_str(chars).expect("TODO: unvalidated parse error");
    let (input, _) = tag(":")(input)?;
    Ok((input, len))
}

fn parse_int(input: &str) -> IResult<&str, Simple> {
    let (input, _) = tag("i:")(input)?;
    let (input, chars) = take_while1(|c: char| c.is_numeric())(input)?;
    let val = i64::from_str(chars).expect("TODO: unvalidated parse error");
    Ok((input, Simple::Int(val)))
}

fn parse_string(input: &str) -> IResult<&str, Simple> {
    let (input, _) = tag("s:")(input)?;
    let (input, len) = parse_len(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, value) = take(len)(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Simple::Str(value.to_string())))
}

fn parse_hash(input: &str) -> IResult<&str, Obj> {
    let (input, _) = tag("a:")(input)?;
    let (input, len) = parse_len(input)?;
    let (input, _) = drop_ws(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = drop_ws(input)?;
    let (input, pairs) = count(parse_kv, len)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = drop_ws(input)?;
    Ok((input, Obj::Hash(pairs.into_iter().collect())))
}

fn parse_kv(input: &str) -> IResult<&str, (Simple, Obj)> {
    let (input, key) = parse_simple(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, value) = parse_obj(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    let (input, _) = drop_ws(input)?;

    Ok((input, (key, value)))
}

fn parse_simple(input: &str) -> IResult<&str, Simple> {
    alt((parse_int, parse_string))(input)
}

fn parse_obj(msg: &str) -> IResult<&str, Obj> {
    alt((map(parse_simple, |s| Obj::Simple(s)), parse_hash))(msg)
}

pub fn parse(msg: &str) -> Result<Obj> {
    let (rest, obj) = parse_obj(msg).map_err(|e| anyhow!("{:?}", e))?;
    ensure!(rest.is_empty(), "rest should be empty: {:?}", rest);
    Ok(obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() -> Result<()> {
        parse(r#"a:1:{i:0;s:3:"yes";}"#).unwrap();
        Ok(())
    }

    #[test]
    fn big() -> Result<()> {
        let obj = parse(
            r#"a:12:
{
s:12:"cfdb7_status";s:4:"read";
s:16:"your-team-number";s:10:"F MANGUT  ";
s:19:"radio-permit-number";a:1:{i:0;s:3:"yes";}
s:7:"your-pn";s:3:"359";
s:17:"your-house-number";s:11:"SEDGEBROOK ";
s:13:"your-postcode";s:7:"SN3 6EZ";
s:14:"menu-team-size";a:1:{i:0;s:1:"4";}
s:22:"radio-correct-drawings";a:1:{i:0;s:3:"yes";}
s:15:"radio-safe-digs";a:1:{i:0;s:3:"yes";}
s:15:"radio-materials";a:1:{i:0;s:3:"yes";}
s:12:"your-message";s:0:"";
s:20:"file-photocfdb7_file";s:50:"1663140629-file-photo-B6F8CFA5-6674-4058-B37C.jpeg";
}"#,
        )?;
        dbg!(obj);
        Ok(())
    }
}
