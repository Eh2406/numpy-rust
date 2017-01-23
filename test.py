import subprocess
import shutil
import numpy as np

subprocess.call(["cargo", "build", "--release"])
shutil.copy(r'target\release\hello.dll', r'hello.pyd')

import hello;

print dir(hello)

x = np.arange(12.).astype('float64').reshape((-1, 2), order='c')
print x
print hello.func(x[:, 1])# seams to only work with 1 d arrays
