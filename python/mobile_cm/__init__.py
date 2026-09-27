import re
from types import MappingProxyType
from typing import Mapping, Optional

OPERATOR_PREFIXES: Mapping[str, tuple[str, ...]] = MappingProxyType({
    "mtn": ("67", "650", "651", "652", "653", "654", "680", "681", "682", "683"),
    "orange": ("69", "640", "641", "642", "655", "656", "657", "658", "659", "686", "687", "688", "689"),
    "nexttel": ("66", "684", "685"),
    "camtel": ("62", "222", "233", "242", "243"),
})


def check(tel: str) -> Optional[str]:
    """Return the issuing operator for a Cameroonian phone number."""
    compact = re.sub(r"\s+", "", tel, flags=re.ASCII)
    match = re.fullmatch(r"(?:(?:\+|00)?237)?([0-9]{9})", compact, flags=re.ASCII)
    if match is None:
        return None

    number = match.group(1)
    for operator, prefixes in OPERATOR_PREFIXES.items():
        if any(number.startswith(prefix) for prefix in prefixes):
            return operator
    return None


def is_mtn(tel: str) -> bool:
    return check(tel) == "mtn"


def is_orange(tel: str) -> bool:
    return check(tel) == "orange"


def is_nexttel(tel: str) -> bool:
    return check(tel) == "nexttel"


def is_camtel(tel: str) -> bool:
    return check(tel) == "camtel"


__all__ = [
    "OPERATOR_PREFIXES",
    "check",
    "is_camtel",
    "is_mtn",
    "is_nexttel",
    "is_orange",
]
