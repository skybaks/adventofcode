import numpy as np
import scipy.optimize
import heapq
from pprint import pprint


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

            print(buttons)
            (nnls_result, norm) = scipy.optimize.nnls(buttons, power)
            #print()
            #continue
            print(nnls_result)
            first_guess = tuple(nnls_result.round().astype(int))
            #first_guess = tuple(np.zeros(buttons.shape[1], dtype=int))
            #first_guess = tuple(np.array([3, 0, 5, 2, 2]))
            guesses = []
            heapq.heappush(guesses, first_guess)
            #heapq.heappush(guesses, tuple(nnls_result.astype(int)))
            presses = -1
            bad_guesses = {}
            iterations = 0
            last_closeness = -1
            best_guess = None
            best_result = None
            most_closeness = -1
            while True:
                getting_better = False
                while guesses:
                    guess = heapq.heappop(guesses)
                    if guess in bad_guesses:
                        pass
                        result = None
                        continue
                    result = (buttons @ guess) - power
                    closeness = abs(result).sum()
                    if True:
                        print("result " + str(result) + " guess " + str(np.array(guess)) + " closeness " + str(closeness))
                        pass
                    if most_closeness < 0:
                        getting_better = True
                        most_closeness = closeness
                        best_guess = guess
                        best_result = result
                    elif closeness < most_closeness or (closeness == most_closeness and sum(guess) < sum(best_guess)):
                        getting_better = True
                        most_closeness = closeness
                        best_guess = guess
                        best_result = result
                    if not np.any(result):
                        presses = np.array(guess).sum()
                        break
                    bad_guesses[guess] = None

                if last_closeness == most_closeness:
                    #raise Exception("get me out")
                    pass
                last_closeness = most_closeness

                closeness = most_closeness
                result = best_result
                guess = best_guess
                # Need to get better at making guesses
                print("iteration best result " + str(result) + " guess " + str(np.array(guess)) + " closeness " + str(closeness))
                print("----------------------------------")
                if presses > 0:
                    break
                if not getting_better:
                    raise Exception("not improving")
                    #most_closeness = -1

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
                        new_guess2 = np.array(guess)
                        for new_guess_i in range(len(new_guess)):
                            if buttons[i][new_guess_i] != 0:
                                num_add_count += 1
                            if num_add_count > r:
                                new_guess[new_guess_i] += (curr_result_num * -1)
                                new_guess2[new_guess_i] += (singular_change * -1)
                                break
                        new_guess[new_guess < 0] = 0
                        new_guess2[new_guess2 < 0] = 0
                        #print(new_guess)
                        heapq.heappush(guesses, tuple(new_guess))
                        heapq.heappush(guesses, tuple(new_guess2))



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
                #if iterations > 10000:
                #    break

            if presses <= 0:
                raise Exception("Did no converge")
            count += 1
            print(f"{count} presses: {presses}")
            total += presses
            #break
    print(f"total: {total}")
