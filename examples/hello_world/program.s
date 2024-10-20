.section .text
.globl _start
_start:
    # Syscall to print "Hello, World!\n" inline
    li a0, 1                  # stdout (file descriptor 1)
    la a1, hello_world         # Address of the string
    li a2, 13                 # Length of the string (13 characters)

    li a7, 64                 # Syscall number for write (assuming Linux ABI)
    ecall                     # Call kernel

    li a7, 93                 # Syscall number for exit
    ecall                     # Exit program

hello_world:
    .ascii "Hello, World!\n"   # The string is embedded in the .text section
