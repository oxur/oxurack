# CDC repaired-candidate review07 evidence

Source `37deeace90e7fe2dff633ada9d58ef9051577c3d`; planning intake is pinned in
provenance.txt. Product source was not edited. checks.json records 13 commands,
all exit zero: two focused tests, core, two umbrella modes, two host workspace
matrices, Clippy, format, two strict docs, make check and outdated.
Host access was requested
up front because the established MIDI initialization/network gates require it;
prior restricted failures remain in review06 and CC evidence.

probe-results.json records 12 independent public-rack probe processes: the ten
original positive controls and the two formerly failing Consume-stop cases.
Both feature modes now return/completed=1 with Fixed/Virtual=1ms for request3.
The runner additionally checks those exact clock/count strings, so a return-only
clamp fails. The connected two-instance seeded traces remain equal across fresh
processes, grouped Live and reordered spawn; changed seed differs. This fixture
is synthetic, not a TM or hardware demonstration. SmallRng fresh-process tests
also run in the package suites.

probe/ is the original reviewer fixture; run_probe.py rebuilds it against the
pinned source using that source's dependency versions. Paths in runners and the
manifest are local provenance: adjust them when replaying elsewhere. No public
accessor was added. child-walk.json checks all four existing child ledgers, six
done rows each. planning-before.log records Slice04's baseline and three negative
controls. Its scope is Slice04, not the forthcoming Clock assignment.

Raw command logs are preserved, including Cargo/make trailing blank lines.
Whitespace checks on prose/code exclude raw .log files; complete-packet checks
disable only blank-at-EOF for that reason. Three existing ignored RT tests and
the block 0.1.6 future-incompatibility notice remain explicit. No coverage,
hardware timing/playback, benchmark, soak or musical acceptance was performed.
