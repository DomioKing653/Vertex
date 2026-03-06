import os
import shutil
import subprocess

REPO = "https://github.com/DomioKing653/flare"
DIR = "flare"


def run(cmd):
    print("Running:", " ".join(cmd))
    subprocess.check_call(cmd)


# clone repo
if not os.path.exists(DIR):
    run(["git", "clone", REPO])
else:
    print("Repo already exists")

# build binaries
os.chdir(DIR)

run(["cargo", "build", "--bin", "flarec", "--release"])
run(["cargo", "build", "--bin", "flauncher", "--release"])

target = os.path.join("target", "release")

if os.name == "nt":
    flarec = os.path.join(target, "flarec.exe")
    flauncher = os.path.join(target, "flauncher.exe")
    install_dir = r"C:\Program Files\flare"
else:
    flarec = os.path.join(target, "flarec")
    flauncher = os.path.join(target, "flauncher")
    install_dir = "/usr/local/bin"

# create install dir if needed
os.makedirs(install_dir, exist_ok=True)

# copy binaries
shutil.copy(flarec, install_dir)
shutil.copy(flauncher, install_dir)

print("Installed flarec and flauncher to", install_dir)
