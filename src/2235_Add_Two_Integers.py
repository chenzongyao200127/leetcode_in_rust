# 2235_Add_Two_Integers
# https://leetcode.cn/problems/add-two-integers/description/

from ctypes import *
import mmap
import os

# This code appears to define a method to compile and execute hand-written assembly code within Python. 
# Here's a breakdown of what the code does:

# 1. **Imports**:
#    - `ctypes`: A foreign function library for Python. 
#                It provides C-compatible data types and allows calling functions in DLLs/shared libraries. 
#    - `mmap`: A module that allows memory-mapped file support for Python. 

# 2. **translate function**:
#    - Translates a string representation of assembly code into raw bytes. 
#      The code assumes that the input string is in a specific format (like that produced by some disassemblers), 
#      with addresses in front, followed by colon-separated machine code bytes, followed by assembly mnemonics. 

# 3. **buf**:
#    - This creates a memory-mapped buffer. The buffer is page-sized, 
#      and the protection flags (PROT_READ, PROT_WRITE, and PROT_EXEC) 
#      mean the buffer is readable, writable, and executable. 

# 4. **compile_asm function**:
#    - This function takes an `asm_code` (assembly code in string format) 
#      and a `ftype` (function type signature from `ctypes`).
#    - It writes the translated machine code into the buffer.
#    - It then returns a callable function object that points to the buffer's address. 
#      This allows the raw machine code in the buffer to be called as if it was a Python function.

# 5. **fn**:
#    - This compiles a short assembly code function, which essentially adds two numbers.
#    - This function pushes `rbp` onto the stack, moves the stack pointer `rsp` to `rbp`, 
#      then gets the two arguments (`edi` and `esi` are typically used for the first and 
#      second function arguments in some calling conventions). It adds these two arguments and returns the result.

# 6. **Solution class**:
#    - Contains a method `sum`, which takes two integers and returns their sum using the compiled assembly function.

# So, in short, the code is demonstrating how to execute hand-written assembly code from within Python. 
# You can use the `Solution().sum(a, b)` to compute the sum of two integers `a` and `b` using the provided assembly code. 

# This is quite a sophisticated and unusual approach to performing such a simple task. 
# Typically, this kind of approach might be used for tasks that require very 
# high performance or to interface with hand-optimized assembly routines.

# Importing required modules

# This function translates assembly code strings into executable bytecode
def translate(asm_code):
    res = b''   # Initialize an empty bytes object to store the translated code
    # Iterate over each line in the assembly code
    for line in asm_code.split('\n'):
        # If the line doesn't contain ':' or contains '>:', skip processing it
        if ':' not in line or '>:' in line:
            continue
        # Extract the bytecode part from the line
        line = line[line.find(':') + 1:].strip()         # Extract content after the ':' character
        line = line[:line.find('   ')].strip()           # Extract content before three spaces, which marks the end of the bytecode
        # Convert each bytecode value from hex to bytes and append to the result
        for b in line.split(' '):
            res += int(b, 16).to_bytes(1, byteorder='little')
    return res

if os.name == "posix":
    buf = mmap.mmap(-1, mmap.PAGESIZE, prot=mmap.PROT_READ | mmap.PROT_WRITE | mmap.PROT_EXEC)
else:
    # For Windows or other platforms
    buf = mmap.mmap(-1, mmap.PAGESIZE)
    # Set memory as executable on Windows
    PAGE_EXECUTE_READWRITE = 0x40
    kernel32 = windll.kernel32
    buf_pointer = c_void_p.from_buffer(buf)
    kernel32.VirtualProtect(buf_pointer.value, mmap.PAGESIZE, PAGE_EXECUTE_READWRITE, byref(c_uint32()))

    
# This function compiles the provided assembly code into an executable function of the given type
def compile_asm(asm_code, ftype):
    buf.write(translate(asm_code))   # Write the translated bytecode to the buffer
    # Return a callable Python function that, when invoked, will execute the machine code in the buffer
    return ftype(addressof(c_void_p.from_buffer(buf)))

# Compiling a specific assembly function: a function that adds two integers
# The provided assembly code corresponds to the logic to add two integers in the x86_64 architecture
fn = compile_asm(
    """
    0:  55                      push   rbp                 # Save the old base pointer value
    1:  48 89 e5                mov    rbp,rsp             # Set new base pointer value
    4:  89 7d fc                mov    DWORD PTR [rbp-0x4],edi    # Move the first argument to local stack
    7:  89 75 f8                mov    DWORD PTR [rbp-0x8],esi    # Move the second argument to local stack
    a:  8b 55 fc                mov    edx,DWORD PTR [rbp-0x4]    # Load the first argument into edx
    d:  8b 45 f8                mov    eax,DWORD PTR [rbp-0x8]    # Load the second argument into eax
    10: 01 d0                   add    eax,edx             # Add the values in edx and eax
    12: 5d                      pop    rbp                 # Restore the old base pointer value
    13: c3                      ret                        # Return from the function
    """,
    CFUNCTYPE(c_int, c_int, c_int)  # Define the function type as C calling convention with int return type and two int arguments
)

# A Python class to use the above compiled assembly function
class Solution:
    # Method to compute the sum of two integers using the compiled assembly function
    def sum(self, a: int, b: int) -> int:
        return fn(a, b)

# Test
result = Solution().sum(3, 19)
print(result)