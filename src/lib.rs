use crate::TakeValue::*;
use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    str::FromStr,
};

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
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                if !args.quiet && num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" });
                }
                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                let file = BufReader::new(file);
                if let Some(num_bytes) = &args.bytes {
                    print_bytes(file, num_bytes, total_bytes)?;
                } else {
                    print_lines(file, &args.lines, total_lines)?;
                }
            }
        }
    }
    Ok(())
}

fn count_lines_bytes(filename: &str) -> Result<(u64, u64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = vec![];
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as u64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

fn print_bytes<T>(mut file: T, num_bytes: &TakeValue, total_bytes: u64) -> Result<()>
where
    T: Read + Seek,
{
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }
    Ok(())
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: u64) -> Result<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = vec![];
        while file.read_until(b'\n', &mut buf)? > 0 {
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }
    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: u64) -> Option<u64> {
    match *take_val {
        PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeNum(num) => {
            if num == 0 || total == 0 || num.max(0) as u64 > total {
                None
            } else {
                Some(if num < 0 {
                    total.saturating_sub(num.unsigned_abs())
                } else {
                    num as u64 - 1
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{count_lines_bytes, get_start_index, TakeValue, TakeValue::*};
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
    fn test_get_start_index() {
        // 空のファイル(0行/バイト)に対して+0を指定したときはNoneを返す
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // 空でないファイルに対して+0を指定したときは0を返す
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // 0行/バイトを指定した場合はNoneを返す
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // 空のファイルから行/バイトを取得するとNoneを返す
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // ファイルの行数やバイト数を超える位置を取得しようとするとNoneを返す
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // 開始行や開始バイトがファイルの行数やバイト数より小さい場合、
        // 開始行や開始バイトより1小さい値を返す
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // 開始行や開始バイトが負の場合、
        // ファイルの行数/バイト数に開始行/バイトを足した結果を返す
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // 開始行や開始バイトが負で、足した結果が0より小さい場合、
        // ファイル全体を表示するために0を返す
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
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
