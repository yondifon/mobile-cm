# Mobile Operators Cameroon

Find the operator of a Cameroonian phone number.

## Installation

```bash
composer require malico/mobile-cm-php
```

Requires PHP 8.1 or later.

## Usage

```php
use Malico\MobileCM\Network;

Network::check('+237 653 95 67 03'); // 'mtn'
Network::check('00237699238282');    // 'orange'
Network::check('12345');             // null

Network::isMTN('653956703');         // true
Network::isOrange('653956703');      // false
Network::isNexttel('666768293');     // true
Network::isCamtel('222479973');      // true
```

`check()` returns `mtn`, `orange`, `nexttel`, `camtel`, or `null`.

Numbers may start with `237`, `+237`, or `00237`, and may contain spaces.

A prefix names the operator that issued the number. With number portability, the subscriber may have moved to another network since.

The prefix table and its sources are in [`spec/README.md`](spec/README.md).
