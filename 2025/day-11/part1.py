from pprint import pprint


if __name__ == "__main__":
    branches: dict[str, list[str]] = {}
    with open('input.txt', 'r') as handle:
        for line in handle.readlines():
            line = line.strip()
            (key, conns) = line.split(':')
            branches[key] = [c for c in conns.split(' ') if c]
    #pprint(branches)

    path_count = 0
    def traverse_to_out(entry: str) -> None:
        for connection in branches[entry]:
            if connection == 'out':
                global path_count
                path_count += 1
                continue
            traverse_to_out(connection)
    traverse_to_out('you')
    print(path_count)
    pass
