import numpy as np
import heapq


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

            first_guess = tuple(list(np.zeros(buttons.shape[1], dtype=int)))
            guesses = []
            heapq.heappush(guesses, first_guess)
            presses = -1
            bad_guesses = {}
            while guesses:
                guess = heapq.heappop(guesses)
                if guess in bad_guesses:
                    continue
                result = (buttons @ guess) - power
                if not np.any(result):
                    presses = np.array(guess).sum()
                    break
                print(np.array(guess))
                bad_guesses[guess] = None
                for i in range(len(result)):
                    for j in range(len(buttons[i])):
                        if result[i] > 0 and buttons[i][j] != 0:
                            new_guess = list(guess)
                            if new_guess[j] > 0:
                                new_guess[j] -= 1
                                heapq.heappush(guesses, tuple(new_guess))
                        elif result[i] < 0 and buttons[i][j] != 0:
                            new_guess = list(guess)
                            new_guess[j] += 1
                            heapq.heappush(guesses, tuple(new_guess))

            count += 1
            print(f"{count} presses: {presses}")
            total += presses
            #break
    print(f"total: {total}")
