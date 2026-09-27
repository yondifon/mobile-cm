from __future__ import annotations

import json
from pathlib import Path

import pytest

from mobile_cm import check, is_camtel, is_mtn, is_nexttel, is_orange

CASES_PATH = Path(__file__).resolve().parents[2] / "spec" / "cases.json"
CASES = json.loads(CASES_PATH.read_text(encoding="utf-8"))


@pytest.mark.parametrize("case", CASES)
def test_shared_cases(case: dict[str, str | None]) -> None:
    expected = case["operator"]
    input_number = case["input"]

    assert check(input_number) == expected
    assert is_mtn(input_number) is (expected == "mtn")
    assert is_orange(input_number) is (expected == "orange")
    assert is_nexttel(input_number) is (expected == "nexttel")
    assert is_camtel(input_number) is (expected == "camtel")
