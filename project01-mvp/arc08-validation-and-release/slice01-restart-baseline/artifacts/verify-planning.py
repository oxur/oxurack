"""Structural restart checks; neither behavioral nor independent acceptance."""
import csv
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
PROJECT = HERE.parents[2]
WORKTREE = PROJECT.parent
BASELINE = 'f0cf3a5172e02c717c7f924c84156f5fe0405733'
SLICE = HERE.parent


def require(value, message):
    if not value:
        raise RuntimeError(message)


def old(path):
    return subprocess.check_output(
        ['git', 'show', f'{BASELINE}:{path.relative_to(WORKTREE).as_posix()}'],
        cwd=WORKTREE,
    )


def ledger_ids(path):
    return re.findall(r'^\| ((?:S|A|R|P)-[0-9-]+) \|', path.read_text(), re.M)


def main():
    with (PROJECT / 'artifacts/migration-manifest.tsv').open() as stream:
        imported = list(csv.DictReader(stream, delimiter='\t'))
    historical = {}
    for row in imported:
        path = WORKTREE / row['destination_path']
        before = old(path)
        after = path.read_bytes()
        if row['action'] == 'move-unchanged':
            require(after == before, f'Historical artifact changed: {path}')
        else:
            require(after.startswith(before), f'Imported plan text changed: {path}')
        historical[path] = before.decode()
    require(len(historical) == 23, 'Expected all 23 imported documents')
    print('PASS: 23 migrated documents retained; imported plan text preserved as prefix')

    legacy_slices = sorted(d for d in PROJECT.glob('arc*/slice*') if d != SLICE)
    require(len(legacy_slices) == 6, 'Expected six historical slices')
    legacy_arcs = sorted(d for d in PROJECT.glob('arc*') if int(d.name[3:5]) <= 6)
    require(len(legacy_arcs) == 6, 'Expected six started historical arcs')
    expected = set()
    for d in [*legacy_slices, *legacy_arcs]:
        ledger = d / 'ledger.md'
        before_rows = re.findall(r'^\| ((?:S|A)-[0-9-]+) \|(.+)$', old(ledger).decode(), re.M)
        current_rows = dict(re.findall(r'^\| ((?:S|A)-[0-9-]+) \|(.+)$', ledger.read_text(), re.M))
        for row_id, cells in before_rows:
            require(row_id in current_rows, f'Missing original row: {ledger}:{row_id}')
            require(cells.split('|')[:4] == current_rows[row_id].split('|')[:4], f'Original criterion/verify/significance/origin altered: {ledger}:{row_id}')
        ids = ledger_ids(ledger)
        require(len(ids) == 2, f'Wrong inherited row denominator: {ledger}')
        expected.update((ledger.relative_to(PROJECT).as_posix(), i) for i in ids)
    with (HERE / 'carryover-register.tsv').open() as stream:
        rows = list(csv.DictReader(stream, delimiter='\t'))
    actual = {(r['source_ledger'], r['row_id']) for r in rows}
    require(len(rows) == len(actual) == 24 and actual == expected, 'Carryover set mismatch')
    require(all(r['receiving_row'] == 'R-02' and r['capability_owner'] and r['reentry'] for r in rows), 'Missing owner/re-entry')
    for d in legacy_slices:
        report = (d / 'closing-report.md').read_text()
        require(re.findall(r'^\| (S-\d+) \| deferred \|', report, re.M) == ['S-01', 'S-02'], f'Wrong proposed row walk: {d}')
        require('Proposed only' in report, f'Premature close: {d}')
    print('PASS: 6 historical slices / 6 started arcs / 24 distinct rows; all criteria preserved')

    require(ledger_ids(SLICE / 'ledger.md') == [f'R-{i:02}' for i in range(1, 7)], 'Wrong restart row denominator')
    for name in ['slice-plan.md', 'ledger.md', 'cc-prompt.md']:
        require((SLICE / name).is_file(), f'Missing open set: {name}')
    require((PROJECT / 'cdc-directive01.md').is_file(), 'Missing preserved initial CRC assignment')
    require((PROJECT / 'cdc-directive02.md').is_file(), 'Missing current CRC assignment')
    require((SLICE / 'cc-prompt-iteration01.md').is_file(), 'Missing current CC assignment')
    require('Current assignment: `arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`' in (SLICE / 'slice-plan.md').read_text(), 'Active assignment mismatch')
    require(len(list(PROJECT.glob('arc*'))) == 8, 'Unexpected arc proliferation')
    require(len(list(PROJECT.glob('arc*/slice*'))) == 7, 'Expected six historical plus one restart slice')

    links = 0
    docs = [WORKTREE/'AGENTS.md', WORKTREE/'README.md', *PROJECT.rglob('*.md'), *(WORKTREE/'status').glob('*.md')]
    for doc in docs:
        text = doc.read_text()
        if doc in historical:
            text = text[len(historical[doc]):]
        for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', text):
            if '://' in target or target.startswith('#'):
                continue
            require((doc.parent / target.split('#')[0]).exists(), f'Broken current link: {doc}: {target}')
            links += 1
    print(f'PASS: current open set, initial directive, 8 arcs / 7 slices, {links} local links')
    print('SELF-CHECKED structural planning only; CRC/CDC review and all behavioral gates remain pending')


if __name__ == '__main__':
    main()
