# rillrate-py

[![PyPI][pypi-badge]][pypi-url]

[pypi-badge]: https://badge.fury.io/py/rillrate.svg
[pypi-url]: https://pypi.org/project/rillrate

Dynamic UI for bots, microservices, and IoT.

Python 3 bindings.

## How to use

Install the library:

```sh
pip install rillrate
```

Import it in your code and install a tracer:

```python
import rillrate
rillrate.install()
```

Add a metric and use methods to update it:

```python
gauge = rillrate.Counter("my-package.my-dashboard.my-group.my-counter");
gauge.inc(123);
```

## Expansions
