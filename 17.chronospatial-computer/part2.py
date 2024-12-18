
import sys
import z3

inp = sys.stdin.read()
regs, prog = inp.split('\n\n')
_, b, c = [int(l.split(': ')[1]) for l in regs.split('\n')]
prog = [int(i) for i in prog.strip().split(': ')[1].split(',')]

assert len([op for op in prog[::2] if op == 5]) == 1
assert len([op for op in prog[::2] if op == 3]) == 1
assert prog[-2] == 3
assert prog[-1] == 0

solver = z3.Optimize()
a = a_var = z3.BitVec('a', 64)
out = 0
ip = 0
while True:
    opc = prog[ip]
    opr = prog[ip + 1]
    combo = opr
    match opr:
        case 4:
            combo = a
        case 5:
            combo = b
        case 6:
            combo = c
    ip += 2
    match opc:
        case 0:
            a = a >> combo
        case 1:
            b = b ^ opr
        case 2:
            b = combo & 0b111
        case 3:
            if out < len(prog):
                solver.add(a != 0)
            else:
                solver.add(a == 0)
                break
            ip = 0
        case 4:
            b = b ^ c
        case 5:
            solver.add((combo & 0b111) == prog[out])
            out += 1
        case 6:
            b = a >> combo
        case 7:
            c = a >> combo
solver.minimize(a_var)
assert solver.check() == z3.sat

print('Result:', solver.model()[a_var])

