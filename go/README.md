# mobilecm

Identify the Cameroonian operator that issued a phone number.

## Install

```sh
go get github.com/yondifon/mobile-cm/go
```


## Usage

```go
import (
	"fmt"

	mobilecm "github.com/yondifon/mobile-cm/go"
)

operator, ok := mobilecm.Check("+237 676 77 77 77")
if ok {
	fmt.Println(operator) // mtn
}
```

See the [shared specification](https://github.com/yondifon/mobile-cm/blob/main/spec/README.md) for the prefix table and the portability caveat: a number may have moved to another network since it was issued.
