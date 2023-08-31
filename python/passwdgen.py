#!/usr/bin/python3

from secrets import randbelow

def rand(min: int, max: int) -> int:
    return min + randbelow((max - min) + 1)

def getLength() -> int:
    val = input("Password length (leave blank for default): ")
    try:
        length = int(val) if int(val) > 0 else 16
    except ValueError:
        length = 16
    return length

length = getLength()

print(''.join(chr(rand(33, 126)) for _ in range(length)))
