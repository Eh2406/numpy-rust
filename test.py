import subprocess
import shutil

subprocess.call(["cargo", "build", "--release"])
shutil.copy(r'target\release\hello.dll', r'hello.pyd')

import hello;

print dir(hello)
