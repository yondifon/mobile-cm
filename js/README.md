# mobile-cm (JavaScript / TypeScript)

Identify the Cameroonian mobile operator (MTN, Orange, Nexttel, Camtel) that issued a phone number.

Not published to npm. Written in TypeScript, distributed as ESM with type declarations, no runtime dependencies.

## Install

Use as a local path dependency until/unless it's published:

```sh
bun add file:../js
```

## Usage

```ts
import { check, isMTN, isOrange, isNexttel, isCamtel, OPERATOR_PREFIXES } from 'mobile-cm';

check('676777777'); // 'mtn'
check('+237699238282'); // 'orange'
check('123456789'); // null

isMTN('676777777'); // true
```

See [`../spec/README.md`](../spec/README.md) for the input rules, the prefix table, and the portability caveat: a prefix names the operator that issued the number, not necessarily the network the subscriber is on today.
