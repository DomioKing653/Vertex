import subprocess

compilerPath = "C:/Users/simon/RustroverProjects/Flare/target/debug/flarec"


def run_compiler(source: str):
    proc = subprocess.run(
        [compilerPath, "build", source, "testing-1"],

    )
    subprocess.run(
        [compilerPath,
         "run",
         "target/testing-1"
         ],
        capture_output=True,
        text=True
    )
    print(proc.returncode)
    return proc.returncode, proc.stdout, proc.stderr
