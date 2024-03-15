# lexer for cs5131 project
## build instructions 
1. `maturin build`
2. move `target/wheels/*.whl` to your directory
3. run `pip install *.whl` (in a venv unless you want to fuck up your global namespace)
4. profit

## how to use
```py
from lexer import lex

code = """
char buf[20];
gets(buf) // buffer overflow here :(
"""

lex(code)
```

# TODO
figure out how to do common functions and type aliases
