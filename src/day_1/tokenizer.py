from typing import List, Dict, Iterator, Optional
import dataclass

@dataclass.dataclass
class SizedBuffer:
    size: int
    memory: List[str] = []

    def append(self, other: str) -> None:
        assert len(other) == 1 # requires characters to be feed one by one
        if len(self.memory) == size:
            self.memory = self.memory[1:] # pop front
        self.memory.append(other)

    def try_remove(self, value: str) -> bool:
        if not self.memory:
            return False
        word = "".join(self.memory)
        if value in word:
            word = word.replace(value, "") # remove value from word, this bugs out with value is in word twice but we ignore that
            self.memory = [chr for chr in word] # set buffer memory to not contain value anymore
            return True
        return False

@dataclass.dataclass
class Tokenizer:
    buffer: SizedBuffer
    tokens: Dict[str, int] # maps text tokens to their value, ie "one" -> 1

    def next(self, stream: Iterator[str]) -> Optional[int]:
        # given a stream of chars finds the next token and maps it to its value, partially consuming the stream
        for chr in stream:
            self.buffer.append(chr)
            for token, value in self.tokens.items():
                if self.buffer.try_remove(token):
                    return value

    def last(self, stream: Iterator[str]) -> Optional[int]:
        # finds the last occurrence of any token in a stream, consuming it in process
        next_value = self.next(stream)
        last_value = None
        while next_value is not None:
            last_value = next_value
            next_value = self.next(stream) # continue to consume the stream
        return last_value

if __name__ == "__main__":
    example_stream = "onetwothree"

    tokens = {"one": 1, "two": 2, "three": 3}
    buffer_size = max([len(key) for key in tokens]) + 1
    tokenizer = Tokenizer(SizedBuffer(buffer_size), tokens)

    first_digit = tokenizer.next(example_stream)
    assert first_digit == 1
    last_digit = tokenizer.last(example_stream)
    assert last_digit == 3