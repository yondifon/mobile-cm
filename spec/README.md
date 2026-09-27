# Cameroon number plan

Every implementation in this repo follows this file and must pass `cases.json`.

## Rules

- A number is 9 digits, optionally preceded by `237`, `+237`, or `00237`.
- Whitespace anywhere is ignored. Any other character (`-`, `.`, `(`) makes the number invalid.
- The operator is the one whose prefix starts the 9-digit number; prefixes never overlap. No match → no operator (`null`/`None`).
- Operator names are lowercase: `mtn`, `orange`, `nexttel`, `camtel`.

| Operator | Prefixes |
|---|---|
| `mtn` | `67`, `650`–`654`, `680`–`683` |
| `orange` | `69`, `640`–`642`, `655`–`659`, `686`–`689` |
| `nexttel` | `66`, `684`, `685` |
| `camtel` | `62` (Blue mobile), `222`, `233` (fixed), `242`, `243` (CDMA) |

Mobile number portability is live in Cameroon, so a prefix names the operator that issued the number. The subscriber may since have moved to another network.

## Sources

- ART numbering plan filed with the ITU, 6 Oct 2014, for the 9-digit switch on 21 Nov 2014: [ITU-T national numbering plan, Cameroon](https://www.itu.int/oth/T0202000024/en). Lists `66` Nexttel, `67` and `650`–`654` MTN, `69` and `655`–`659` Orange, `222`/`233` fixed and `242`/`243` CDMA for Camtel.
- Google libphonenumber carrier map, last changed July 2025: [`resources/carrier/en/237.txt`](https://github.com/google/libphonenumber/blob/master/resources/carrier/en/237.txt). Adds the later blocks: `62` Camtel, `64` Orange, `680`–`683` MTN, `684`–`685` Nexttel, `686`–`689` Orange.
- libphonenumber metadata for `CM`: [`PhoneNumberMetadata.xml`](https://github.com/google/libphonenumber/blob/master/resources/PhoneNumberMetadata.xml). Its mobile pattern only allows `640`–`642` inside `64`, so `643`–`649` are left out here.

## Open questions

- No ART document after 2014 was found online for the `62`, `64`, and `68` blocks; they rest on libphonenumber.
- `88` toll-free numbers (8 or 9 digits) are not handled.
