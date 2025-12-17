import numpy as np
import heapq
from pprint import pprint


if __name__ == "__main__":
    total = 0
    with open('test.txt', 'r') as handle:
        count = 0
        for line in handle.readlines():
            buttons = [
                [
                    int(n) for n in b.strip(")(").split(',')
                ]
                for b in line[line.index(']')+1:line.index('{')].strip().split(' ')
            ]

            # sort the buttons with more connections to the back of the list
            buttons.sort(key=lambda x: len(x))

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

            first_guess = tuple(np.zeros(buttons.shape[1], dtype=int))
            guesses = []
            heapq.heappush(guesses, first_guess)
            presses = -1
            bad_guesses = {}
            iterations = 0
            print(buttons)
            best_guess = None
            best_result = None
            most_closeness = -1
            while True:
                getting_better = False
                while guesses:
                    guess = heapq.heappop(guesses)
                    if guess in bad_guesses:
                        continue
                    result = (buttons @ guess) - power
                    closeness = abs(result).sum()
                    if most_closeness < 0:
                        getting_better = True
                        most_closeness = closeness
                        best_guess = guess
                        best_result = result
                    elif closeness < most_closeness:
                        getting_better = True
                        most_closeness = closeness
                        best_guess = guess
                        best_result = result
                    if not np.any(result):
                        presses = np.array(guess).sum()
                        break
                    bad_guesses[guess] = None

                closeness = most_closeness
                result = best_result
                guess = best_guess
                # Need to get better at making guesses
                print("result " + str(result) + " guess " + str(np.array(guess)) + " closeness " + str(closeness))
                if not getting_better or presses > 0:
                    break

                for i in range(len(result)):
                    curr_result_num = result[i]
                    #print(curr_result_num)
                    if curr_result_num == 0:
                        continue
                    singular_change = 1
                    if curr_result_num < 0:
                        singular_change = -1
                    num_splits = np.array(buttons[i]).sum()
                    for r in range(num_splits):
                        num_add_count = 0
                        new_guess = np.array(guess)
                        for new_guess_i in range(len(new_guess)):
                            if buttons[i][new_guess_i] != 0:
                                num_add_count += 1
                            if num_add_count > r:
                                #new_guess[new_guess_i] += (curr_result_num * -1)
                                new_guess[new_guess_i] += (singular_change * -1)
                                break
                        new_guess[new_guess < 0] = 0
                        #print(new_guess)
                        heapq.heappush(guesses, tuple(new_guess))
                    """
                    numr = abs(curr_result_num) // num_splits
                    rmdr = abs(curr_result_num) % num_splits
                    if curr_result_num > 0:
                        numr *= -1
                        rmdr *= -1
                    new_guess_mod = np.array(buttons[i]) * numr
                    if rmdr != 0:
                        for r in range(num_splits):
                            rmdr_add_count = 0
                            new_guess = np.array(guess) + new_guess_mod
                            for new_guess_i in range(len(new_guess)):
                                if new_guess_mod[new_guess_i] != 0:
                                    rmdr_add_count += 1
                                if rmdr_add_count > r:
                                    new_guess[new_guess_i] += rmdr
                                    break
                            new_guess[new_guess < 0] = 0
                            #print(new_guess)
                            heapq.heappush(guesses, tuple(new_guess))
                    else:
                        new_guess = np.array(guess) + new_guess_mod
                        new_guess[new_guess < 0] = 0
                        #print(new_guess)
                        heapq.heappush(guesses, tuple(new_guess))
                    """

                    """
                    for j in range(len(buttons[i])):
                        if result[i] > 0 and buttons[i][j] != 0:
                            new_guess = list(guess)
                            if new_guess[j] > 0:
                                new_guess[j] -= (abs(result[i]) // buttons[i].sum())
                                heapq.heappush(guesses, tuple(new_guess))
                        elif result[i] < 0 and buttons[i][j] != 0:
                            new_guess = list(guess)
                            new_guess[j] += (abs(result[i]) // buttons[i].sum())
                            print(np.array(new_guess))
                            heapq.heappush(guesses, tuple(new_guess))
                    """
                    #break
                #break
                iterations += 1
                if iterations > 100:
                    break

            count += 1
            print(f"{count} presses: {presses}")
            total += presses
            #break
    print(f"total: {total}")
