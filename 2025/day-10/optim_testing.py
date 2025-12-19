
import numpy as np
from scipy import optimize


if __name__ == "__main__":
    buttons = np.array([
        [0, 0, 0, 0, 1, 1],
        [0, 0, 1, 0, 0, 1],
        [0, 1, 0, 1, 1, 0],
        [1, 0, 1, 1, 0, 0]
    ])
    guess = np.ones(6)
    power = np.array([3,5,4,7])
    constraint = optimize.LinearConstraint(A=buttons, lb=power, ub=power)
    result = optimize.milp(guess, constraints=constraint)
    print(result)
