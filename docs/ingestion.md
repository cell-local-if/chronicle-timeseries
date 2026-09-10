# Delimited ingestion contract

Status: specified, not yet implemented.

The input is a UTF-8 byte stream containing one observation per line. A line has exactly four tab-separated fields:

```text
channel<TAB>timestamp_us<TAB>value<TAB>unit
```

There is no header, quoting, escaping, or comment syntax. For example:

```text
motor.temperature	1250000	23.5	Cel
motor.vibration	1250050	-0.012	m/s2
```

## Records and validation

Both LF and CRLF terminate records. A CR is treated as part of the terminator only when immediately followed by LF. A final nonempty line without LF is a record when the caller signals end of input; a trailing CR in that line is data and therefore invalid. Empty records are invalid. A stream ending immediately after LF does not contain an extra empty record.

The timestamp uses a signed base-10 integer spelling without whitespace. Its value must fit `i64`. The numeric value uses Rust's `f64` parsing rules without leading or trailing whitespace and must be finite. The other fields follow the observation contract. The byte stream must not silently trim fields, round a timestamp, replace invalid UTF-8, or substitute an unknown unit.

## Streaming behavior

Callers supply arbitrary byte chunks; a chunk boundary has no meaning to the record format. Completed records can be reported before end of input. Incomplete records remain pending until more bytes arrive or the caller finishes the input. Finishing is terminal: later input is rejected, and a second finish produces no additional records.

The caller supplies a positive maximum record size measured in bytes before LF, including CR when present. An oversized record produces one error at its starting position. Its remaining bytes are discarded through LF, and parsing then resumes with the next record. Pending record storage must be bounded by the configured limit regardless of the total stream length. A zero limit is invalid.

Each completed valid record produces a validated observation. An invalid record produces a diagnostic and does not prevent later records from being read. Report diagnostics in input order with a one-based line number, the zero-based absolute byte offset of the record start, and a reason that distinguishes size, encoding, shape, and field validation errors. Diagnostic payloads must not retain the entire rejected record. This behavior is independent of chunk boundaries.

Public API names are left to the implementation. The existing observation and batch interfaces retain their behavior. Small deterministic examples should demonstrate successful decoding and recovery after a rejected record.
