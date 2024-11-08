#!/usr/bin/python3


class Abc:
    def get_state(self):
        return "a"


class Abd:
    def get_state(self):
        return "b"


def foo(data):
    print(data.get_state())


data1 = Abc()

data2 = Abd()

foo(data1)
foo(data2)
