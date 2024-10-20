    .section .text
    .globl _start
_start:
    # Initialize registers x6 and x7
    li   x6, 10           # Load immediate value 10 into x6
    li   x7, 20           # Load immediate value 20 into x7

    # ADD: x5 = x6 + x7
    add  x5, x6, x7       # x5 = 10 + 20 = 30

    # SUB: x5 = x6 - x7
    sub  x5, x6, x7       # x5 = 10 - 20 = -10 

    # AND: x5 = x6 & x7
    and  x5, x6, x7       # x5 = x6 AND x7

    # OR: x5 = x6 | x7
    or   x5, x6, x7       # x5 = x6 OR x7

    # XOR: x5 = x6 ^ x7
    xor  x5, x6, x7       # x5 = x6 XOR x7

    # ADDI: x5 = x6 + (-5)
    addi x5, x6, -5       # x5 = x6 + (-5)
