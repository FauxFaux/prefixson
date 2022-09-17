use anyhow::Result;
use std::collections::HashMap;

pub enum Simple {
    Str(String),
    Int(i64),
}

pub enum Obj {
    Hash(HashMap<Simple, Obj>),
    Simple(Simple),
}

pub fn parse(msg: &str) -> Result<Obj> {
    Ok(Obj::Simple(Simple::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        parse(
            r#"a:12:
{
s:12:"cfdb7_status";s:4:"read";
s:16:"your-team-number";s:10:"F MANGUT ";
s:19:"radio-permit-number";a:1:{i:0;s:3:"yes";}
s:7:"your-pn";s:3:"359";
s:17:"your-house-number";s:11:"SEDGEBROOK ";
s:13:"your-postcode";s:7:"SN3 6EZ";
s:14:"menu-team-size";a:1:{i:0;s:1:"4";}
s:22:"radio-correct-drawings";a:1:{i:0;s:3:"yes";}
s:15:"radio-safe-digs";a:1:{i:0;s:3:"yes";}
s:15:"radio-materials";a:1:{i:0;s:3:"yes";}
s:12:"your-message";s:0:"";
s:20:"file-photocfdb7_file";s:63:"1663140629-file-photo-B6F8CFA5-6674-4058-B37C.jpeg";
}"#,
        )
        .unwrap();
        Ok(())
    }
}
