import json
import typing as ty


TABLE_SIZE = 7


def to_char(inp: str) -> str:
    vals = {"One": "1", "Two": "2", "Three": "3", "Zero": "0"}
    return vals[inp]


def convert(inp: str) -> str:
    dct = json.loads(inp)
    tbl = dct["tbl"]
    outp = []
    for j in range(TABLE_SIZE):
        for i in range(TABLE_SIZE):
            outp.append(tbl[i][j])
    return "".join(map(to_char, outp))


def parse(inp: str) -> ty.Dict[str, str]:
    title, src = inp.split("|")
    return {"title": title.strip(), "rule": convert(src.strip())}


def print_case(**kwargs) -> None:
    print(
        """
{title}

```json
{rule}
```""".format(
            **kwargs
        )
    )


if __name__ == "__main__":
    with open("conv-rules.md") as inf:
        inp = inf.readlines()
    for case in inp:
        print_case(**parse(case))
