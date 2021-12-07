import sys
import numpy as np

x = np.array(list(map(int, sys.stdin.read().strip().split(","))))
print(np.sum(np.abs(x - np.median(x).astype(np.int32))))
