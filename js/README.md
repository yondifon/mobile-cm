# mobile-cm (JavaScript / TypeScript)

Identify the Cameroonian mobile operator (MTN, Orange, Nexttel, Camtel) that issued a phone number.

## Install

```sh
npm install mobile-cm
```


## Usage

```ts
import { check, isMTN, isOrange, isNexttel, isCamtel, OPERATOR_PREFIXES } from 'mobile-cm';

check('676777777'); // 'mtn'
check('+237699238282'); // 'orange'
check('123456789'); // null

isMTN('676777777'); // true
```

See the [shared spec](https://github.com/yondifon/mobile-cm-php/blob/main/spec/README.md) for the input rules, the prefix table, and the portability caveat: a prefix names the operator that issued the number, not necessarily the network the subscriber is on today.
