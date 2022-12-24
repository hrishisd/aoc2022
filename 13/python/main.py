import itertools
from enum import Enum
import functools
import math


def get_packets():
    with open("../input") as f:
        return [parse(line.strip())[0] for line in f if not line.isspace()]


def parse(packet, idx=0) -> tuple[list, int]:
    def parse_int(packet, idx) -> tuple[int, int]:
        c = packet[idx]
        assert c.isdigit()
        if idx + 1 < len(packet) and packet[idx + 1].isdigit():
            # two digit number:
            return int(c) * 10 + int(packet[idx + 1]), idx + 1
        else:
            return int(c), idx + 1

    c = packet[idx]
    assert c == "["
    result = []
    idx += 1
    while packet[idx] != "]":
        c = packet[idx]
        if c == ",":
            idx += 1
        elif c.isdigit():
            d, idx = parse_int(packet, idx)
            # print(f"appening {d=}")
            result.append(d)
        else:
            assert c == "["
            nested_packet, idx = parse(packet, idx)
            # print(f"appending {nested_packet=}")
            result.append(nested_packet)

    return result, idx + 1


class Ord(Enum):
    LT = -1
    EQ = 0
    GT = 1


def compare(l, r):
    # print(f"comparing {(l,r)=}")
    for idx, (l_elem, r_elem) in enumerate(itertools.zip_longest(l, r)):
        if l_elem is None:
            return Ord.LT
        if r_elem is None:
            return Ord.GT
        if isinstance(l_elem, int) != isinstance(r_elem, int):
            if isinstance(l_elem, int):
                l_elem = [l_elem]
            if isinstance(r_elem, int):
                r_elem = [r_elem]
        if isinstance(l_elem, int):
            if l_elem < r_elem:
                return Ord.LT
            elif l_elem > r_elem:
                return Ord.GT
            else:
                continue
        else:
            match compare(l_elem, r_elem):
                case Ord.LT:
                    return Ord.LT
                case Ord.EQ:
                    continue
                case Ord.GT:
                    return Ord.GT
    return Ord.EQ


packets = get_packets()
packet_pairs = zip(packets[::2], packets[1::2])
correct_idices = [
    idx for idx, pair in enumerate(packet_pairs, start=1) if compare(*pair) == Ord.LT
]
print("part 1:", sum(correct_idices))
divider_packets = [[[2]], [[6]]]
sorted_packets = sorted(
    packets + divider_packets,
    key=functools.cmp_to_key(lambda l, r: compare(l, r).value),
)

decoder_key = math.prod(
    (
        idx
        for idx, packet in enumerate(sorted_packets, start=1)
        if packet in divider_packets
    )
)

print("part 2:", decoder_key)
