use crate::TakeValue::*;
use anyhow::Result;
use clap::Parser;
use std::{fs::File, str::FromStr};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", required = true)]
    files: Vec<String>,

    #[arg(short = 'n', long, help = "Number of lines", default_value = "10")]
    lines: TakeValue,

    #[arg(short = 'c', long, conflicts_with = "lines", help = "Number of bytes")]
    bytes: Option<TakeValue>,

    #[arg(short, long, help = "Suppress headers")]
    quiet: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

impl FromStr for TakeValue {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s
            .starts_with(['+', '-'])
            .then(|| s.parse())
            .unwrap_or_else(|| s.parse().map(i64::wrapping_neg))?;
        if num == 0 && s.starts_with('+') {
            Ok(PlusZero)
        } else {
            Ok(TakeNum(num))
        }
    }
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    for filename in &args.files {
        match File::open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(_) => {
                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                println!("{filename} has {total_lines} lines and {total_bytes} bytes",);
            }
        }
    }
    Ok(())
}

fn count_lines_bytes(filename: &str) -> Result<(i64, i64)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{count_lines_bytes, TakeValue, TakeValue::*};
    use std::str::FromStr;

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }

    #[test]
    fn test_parse_num() {
        // すべての整数は負の数として解釈される必要がある
        let res = TakeValue::from_str("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // 先頭に「+」が付いている場合は正の数として解釈される必要がある
        let res = TakeValue::from_str("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // 明示的に「-」が付いている場合は負の数として解釈される必要がある
        let res = TakeValue::from_str("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // ゼロはゼロのまま
        let res = TakeValue::from_str("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // プラスゼロは特別扱い
        let res = TakeValue::from_str("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // 境界値のテスト
        let res = TakeValue::from_str(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = TakeValue::from_str(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = TakeValue::from_str(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = TakeValue::from_str(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // 浮動小数点数は無効
        let res = TakeValue::from_str("3.14");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "invalid digit found in string"
        );

        // 整数でない文字列は無効
        let res = TakeValue::from_str("foo");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
