# Iteration02 assignment-checker fixtures

The helper `check_current_assignment` was exercised through the temporary
script below from the source checkout. The real planning tree was not mutated
to create any failure case. The raw run is preserved in
`artifacts/logs/25-verifier-fixtures-dirty.txt`.

Command:

```text
python3 /private/tmp/oxurack_iteration02_verifier_fixtures.py
```

Exact temporary script:

```python
import importlib.util
import tempfile
from pathlib import Path

checker_path = Path('/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py')
spec = importlib.util.spec_from_file_location('verify_planning', checker_path)
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)

PREFIX = 'arc09-executable-rack/slice01-module-behavior-installation/'
BASE_FILES = ('cc-prompt.md', 'cc-prompt-iteration01.md')


def run_case(label, current_line, history_name, extra_files=(), expected='fail'):
    with tempfile.TemporaryDirectory(prefix='oxurack-verify-') as root:
        project = Path(root) / 'project01-mvp'
        slice_dir = project / 'arc09-executable-rack' / 'slice01-module-behavior-installation'
        slice_dir.mkdir(parents=True)
        for name in BASE_FILES + tuple(extra_files):
            (slice_dir / name).write_text(name + '\n')
        history = f'| 2026-09-17 | {history_name} | predecessor | test | fixture |'
        plan = '\n'.join(filter(None, (current_line, '## Assignment history', history)))
        try:
            checker.check_current_assignment(plan, slice_dir)
        except RuntimeError as error:
            outcome = f'FAIL {error}'
        else:
            outcome = 'PASS'
        print(f'{label}: {outcome}')
        if expected == 'fail' and not outcome.startswith('FAIL '):
            raise SystemExit(f'{label} unexpectedly passed')
        if expected == 'pass' and outcome != 'PASS':
            raise SystemExit(f'{label} unexpectedly failed')


run_case('missing-current', '', 'cc-prompt-iteration02.md')
run_case(
    'nonexistent-current',
    f'Current assignment: `{PREFIX}cc-prompt-iteration02.md`',
    'cc-prompt-iteration02.md',
)
run_case(
    'field-history-mismatch',
    f'Current assignment: `{PREFIX}cc-prompt-iteration02.md`',
    'cc-prompt-iteration01.md',
    extra_files=('cc-prompt-iteration02.md',),
)
run_case(
    'duplicate-current',
    f'Current assignment: `{PREFIX}cc-prompt-iteration02.md`\nCurrent assignment: `{PREFIX}cc-prompt-iteration02.md`',
    'cc-prompt-iteration02.md',
    extra_files=('cc-prompt-iteration02.md',),
)
run_case(
    'out-of-slice-path',
    'Current assignment: `arc09-executable-rack/slice02-other/cc-prompt-iteration02.md`',
    'cc-prompt-iteration02.md',
    extra_files=('cc-prompt-iteration02.md',),
)
run_case(
    'valid-alternate',
    f'Current assignment: `{PREFIX}cc-prompt-iteration09.md`',
    'cc-prompt-iteration09.md',
    extra_files=('cc-prompt-iteration09.md',),
    expected='pass',
)
```

Observed output:

```text
missing-current: FAIL Current assignment field count mismatch
nonexistent-current: FAIL Current assignment prompt is missing
field-history-mismatch: FAIL Current assignment history mismatch
duplicate-current: FAIL Current assignment field count mismatch
out-of-slice-path: FAIL Non-canonical current assignment path
valid-alternate: PASS
```

All six cases produced their specified outcomes: five invalid records failed,
and the valid alternate numbered assignment passed with its matching file and
history row.
