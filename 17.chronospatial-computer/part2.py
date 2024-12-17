
import z3

prog = [2, 4, 1, 2, 7, 5, 1, 3, 4, 4, 5, 5, 0, 3, 3, 0]

solver = z3.Optimize()
a_var = z3.BitVec('a', 64)
a, b, c = a_var, 0, 0
for out in prog:
    solver.add(a != 0)
    b = a & 0b111
    b = b ^ 0b010
    c = a >> b
    b = b ^ 0b011
    b = b ^ c
    solver.add((b & 0b111) == out)
    a = a >> 3
solver.minimize(a_var)
assert solver.check() == z3.sat

print('Result:', solver.model()[a_var])

