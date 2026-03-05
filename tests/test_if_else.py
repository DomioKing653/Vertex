import os
import subprocess

COMPILER_PATH = "target/debug/flarec"
TEST_DIR = "testing/temp"
OUTPUT_NAME = "test-if-else"


def setup():
    os.makedirs(TEST_DIR, exist_ok=True)


def run_flare_code(code: str) -> tuple[int, str, str]:
    """Compile and run Flare code, return (exit_code, stdout, stderr)"""
    test_file = os.path.join(TEST_DIR, "test.flare")

    with open(test_file, "w") as f:
        f.write(code)

    build_proc = subprocess.run(
        [COMPILER_PATH, "build", test_file, OUTPUT_NAME], capture_output=True, text=True
    )

    if build_proc.returncode != 0:
        return build_proc.returncode, build_proc.stdout, build_proc.stderr

    run_proc = subprocess.run(
        [COMPILER_PATH, "run", f"target/{OUTPUT_NAME}"], capture_output=True, text=True
    )

    return run_proc.returncode, run_proc.stdout, run_proc.stderr


def test_if_true_no_else():
    code = """
if(true){
    writeLn!("executed")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "executed" in stdout
    assert stderr == ""


def test_if_false_no_else():
    code = """
if(false){
    writeLn!("should not print")
}
writeLn!("after if")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "should not print" not in stdout
    assert "after if" in stdout


def test_if_true_with_else():
    code = """
if(true){
    writeLn!("then branch")
}
else{
    writeLn!("else branch")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "then branch" in stdout
    assert "else branch" not in stdout


def test_if_false_with_else():
    code = """
if(false){
    writeLn!("then branch")
}
else{
    writeLn!("else branch")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "then branch" not in stdout
    assert "else branch" in stdout


def test_if_greater_than_true():
    code = """
if(10 > 5){
    writeLn!("10 is greater")
}
else{
    writeLn!("10 is not greater")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "10 is greater" in stdout
    assert "10 is not greater" not in stdout


def test_if_greater_than_false():
    code = """
if(3 > 5){
    writeLn!("3 is greater")
}
else{
    writeLn!("3 is not greater")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "3 is greater" not in stdout
    assert "3 is not greater" in stdout


def test_if_less_than_true():
    code = """
if(3 < 5){
    writeLn!("3 is less")
}
else{
    writeLn!("3 is not less")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "3 is less" in stdout
    assert "3 is not less" not in stdout


def test_if_less_than_false():
    code = """
if(10 < 5){
    writeLn!("10 is less")
}
else{
    writeLn!("10 is not less")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "10 is less" not in stdout
    assert "10 is not less" in stdout


def test_if_with_variable():
    code = """
var x = 10;
if(x > 5){
    writeLn!("x is big")
}
else{
    writeLn!("x is small")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "x is big" in stdout
    assert "x is small" not in stdout


def test_if_with_constant_folding():
    code = """
if(5 + 5 > 8){
    writeLn!("10 is greater than 8")
}
else{
    writeLn!("10 is not greater than 8")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "10 is greater than 8" in stdout
    assert "10 is not greater than 8" not in stdout


def test_if_else_with_multiple_statements():
    code = """
if(true){
    writeLn!("line 1")
    writeLn!("line 2")
    writeLn!("line 3")
}
else{
    writeLn!("else 1")
    writeLn!("else 2")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "line 1" in stdout
    assert "line 2" in stdout
    assert "line 3" in stdout
    assert "else 1" not in stdout
    assert "else 2" not in stdout


def test_if_with_variable_assignment():
    code = """
var result = 0;
if(true){
    result = 100;
}
else{
    result = 200;
}
writeLn!(result)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "100" in stdout
    assert "200" not in stdout


def test_multiple_if_statements():
    code = """
if(true){
    writeLn!("first if")
}
if(false){
    writeLn!("second if")
}
if(true){
    writeLn!("third if")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "first if" in stdout
    assert "second if" not in stdout
    assert "third if" in stdout


def test_if_after_if_else():
    code = """
if(false){
    writeLn!("first then")
}
else{
    writeLn!("first else")
}
writeLn!("between")
if(true){
    writeLn!("second then")
}
else{
    writeLn!("second else")
}
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "first then" not in stdout
    assert "first else" in stdout
    assert "between" in stdout
    assert "second then" in stdout
    assert "second else" not in stdout


if __name__ == "__main__":
    setup()
    print("Running if-else tests...")

    tests = [
        ("if true no else", test_if_true_no_else),
        ("if false no else", test_if_false_no_else),
        ("if true with else", test_if_true_with_else),
        ("if false with else", test_if_false_with_else),
        ("if greater than true", test_if_greater_than_true),
        ("if greater than false", test_if_greater_than_false),
        ("if less than true", test_if_less_than_true),
        ("if less than false", test_if_less_than_false),
        ("if with variable", test_if_with_variable),
        ("if with constant folding", test_if_with_constant_folding),
        ("if else with multiple statements", test_if_else_with_multiple_statements),
        ("if with variable assignment", test_if_with_variable_assignment),
        ("multiple if statements", test_multiple_if_statements),
        ("if after if else", test_if_after_if_else),
    ]

    passed = 0
    failed = 0

    for name, test_func in tests:
        try:
            test_func()
            print(f"✓ {name}")
            passed += 1
        except AssertionError as e:
            print(f"✗ {name}: {e}")
            failed += 1
        except Exception as e:
            print(f"✗ {name}: ERROR - {e}")
            failed += 1

    print(f"\n{passed} passed, {failed} failed")
