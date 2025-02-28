import re
from dataclasses import dataclass


def parse_stringlist(value: str) -> list[str]:
    return value.split()


def parse_datetime(value: str) -> list[str]:
    return value.split()


_DURATION_PATTERN = re.compile(
    r"^P(\d+Y)?(\d+M)?(\d+W)?(\d+D)?(?:T(\d+H)?(\d+M)?(\d+S)?)?$"
)


@dataclass
class Duration:
    years: int
    months: int
    weeks: int
    days: int
    hours: int
    minutes: int
    seconds: int

    @staticmethod
    def from_iso8601_duration(s: str) -> "Duration":
        m = _DURATION_PATTERN.search(s)
        if not m:
            raise ValueError(f"Invalid ISO 8601 duration: {s}")

        result = Duration(0, 0, 0, 0, 0, 0, 0)
        matches = m.groups()
        if matches[0]:
            result.years = int(matches[0][:-1])
        if matches[1]:
            result.months = int(matches[1][:-1])
        if matches[2]:
            result.weeks = int(matches[2][:-1])
        if matches[3]:
            result.days = int(matches[3][:-1])
        if matches[4]:
            result.hours = int(matches[4][:-1])
        if matches[5]:
            result.minutes = int(matches[5][:-1])
        if matches[6]:
            result.seconds = int(matches[6][:-1])

        return result
