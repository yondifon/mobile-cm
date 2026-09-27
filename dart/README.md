# mobile_cm (Dart)

Identify the Cameroonian mobile operator (MTN, Orange, Nexttel, Camtel) that issued a phone number.

Not published to pub.dev. Pure Dart, no runtime dependencies, works in Flutter.

## Install

Use as a local path dependency until/unless it's published:

```yaml
dependencies:
  mobile_cm:
    path: ../mobile-cm-php/dart
```

## Usage

```dart
import 'package:mobile_cm/mobile_cm.dart';

check('676777777'); // Operator.mtn
check('+237699238282'); // Operator.orange
check('123456789'); // null

isMtn('676777777'); // true
```

See [`../spec/README.md`](../spec/README.md) for the input rules, the prefix table, and the portability caveat: a prefix names the operator that issued the number, not necessarily the network the subscriber is on today.
