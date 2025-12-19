import numpy as np
import scipy.optimize
import heapq
from pprint import pprint

# wrong, too low: 16345
# wrong, too low: 16460
# 16474!
# wrong, too high: 32690


if __name__ == "__main__":
    total = 0
    with open('input.txt', 'r') as handle:
        count = 0
        for line in handle.readlines():
            buttons = [
                [
                    int(n) for n in b.strip(")(").split(',')
                ]
                for b in line[line.index(']')+1:line.index('{')].strip().split(' ')
            ]

            # sort the buttons with more connections to the back of the list
            #buttons.sort(key=lambda x: len(x))

            power = np.array([
                int(n) for n in line[line.index('{'):line.index("}")].strip("}{").split(',')
            ], dtype=int)
            btns = []
            for button in buttons:
                btn = np.zeros_like(power)
                for pos in button:
                    btn[pos] = 1
                btns.append(btn)
            buttons = np.array(btns).transpose()

            #print(buttons)
            guess = np.ones(buttons.shape[1], dtype=int)
            constraint = scipy.optimize.LinearConstraint(A=buttons, lb=power, ub=power)
            result = scipy.optimize.milp(guess, constraints=constraint, integrality=np.ones_like(guess))
            print(result.x)
            #print(result.x.astype(int))
            presses = sum(result.x)
            count += 1
            print(f"{count} presses: {presses}")
            total += presses
            #break
    print(f"total: {total}")
