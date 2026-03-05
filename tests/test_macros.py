import os
import subprocess

COMPILER_PATH = "target/debug/flarec"
TEST_DIR = "testing/temp"
OUTPUT_NAME = "test-macros"


def setup():
    os.makedirs(TEST_DIR, exist_ok=True)


def run_flare_code(code: str, input_data: str | None = None) -> tuple[int, str, str]:
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
        [COMPILER_PATH, "run", f"target/{OUTPUT_NAME}"],
        input=input_data,
        capture_output=True,
        text=True,
    )

    return run_proc.returncode, run_proc.stdout, run_proc.stderr


# writeLn! tests
def test_writeln_string():
    code = """writeLn!("hello world")"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "hello world" in stdout


def test_writeln_number():
    code = """writeLn!(42)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "42" in stdout


def test_writeln_float():
    code = """writeLn!(3.14)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "3.14" in stdout


def test_writeln_variable():
    code = """
var x = "test";
writeLn!(x)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "test" in stdout


def test_writeln_expression():
    code = """writeLn!(5 + 3)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "8" in stdout


def test_writeln_multiple():
    code = """
writeLn!("line 1")
writeLn!("line 2")
writeLn!("line 3")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "line 1" in stdout
    assert "line 2" in stdout
    assert "line 3" in stdout


# write! tests
def test_write_string():
    code = """
write!("hello")
write!(" ")
write!("world")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "hello world" in stdout


def test_write_no_newline():
    code = """
write!("no newline")
write!("same line")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "no newlinesame line" in stdout


def test_write_number():
    code = """write!(123)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "123" in stdout


def test_write_then_writeln():
    code = """
write!("inline: ")
writeLn!(42)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "inline: 42" in stdout


def test_write_multiple_args():
    code = """write!("hello", " ", "world")"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "hello world" in stdout


# readInput! tests
def test_readinput_basic():
    code = """
var input:string = readInput!("Enter: ");
writeLn!(input)
"""
    exit_code, stdout, stderr = run_flare_code(code, input_data="test input\n")
    assert exit_code == 0
    assert "test input" in stdout


def test_readinput_with_variable():
    code = """
var name:string = readInput!("Name: ");
write!("Hello, ")
writeLn!(name)
"""
    exit_code, stdout, stderr = run_flare_code(code, input_data="Alice\n")
    assert exit_code == 0
    assert "Hello, Alice" in stdout


def test_readinput_multiple():
    code = """
var first:string = readInput!("First: ");
var second:string = readInput!("Second: ");
write!(first)
write!(" ")
writeLn!(second)
"""
    exit_code, stdout, stderr = run_flare_code(code, input_data="Hello\nWorld\n")
    assert exit_code == 0
    assert "Hello World" in stdout


def test_readinput_in_if():
    code = """
var answer:string = readInput!("yes or no: ");
if(answer > "m"){
    writeLn!("yes path")
}
else{
    writeLn!("no path")
}
"""
    exit_code, stdout, stderr = run_flare_code(code, input_data="yes\n")
    assert exit_code == 0
    assert "yes path" in stdout


# processExit! tests
def test_processexit_zero():
    code = """processExit!(0)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert "Exited with code 0" in stdout


def test_processexit_nonzero():
    code = """processExit!(42)"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert "Exited with code 42" in stdout


def test_processexit_before_writeln():
    code = """
writeLn!("before exit")
processExit!(0)
writeLn!("after exit")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert "before exit" in stdout
    assert "after exit" not in stdout


def test_processexit_in_if():
    code = """
if(true){
    writeLn!("exiting")
    processExit!(1)
}
writeLn!("should not print")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert "exiting" in stdout
    assert "should not print" not in stdout


# Combined macro tests
def test_combined_write_writeln():
    code = """
write!("Part 1")
write!(" Part 2")
writeLn!(" Part 3")
writeLn!("New line")
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "Part 1 Part 2 Part 3" in stdout
    assert "New line" in stdout


def test_macros_with_variables():
    code = """
var x = 10;
var y = 20;
write!("x = ")
writeLn!(x)
write!("y = ")
writeLn!(y)
write!("sum = ")
writeLn!(x + y)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "x = 10" in stdout
    assert "y = 20" in stdout
    assert "sum = 30" in stdout


def test_macros_with_if_else():
    code = """
var value = 15;
if(value > 10){
    writeLn!("value is large")
}
else{
    writeLn!("value is small")
}
write!("value is: ")
writeLn!(value)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "value is large" in stdout
    assert "value is small" not in stdout
    assert "value is: 15" in stdout


def test_interactive_program():
    code = """
var name:string = readInput!("Enter your name: ");
write!("Hello, ")
write!(name)
writeLn!("!")
"""
    exit_code, stdout, stderr = run_flare_code(code, input_data="Bob\n")
    assert exit_code == 0
    assert "Hello, Bob!" in stdout


def test_calculator_with_macros():
    code = """
writeLn!("Calculator")
write!("5 + 3 = ")
writeLn!(5 + 3)
write!("10 - 4 = ")
writeLn!(10 - 4)
write!("6 * 7 = ")
writeLn!(6 * 7)
write!("20 / 4 = ")
writeLn!(20 / 4)
"""
    exit_code, stdout, stderr = run_flare_code(code)
    assert exit_code == 0
    assert "5 + 3 = 8" in stdout
    assert "10 - 4 = 6" in stdout
    assert "6 * 7 = 42" in stdout
    assert "20 / 4 = 5" in stdout


if __name__ == "__main__":
    setup()
    print("Running macro tests...")

    tests = [
        # writeLn! tests
        ("writeLn string", test_writeln_string),
        ("writeLn number", test_writeln_number),
        ("writeLn float", test_writeln_float),
        ("writeLn variable", test_writeln_variable),
        ("writeLn expression", test_writeln_expression),
        ("writeLn multiple", test_writeln_multiple),
        # write! tests
        ("write string", test_write_string),
        ("write no newline", test_write_no_newline),
        ("write number", test_write_number),
        ("write then writeLn", test_write_then_writeln),
        ("write multiple args", test_write_multiple_args),
        # readInput! tests
        ("readInput basic", test_readinput_basic),
        ("readInput with variable", test_readinput_with_variable),
        ("readInput multiple", test_readinput_multiple),
        ("readInput in if", test_readinput_in_if),
        # processExit! tests
        ("processExit zero", test_processexit_zero),
        ("processExit nonzero", test_processexit_nonzero),
        ("processExit before writeLn", test_processexit_before_writeln),
        ("processExit in if", test_processexit_in_if),
        # Combined tests
        ("combined write writeLn", test_combined_write_writeln),
        ("macros with variables", test_macros_with_variables),
        ("macros with if else", test_macros_with_if_else),
        ("interactive program", test_interactive_program),
        ("calculator with macros", test_calculator_with_macros),
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
