# Independent seed-vector reference

CRC calculated these values before slice03 implementation. This reference
script uses Python byte operations rather than Rust's current `derive_seed`.
It is planning evidence for the fixed CDC §3 byte contract, not production
code or a substitute for CC's Rust tests.

```python
def derive(master_seed: int, name: str) -> int:
    encoded = name.encode("utf-8")
    data = (
        b"oxurack.module-seed.v1\x00"
        + master_seed.to_bytes(8, "little")
        + len(encoded).to_bytes(8, "little")
        + encoded
    )
    value = 14695981039346656037
    for byte in data:
        value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return value
```

| Master | Name | Hex output |
| ---: | --- | --- |
| 0 | empty | `a854da736b8befab` |
| 42 | `tm_a` | `21819f1bfbd4fcea` |
| 42 | `tm_b` | `21819e1bfbd4fb37` |
| 43 | `tm_a` | `5365fb0ec2a0228b` |
| 0 | `vco_1` | `fda42c6e50ff0d2a` |
| `u64::MAX` | `é` | `199408757f0d299d` |

For `é`, `len(encoded)` is 2. The distinct master/name rows expose omitted
fields, wrong ordering and accidental use of character count. The empty name
tests the length-zero boundary. These vectors validate derivation only; a
fresh-process SmallRng trace needs its own independent test fixture.
