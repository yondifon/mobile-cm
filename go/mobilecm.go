package mobilecm

import "strings"

// Operator is the network that issued a Cameroonian phone number.
type Operator string

const (
	MTN     Operator = "mtn"
	Orange  Operator = "orange"
	Nexttel Operator = "nexttel"
	Camtel  Operator = "camtel"
)

// OperatorPrefixes maps each operator to the prefixes assigned to it.
var OperatorPrefixes = map[Operator][]string{
	MTN:     {"67", "650", "651", "652", "653", "654", "680", "681", "682", "683"},
	Orange:  {"69", "640", "641", "642", "655", "656", "657", "658", "659", "686", "687", "688", "689"},
	Nexttel: {"66", "684", "685"},
	Camtel:  {"62", "222", "233", "242", "243"},
}

// Check returns the issuing operator when tel is a valid Cameroonian number with a known prefix.
func Check(tel string) (Operator, bool) {
	number, ok := nationalNumber(tel)
	if !ok {
		return "", false
	}

	for operator, prefixes := range OperatorPrefixes {
		for _, prefix := range prefixes {
			if strings.HasPrefix(number, prefix) {
				return operator, true
			}
		}
	}

	return "", false
}

// IsMTN reports whether tel is a valid number issued by MTN.
func IsMTN(tel string) bool { return isOperator(tel, MTN) }

// IsOrange reports whether tel is a valid number issued by Orange.
func IsOrange(tel string) bool { return isOperator(tel, Orange) }

// IsNexttel reports whether tel is a valid number issued by Nexttel.
func IsNexttel(tel string) bool { return isOperator(tel, Nexttel) }

// IsCamtel reports whether tel is a valid number issued by Camtel.
func IsCamtel(tel string) bool { return isOperator(tel, Camtel) }

func isOperator(tel string, operator Operator) bool {
	got, ok := Check(tel)
	return ok && got == operator
}

func nationalNumber(tel string) (string, bool) {
	// strings.Fields splits on unicode.IsSpace, which is exactly Unicode White_Space.
	tel = strings.Join(strings.Fields(tel), "")

	if strings.HasPrefix(tel, "+237") {
		tel = tel[4:]
	} else if strings.HasPrefix(tel, "00237") {
		tel = tel[5:]
	} else if strings.HasPrefix(tel, "237") {
		tel = tel[3:]
	}

	if len(tel) != 9 {
		return "", false
	}
	for i := 0; i < len(tel); i++ {
		if tel[i] < '0' || tel[i] > '9' {
			return "", false
		}
	}
	return tel, true
}
