import subprocess
from run_compiler import run_compiler

def test_correct_output():
    code,out,err = run_compiler("C:/Users/simon/RustroverProjects/Flare/testingCode/test01")
    assert code == -3
    assert err == ""
    assert "hello" in out