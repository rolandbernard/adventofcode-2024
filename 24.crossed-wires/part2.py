import sys
import z3

inp = sys.stdin.read()
_, gates = inp.split('\n\n')
gates = [
    list(gate.split()) + [out] for gate, out in
    (list(l.split(' -> ')) for l in gates.strip().split('\n'))
]
xbits = 1 + max(int(i[1:]) for g in gates for i in g if i.startswith('x'))
ybits = 1 + max(int(i[1:]) for g in gates for i in g if i.startswith('y'))
zbits = 1 + max(int(i[1:]) for g in gates for i in g if i.startswith('z'))
gate_iter = 0


def build_gates(solver, x, y, z, mapping=None):
    global gate_iter
    gate_iter += 1
    values = {}
    if type(x) is int:
        x = z3.BitVecVal(x, xbits)
    if type(y) is int:
        y = z3.BitVecVal(y, ybits)
    if type(z) is int:
        z = z3.BitVecVal(z, zbits)
    for i in range(0, xbits):
        values[f'x{i:02}'] = z3.Extract(i, i, x) == 1
    for i in range(0, ybits):
        values[f'y{i:02}'] = z3.Extract(i, i, y) == 1
    for i in range(0, zbits):
        values[f'z{i:02}'] = z3.Extract(i, i, z) == 1
    for _, _, _, out in gates:
        if not out.startswith('z'):
            values[out] = z3.Bool(f'{out}_{gate_iter}')
    for i, [i0, op, i1, out] in enumerate(gates):
        i0, i1 = values[i0], values[i1]
        match op:
            case 'AND':
                value = i0 & i1
            case 'OR':
                value = i0 | i1
            case 'XOR':
                value = i0 ^ i1
        if mapping is None:
            recv = z3.Or([
                swap & values[gates[j][3]]
                for j, swap in enumerate(swaps[i])
            ])
        else:
            recv = values[mapping.get(out, out)]
        solver.add(value == recv)


def find_counter(swaps):
    solver = z3.Solver()
    x = z3.BitVec('x', xbits)
    y = z3.BitVec('y', ybits)
    z = z3.BitVec('z', zbits)
    build_gates(solver, x, y, z, swaps)
    x_ext = z3.ZeroExt(zbits - xbits, x)
    y_ext = z3.ZeroExt(zbits - ybits, y)
    solver.add(x_ext + y_ext != z)
    if solver.check() == z3.unsat:
        return None, None
    else:
        model = solver.model()
        return model[x].as_long(), model[y].as_long()


solver = z3.Solver()
swaps = [
    [z3.Bool(f's_{a[3]}_{b[3]}') for b in gates[:i]] for i, a in enumerate(gates)
]
solver.add(z3.AtMost(*[s for ss in swaps for s in ss], 4))
solver.add(z3.AtLeast(*[s for ss in swaps for s in ss], 4))
for i in range(len(gates)):
    swaps[i].append(z3.Bool(f'ns_{gates[i][3]}'))
    for j in range(i + 1, len(gates)):
        assert len(swaps[i]) == j
        swaps[i].append(swaps[j][i])
    solver.add(z3.AtMost(*swaps[i], 1))
    solver.add(z3.AtLeast(*swaps[i], 1))

while True:
    assert solver.check() == z3.sat
    model = solver.model()
    needed_swaps = {}
    for i, a in enumerate(gates):
        for j, b in enumerate(gates):
            if i != j and bool(model[swaps[i][j]]):
                needed_swaps[a[3]] = b[3]
    x, y = find_counter(needed_swaps)
    if x is None:
        break
    build_gates(solver, x, y, x + y)

print('Result:', ','.join(sorted(needed_swaps.keys())))
