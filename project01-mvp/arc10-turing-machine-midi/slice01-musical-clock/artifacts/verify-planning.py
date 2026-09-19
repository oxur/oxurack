#!/usr/bin/env python3
"""Bounded Clock packet checks; structural evidence, not implementation acceptance."""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[4]
SLICE = Path('project01-mvp/arc10-turing-machine-midi/slice01-musical-clock')
ASSIGNMENT = 'arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md'
FILES = [SLICE / name for name in ('slice-plan.md', 'ledger.md', 'cc-prompt.md')]
FILES += [SLICE.parent / name for name in ('arc-plan.md', 'design.md', 'ledger.md')]
FILES += [Path('project01-mvp/cdc-directive07.md')]


def validate(texts):
    errors = []
    if f'Current assignment: `{ASSIGNMENT}`' not in texts[FILES[0]]:
        errors.append('current assignment mismatch')
    if ASSIGNMENT not in texts[FILES[2]]:
        errors.append('prompt identity mismatch')
    rows = [line.split('|') for line in texts[FILES[1]].splitlines()
            if re.match(r'^\| C-\d\d \|', line)]
    if [row[1].strip() for row in rows] != [f'C-{n:02}' for n in range(1, 7)]:
        errors.append('six-row denominator mismatch')
    if any(row[6].strip() not in ('open', 'done') for row in rows):
        errors.append('invalid row status')
    for path, body in texts.items():
        for target in re.findall(r'\[[^\]]+\]\(([^)]+)\)', body):
            if target.startswith(('https:', 'http:', '#')):
                continue
            filename = target.split('#', 1)[0]
            if filename and not (ROOT / path.parent / filename).exists():
                errors.append(f'broken link: {path}: {target}')
    return errors


if __name__ == '__main__':
    texts = {path: (ROOT / path).read_text() for path in FILES}
    errors = validate(texts)
    if errors:
        raise SystemExit('\n'.join(errors))
    print('Clock packet baseline passed: assignment, six rows, statuses, files/links')
    for name in ('missing-row', 'stale-assignment', 'invalid-status'):
        changed = dict(texts)
        if name == 'missing-row':
            changed[FILES[1]] = re.sub(r'^\| C-06 \|.*\n', '', changed[FILES[1]], flags=re.M)
        elif name == 'stale-assignment':
            changed[FILES[0]] = changed[FILES[0]].replace(ASSIGNMENT, 'wrong/cc-prompt.md')
        else:
            changed[FILES[1]] = changed[FILES[1]].replace('| open |', '| invalid |', 1)
            if changed[FILES[1]] == texts[FILES[1]]:
                changed[FILES[1]] = changed[FILES[1]].replace('| done |', '| invalid |', 1)
        assert validate(changed), f'negative control accepted: {name}'
        print(f'negative control rejected: {name}')
