from pprint import pprint


if __name__ == "__main__":
    branches: dict[str, list[str]] = {}
    with open('input.txt', 'r') as handle:
        for line in handle.readlines():
            line = line.strip()
            (key, conns) = line.split(':')
            branches[key] = [c for c in conns.split(' ') if c]

    def traverse_to_goal(entry: str, goal: str, cache: dict[str, int]) -> int:
        if entry in cache:
            return cache[entry]
        stage_total = 0
        for connection in branches[entry]:
            if connection == 'out' or connection == goal:
                if connection == goal:
                    stage_total += 1
                continue
            result = traverse_to_goal(connection, goal, cache)
            if result > 0:
                stage_total += result
        cache[entry] = stage_total
        return stage_total
    svr_to_fft = traverse_to_goal('svr', 'fft', {})
    fft_to_dac = traverse_to_goal('fft', 'dac', {})
    dac_to_out = traverse_to_goal('dac', 'out', {})
    total = svr_to_fft * fft_to_dac * dac_to_out
    print(total)

    # dac->fft = 0 !!
    # dac->out = 3050
    # roughly the same path to get to dac, should just find all the ways that
    # fft can get on the path that leads to the path that least to dac...

    pass
