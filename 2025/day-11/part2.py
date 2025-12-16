from pprint import pprint


if __name__ == "__main__":
    branches: dict[str, list[str]] = {}
    with open('input.txt', 'r') as handle:
        for line in handle.readlines():
            line = line.strip()
            (key, conns) = line.split(':')
            branches[key] = [c for c in conns.split(' ') if c]

    path_count = 0
    def traverse_to_out(
            entry: str,
            touched_fft: bool,
            touched_dac: bool
        ) -> None:
        local_touched_fft = touched_fft
        local_touched_dac = touched_dac
        for connection in branches[entry]:
            if connection == 'out':
                if touched_dac and touched_fft:
                    global path_count
                    path_count += 1
                continue
            elif connection == 'fft':
                local_touched_fft = True
            elif connection == 'dac':
                local_touched_dac = True
            traverse_to_out(connection, local_touched_fft, local_touched_dac)
    traverse_to_out('svr', False, False)
    print(path_count)
    pass
