package mobilecm

import (
	"encoding/json"
	"os"
	"testing"
)

func TestSharedCases(t *testing.T) {
	data, err := os.ReadFile("../spec/cases.json")
	if err != nil {
		t.Fatalf("read shared cases: %v", err)
	}

	var cases []struct {
		Input    string  `json:"input"`
		Operator *string `json:"operator"`
	}
	if err := json.Unmarshal(data, &cases); err != nil {
		t.Fatalf("decode shared cases: %v", err)
	}

	for _, testCase := range cases {
		t.Run(testCase.Input, func(t *testing.T) {
			want, wantOK := Operator(""), false
			if testCase.Operator != nil {
				want, wantOK = Operator(*testCase.Operator), true
			}

			got, gotOK := Check(testCase.Input)
			if got != want || gotOK != wantOK {
				t.Errorf("Check(%q) = (%q, %t), want (%q, %t)", testCase.Input, got, gotOK, want, wantOK)
			}

			checks := []struct {
				name     string
				check    func(string) bool
				operator Operator
			}{
				{"IsMTN", IsMTN, MTN},
				{"IsOrange", IsOrange, Orange},
				{"IsNexttel", IsNexttel, Nexttel},
				{"IsCamtel", IsCamtel, Camtel},
			}
			for _, check := range checks {
				t.Run(check.name, func(t *testing.T) {
					want := wantOK && want == check.operator
					if got := check.check(testCase.Input); got != want {
						t.Errorf("%s(%q) = %t, want %t", check.name, testCase.Input, got, want)
					}
				})
			}
		})
	}
}
