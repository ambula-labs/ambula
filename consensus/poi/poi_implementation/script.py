import json
import os
import time
import subprocess
import re
import sys

start_time = time.time()

for x in range(100):
               
    os.system('cargo run')
    

end_time = time.time()
runtime = end_time - start_time  
print("Runtime: {:.3f} seconds".format(runtime))
       