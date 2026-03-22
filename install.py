from os import path, makedirs, name, chdir
import shutil
import subprocess

REPO = "https://github.com/DomioKing653/Vertex"
DIR = "Vertex"


def run(cmd):
    print("Running:", " ".join(cmd))
    subprocess.check_call(cmd)


# clone repo
if not path.exists(DIR):
    run(["git", "clone", REPO])
else:
    print('Repo is already cloned')

# build binaries
chdir(DIR)

run(["cargo", "build", "--bin", "vertexC", "--release"])
run(["cargo", "build", "--bin", "vertex", "--release"])

target = path.join("target", "release")

if name == "nt": 
    vertexC = path.join(target, "vertexC.exe")
    vertex = path.join(target, "vertex.exe")
    install_dir = r"C:\Program Files\flare"
else:
    vertexC = path.join(target, "vertexC")
    vertex = path.join(target, "vertex")
    install_dir = "/usr/local/bin"

# create install dir if needed
makedirs(install_dir, exist_ok=True)

# copy binaries
shutil.copy(vertexC, install_dir)
shutil.copy(vertex,
            install_dir)

print('Installed vertexC and vertex to', install_dir)
