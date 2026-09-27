# mobile-cm (Python)

Identify the Cameroonian operator (MTN, Orange, Nexttel, Camtel) that issued a phone number.

## Install

```sh
pip install mobile-cm
```


## Usage

```python
from mobile_cm import check, is_mtn, OPERATOR_PREFIXES

check("676777777")  # "mtn"
check("+237699238282")  # "orange"
check("123456789")  # None

is_mtn("676777777")  # True
```

See the [shared spec](https://github.com/yondifon/mobile-cm/blob/main/spec/README.md) for the input rules, prefix table, and portability caveat: a prefix names the operator that issued the number, not necessarily the subscriber's current network.
