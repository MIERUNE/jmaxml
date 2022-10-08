from jmaxml.custom_types import Duration


def test_duration():
    assert Duration.from_iso8601_duration("PT72H") == Duration(0, 0, 0, 0, 72, 0, 0)
    assert Duration.from_iso8601_duration("PT55S") == Duration(0, 0, 0, 0, 0, 0, 55)
    assert Duration.from_iso8601_duration("P12WT72M") == Duration(0, 0, 12, 0, 0, 72, 0)
    assert Duration.from_iso8601_duration("P3YT10S") == Duration(3, 0, 0, 0, 0, 0, 10)
    assert Duration.from_iso8601_duration("P1DT72H") == Duration(0, 0, 0, 1, 72, 0, 0)
