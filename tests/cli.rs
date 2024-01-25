use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io::Read;

const PRG: &str = "tailr";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn dies_bad_bytes() -> Result<()> {
    let bad = random_string();
    let expected = format!("error: invalid value '{bad}' for '--bytes <BYTES>'");
    Command::cargo_bin(PRG)?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_lines() -> Result<()> {
    let bad = random_string();
    let expected = format!("error: invalid value '{bad}' for '--lines <LINES>'");
    Command::cargo_bin(PRG)?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bytes_and_lines() -> Result<()> {
    let msg = "the argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args([ONE, &bad, TWO])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(predicate::eq(expected.as_bytes() as &[u8]));

    Ok(())
}

#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n0() -> Result<()> {
    run(&[EMPTY, "-n", "0"], "tests/expected/empty.txt.n0.out")
}

#[test]
fn empty_n1() -> Result<()> {
    run(&[EMPTY, "-n", "1"], "tests/expected/empty.txt.n1.out")
}

#[test]
fn empty_n_minus_1() -> Result<()> {
    run(&[EMPTY, "-n=-1"], "tests/expected/empty.txt.n1.out")
}

#[test]
fn empty_n3() -> Result<()> {
    run(&[EMPTY, "-n", "3"], "tests/expected/empty.txt.n3.out")
}

#[test]
fn empty_n_minus_3() -> Result<()> {
    run(&[EMPTY, "-n=-3"], "tests/expected/empty.txt.n3.out")
}

#[test]
fn empty_n4() -> Result<()> {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_n200() -> Result<()> {
    run(&[EMPTY, "-n", "200"], "tests/expected/empty.txt.n200.out")
}

#[test]
fn empty_n_minus_200() -> Result<()> {
    run(&[EMPTY, "-n=-200"], "tests/expected/empty.txt.n200.out")
}

#[test]
fn empty_n_minus_4() -> Result<()> {
    run(&[EMPTY, "-n=-4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_n_plus_0() -> Result<()> {
    run(&[EMPTY, "-n", "+0"], "tests/expected/empty.txt.n+0.out")
}

#[test]
fn empty_n_plus_1() -> Result<()> {
    run(&[EMPTY, "-n", "+1"], "tests/expected/empty.txt.n+1.out")
}

#[test]
fn empty_n_plus_2() -> Result<()> {
    run(&[EMPTY, "-n", "+2"], "tests/expected/empty.txt.n+2.out")
}

#[test]
fn empty_c3() -> Result<()> {
    run(&[EMPTY, "-c", "3"], "tests/expected/empty.txt.c3.out")
}

#[test]
fn empty_c_minus_3() -> Result<()> {
    run(&[EMPTY, "-c=-3"], "tests/expected/empty.txt.c3.out")
}

#[test]
fn empty_c8() -> Result<()> {
    run(&[EMPTY, "-c", "8"], "tests/expected/empty.txt.c8.out")
}

#[test]
fn empty_c_minus_8() -> Result<()> {
    run(&[EMPTY, "-c=8"], "tests/expected/empty.txt.c8.out")
}

#[test]
fn empty_c12() -> Result<()> {
    run(&[EMPTY, "-c", "12"], "tests/expected/empty.txt.c12.out")
}

#[test]
fn empty_c_minus_12() -> Result<()> {
    run(&[EMPTY, "-c=-12"], "tests/expected/empty.txt.c12.out")
}

#[test]
fn empty_c200() -> Result<()> {
    run(&[EMPTY, "-c", "200"], "tests/expected/empty.txt.c200.out")
}

#[test]
fn empty_c_minus_200() -> Result<()> {
    run(&[EMPTY, "-c=-200"], "tests/expected/empty.txt.c200.out")
}

#[test]
fn empty_c_plus_0() -> Result<()> {
    run(&[EMPTY, "-c", "+0"], "tests/expected/empty.txt.c+0.out")
}

#[test]
fn empty_c_plus_1() -> Result<()> {
    run(&[EMPTY, "-c", "+1"], "tests/expected/empty.txt.c+1.out")
}

#[test]
fn empty_c_plus_2() -> Result<()> {
    run(&[EMPTY, "-c", "+2"], "tests/expected/empty.txt.c+2.out")
}

#[test]
fn one() -> Result<()> {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n0() -> Result<()> {
    run(&[ONE, "-n", "0"], "tests/expected/one.txt.n0.out")
}

#[test]
fn one_n1() -> Result<()> {
    run(&[ONE, "-n", "1"], "tests/expected/one.txt.n1.out")
}

#[test]
fn one_n_minus_1() -> Result<()> {
    run(&[ONE, "-n=-1"], "tests/expected/one.txt.n1.out")
}

#[test]
fn one_n3() -> Result<()> {
    run(&[ONE, "-n", "3"], "tests/expected/one.txt.n3.out")
}

#[test]
fn one_n_minus_3() -> Result<()> {
    run(&[ONE, "-n=-3"], "tests/expected/one.txt.n3.out")
}

#[test]
fn one_n4() -> Result<()> {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_n_minus_4() -> Result<()> {
    run(&[ONE, "-n=-4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_n200() -> Result<()> {
    run(&[ONE, "-n", "200"], "tests/expected/one.txt.n200.out")
}

#[test]
fn one_n_minus_200() -> Result<()> {
    run(&[ONE, "-n=-200"], "tests/expected/one.txt.n200.out")
}

#[test]
fn one_n_plus_0() -> Result<()> {
    run(&[ONE, "-n", "+0"], "tests/expected/one.txt.n+0.out")
}

#[test]
fn one_n_plus_1() -> Result<()> {
    run(&[ONE, "-n", "+1"], "tests/expected/one.txt.n+1.out")
}

#[test]
fn one_n_plus_2() -> Result<()> {
    run(&[ONE, "-n", "+2"], "tests/expected/one.txt.n+2.out")
}

#[test]
fn one_c3() -> Result<()> {
    run(&[ONE, "-c", "3"], "tests/expected/one.txt.c3.out")
}

#[test]
fn one_c_minus_3() -> Result<()> {
    run(&[ONE, "-c=-3"], "tests/expected/one.txt.c3.out")
}

#[test]
fn one_c8() -> Result<()> {
    run(&[ONE, "-c", "8"], "tests/expected/one.txt.c8.out")
}

#[test]
fn one_c_minus_8() -> Result<()> {
    run(&[ONE, "-c=8"], "tests/expected/one.txt.c8.out")
}

#[test]
fn one_c12() -> Result<()> {
    run(&[ONE, "-c", "12"], "tests/expected/one.txt.c12.out")
}

#[test]
fn one_c_minus_12() -> Result<()> {
    run(&[ONE, "-c=-12"], "tests/expected/one.txt.c12.out")
}

#[test]
fn one_c200() -> Result<()> {
    run(&[ONE, "-c", "200"], "tests/expected/one.txt.c200.out")
}

#[test]
fn one_c_minus_200() -> Result<()> {
    run(&[ONE, "-c=-200"], "tests/expected/one.txt.c200.out")
}

#[test]
fn one_c_plus_0() -> Result<()> {
    run(&[ONE, "-c", "+0"], "tests/expected/one.txt.c+0.out")
}

#[test]
fn one_c_plus_1() -> Result<()> {
    run(&[ONE, "-c", "+1"], "tests/expected/one.txt.c+1.out")
}

#[test]
fn one_c_plus_2() -> Result<()> {
    run(&[ONE, "-c", "+2"], "tests/expected/one.txt.c+2.out")
}

#[test]
fn two() -> Result<()> {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n0() -> Result<()> {
    run(&[TWO, "-n", "0"], "tests/expected/two.txt.n0.out")
}

#[test]
fn two_n1() -> Result<()> {
    run(&[TWO, "-n", "1"], "tests/expected/two.txt.n1.out")
}

#[test]
fn two_n_minus_1() -> Result<()> {
    run(&[TWO, "-n=-1"], "tests/expected/two.txt.n1.out")
}

#[test]
fn two_n3() -> Result<()> {
    run(&[TWO, "-n", "3"], "tests/expected/two.txt.n3.out")
}

#[test]
fn two_n_minus_3() -> Result<()> {
    run(&[TWO, "-n=-3"], "tests/expected/two.txt.n3.out")
}

#[test]
fn two_n4() -> Result<()> {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_n_minus_4() -> Result<()> {
    run(&[TWO, "-n=-4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_n200() -> Result<()> {
    run(&[TWO, "-n", "200"], "tests/expected/two.txt.n200.out")
}

#[test]
fn two_n_minus_200() -> Result<()> {
    run(&[TWO, "-n=-200"], "tests/expected/two.txt.n200.out")
}

#[test]
fn two_n_plus_0() -> Result<()> {
    run(&[TWO, "-n", "+0"], "tests/expected/two.txt.n+0.out")
}

#[test]
fn two_n_plus_1() -> Result<()> {
    run(&[TWO, "-n", "+1"], "tests/expected/two.txt.n+1.out")
}

#[test]
fn two_n_plus_2() -> Result<()> {
    run(&[TWO, "-n", "+2"], "tests/expected/two.txt.n+2.out")
}

#[test]
fn two_c3() -> Result<()> {
    run(&[TWO, "-c", "3"], "tests/expected/two.txt.c3.out")
}

#[test]
fn two_c_minus_3() -> Result<()> {
    run(&[TWO, "-c=-3"], "tests/expected/two.txt.c3.out")
}

#[test]
fn two_c8() -> Result<()> {
    run(&[TWO, "-c", "8"], "tests/expected/two.txt.c8.out")
}

#[test]
fn two_c_minus_8() -> Result<()> {
    run(&[TWO, "-c=8"], "tests/expected/two.txt.c8.out")
}

#[test]
fn two_c12() -> Result<()> {
    run(&[TWO, "-c", "12"], "tests/expected/two.txt.c12.out")
}

#[test]
fn two_c_minus_12() -> Result<()> {
    run(&[TWO, "-c=-12"], "tests/expected/two.txt.c12.out")
}

#[test]
fn two_c200() -> Result<()> {
    run(&[TWO, "-c", "200"], "tests/expected/two.txt.c200.out")
}

#[test]
fn two_c_minus_200() -> Result<()> {
    run(&[TWO, "-c=-200"], "tests/expected/two.txt.c200.out")
}

#[test]
fn two_c_plus_0() -> Result<()> {
    run(&[TWO, "-c", "+0"], "tests/expected/two.txt.c+0.out")
}

#[test]
fn two_c_plus_1() -> Result<()> {
    run(&[TWO, "-c", "+1"], "tests/expected/two.txt.c+1.out")
}

#[test]
fn two_c_plus_2() -> Result<()> {
    run(&[TWO, "-c", "+2"], "tests/expected/two.txt.c+2.out")
}

#[test]
fn three() -> Result<()> {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n0() -> Result<()> {
    run(&[THREE, "-n", "0"], "tests/expected/three.txt.n0.out")
}

#[test]
fn three_n1() -> Result<()> {
    run(&[THREE, "-n", "1"], "tests/expected/three.txt.n1.out")
}

#[test]
fn three_n_minus_1() -> Result<()> {
    run(&[THREE, "-n=-1"], "tests/expected/three.txt.n1.out")
}

#[test]
fn three_n3() -> Result<()> {
    run(&[THREE, "-n", "3"], "tests/expected/three.txt.n3.out")
}

#[test]
fn three_n_minus_3() -> Result<()> {
    run(&[THREE, "-n=-3"], "tests/expected/three.txt.n3.out")
}

#[test]
fn three_n4() -> Result<()> {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_n_minus_4() -> Result<()> {
    run(&[THREE, "-n=-4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_n200() -> Result<()> {
    run(&[THREE, "-n", "200"], "tests/expected/three.txt.n200.out")
}

#[test]
fn three_n_minus_200() -> Result<()> {
    run(&[THREE, "-n=-200"], "tests/expected/three.txt.n200.out")
}

#[test]
fn three_n_plus_0() -> Result<()> {
    run(&[THREE, "-n", "+0"], "tests/expected/three.txt.n+0.out")
}

#[test]
fn three_n_plus_1() -> Result<()> {
    run(&[THREE, "-n", "+1"], "tests/expected/three.txt.n+1.out")
}

#[test]
fn three_n_plus_2() -> Result<()> {
    run(&[THREE, "-n", "+2"], "tests/expected/three.txt.n+2.out")
}

#[test]
fn three_c3() -> Result<()> {
    run(&[THREE, "-c", "3"], "tests/expected/three.txt.c3.out")
}

#[test]
fn three_c_minus_3() -> Result<()> {
    run(&[THREE, "-c=-3"], "tests/expected/three.txt.c3.out")
}

#[test]
fn three_c8() -> Result<()> {
    run(&[THREE, "-c", "8"], "tests/expected/three.txt.c8.out")
}

#[test]
fn three_c_minus_8() -> Result<()> {
    run(&[THREE, "-c=8"], "tests/expected/three.txt.c8.out")
}

#[test]
fn three_c12() -> Result<()> {
    run(&[THREE, "-c", "12"], "tests/expected/three.txt.c12.out")
}

#[test]
fn three_c_minus_12() -> Result<()> {
    run(&[THREE, "-c=-12"], "tests/expected/three.txt.c12.out")
}

#[test]
fn three_c200() -> Result<()> {
    run(&[THREE, "-c", "200"], "tests/expected/three.txt.c200.out")
}

#[test]
fn three_c_minus_200() -> Result<()> {
    run(&[THREE, "-c=-200"], "tests/expected/three.txt.c200.out")
}

#[test]
fn three_c_plus_0() -> Result<()> {
    run(&[THREE, "-c", "+0"], "tests/expected/three.txt.c+0.out")
}

#[test]
fn three_c_plus_1() -> Result<()> {
    run(&[THREE, "-c", "+1"], "tests/expected/three.txt.c+1.out")
}

#[test]
fn three_c_plus_2() -> Result<()> {
    run(&[THREE, "-c", "+2"], "tests/expected/three.txt.c+2.out")
}

#[test]
fn ten() -> Result<()> {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n0() -> Result<()> {
    run(&[TEN, "-n", "0"], "tests/expected/ten.txt.n0.out")
}

#[test]
fn ten_n1() -> Result<()> {
    run(&[TEN, "-n", "1"], "tests/expected/ten.txt.n1.out")
}

#[test]
fn ten_n_minus_1() -> Result<()> {
    run(&[TEN, "-n=-1"], "tests/expected/ten.txt.n1.out")
}

#[test]
fn ten_n3() -> Result<()> {
    run(&[TEN, "-n", "3"], "tests/expected/ten.txt.n3.out")
}

#[test]
fn ten_n_minus_3() -> Result<()> {
    run(&[TEN, "-n=-3"], "tests/expected/ten.txt.n3.out")
}

#[test]
fn ten_n4() -> Result<()> {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_n_minus_4() -> Result<()> {
    run(&[TEN, "-n=-4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_n200() -> Result<()> {
    run(&[TEN, "-n", "200"], "tests/expected/ten.txt.n200.out")
}

#[test]
fn ten_n_minus_200() -> Result<()> {
    run(&[TEN, "-n=-200"], "tests/expected/ten.txt.n200.out")
}

#[test]
fn ten_c3() -> Result<()> {
    run(&[TEN, "-c", "3"], "tests/expected/ten.txt.c3.out")
}

#[test]
fn ten_c_minus_3() -> Result<()> {
    run(&[TEN, "-c=-3"], "tests/expected/ten.txt.c3.out")
}

#[test]
fn ten_c8() -> Result<()> {
    run(&[TEN, "-c", "8"], "tests/expected/ten.txt.c8.out")
}

#[test]
fn ten_c_minus_8() -> Result<()> {
    run(&[TEN, "-c=8"], "tests/expected/ten.txt.c8.out")
}

#[test]
fn ten_c12() -> Result<()> {
    run(&[TEN, "-c", "12"], "tests/expected/ten.txt.c12.out")
}

#[test]
fn ten_c_minus_12() -> Result<()> {
    run(&[TEN, "-c=-12"], "tests/expected/ten.txt.c12.out")
}

#[test]
fn ten_c200() -> Result<()> {
    run(&[TEN, "-c", "200"], "tests/expected/ten.txt.c200.out")
}

#[test]
fn ten_c_minus_200() -> Result<()> {
    run(&[TEN, "-c=-200"], "tests/expected/ten.txt.c200.out")
}

#[test]
fn ten_n_plus_0() -> Result<()> {
    run(&[TEN, "-n", "+0"], "tests/expected/ten.txt.n+0.out")
}

#[test]
fn ten_n_plus_1() -> Result<()> {
    run(&[TEN, "-n", "+1"], "tests/expected/ten.txt.n+1.out")
}

#[test]
fn ten_n_plus_2() -> Result<()> {
    run(&[TEN, "-n", "+2"], "tests/expected/ten.txt.n+2.out")
}

#[test]
fn ten_c_plus_0() -> Result<()> {
    run(&[TEN, "-c", "+0"], "tests/expected/ten.txt.c+0.out")
}

#[test]
fn ten_c_plus_1() -> Result<()> {
    run(&[TEN, "-c", "+1"], "tests/expected/ten.txt.c+1.out")
}

#[test]
fn ten_c_plus_2() -> Result<()> {
    run(&[TEN, "-c", "+2"], "tests/expected/ten.txt.c+2.out")
}

#[test]
fn multiple_files() -> Result<()> {
    run(&[TEN, EMPTY, ONE, THREE, TWO], "tests/expected/all.out")
}

#[test]
fn multiple_files_n0() -> Result<()> {
    run(
        &["-n", "0", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n0.out",
    )
}

#[test]
fn multiple_files_n1() -> Result<()> {
    run(
        &["-n", "1", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.out",
    )
}

#[test]
fn multiple_files_n1_q() -> Result<()> {
    run(
        &["-n", "1", "-q", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.q.out",
    )
}

#[test]
fn multiple_files_n1_quiet() -> Result<()> {
    run(
        &["-n", "1", "--quiet", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.q.out",
    )
}

#[test]
fn multiple_files_n_minus_1() -> Result<()> {
    run(
        &["-n=-1", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.out",
    )
}

#[test]
fn multiple_files_n_plus_1() -> Result<()> {
    run(
        &["-n", "+1", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n+1.out",
    )
}

#[test]
fn multiple_files_n3() -> Result<()> {
    run(
        &["-n", "3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n3.out",
    )
}

#[test]
fn multiple_files_n_minus_3() -> Result<()> {
    run(
        &["-n=-3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n3.out",
    )
}

#[test]
fn multiple_files_n_plus_3() -> Result<()> {
    run(
        &["-n", "+3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n+3.out",
    )
}

#[test]
fn multiple_files_c0() -> Result<()> {
    run(
        &["-c", "0", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c0.out",
    )
}

#[test]
fn multiple_files_c3() -> Result<()> {
    run(
        &["-c", "3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c3.out",
    )
}

#[test]
fn multiple_files_c_minus_3() -> Result<()> {
    run(
        &["-c=-3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c3.out",
    )
}

#[test]
fn multiple_files_c_plus_3() -> Result<()> {
    run(
        &["-c", "+3", TEN, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c+3.out",
    )
}
