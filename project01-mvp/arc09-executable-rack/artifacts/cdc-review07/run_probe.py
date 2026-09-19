from pathlib import Path
import subprocess,os,tomllib,json
out=Path('/tmp/oxurack-cdc-arc09-review07');b=out/'probe';src=Path('/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation');exe=src/'target/debug/cdc-arc09-probe'
expected=(b/'expected.txt').read_text();rows=[]
versions={p['name']+':'+p['version'] for p in tomllib.loads((src/'Cargo.lock').read_text())['package']};pv={p['name']+':'+p['version'] for p in tomllib.loads((b/'Cargo.lock').read_text())['package']};assert pv-versions=={'cdc-arc09-probe:0.0.0'}
for feature in ['default','no-default']:
 cmd=['cargo','build','--offline','--locked','--manifest-path',str(b/'Cargo.toml')]+(['--no-default-features'] if feature=='no-default' else [])
 with (out/f'probe-final-build-{feature}.log').open('w') as log:r=subprocess.run(cmd,env={**os.environ,'CARGO_TARGET_DIR':str(src/'target')},stdout=log,stderr=subprocess.STDOUT)
 assert r.returncode==0
 for name,mode,fixture in [('manual-process1','manual','probe.ron'),('manual-process2','manual','probe.ron'),('live-grouped','live','probe.ron'),('reordered','manual','reordered.ron'),('changed-seed','manual','changed.ron'),('mid-stop','mid-stop','probe.ron')]:
  cmd=[str(exe),mode,str(b/fixture)];r=subprocess.run(cmd,capture_output=True,text=True);log=f'probe-final-{feature}-{name}.log';(out/log).write_text(r.stdout+r.stderr)
  if name=='mid-stop':
   assert r.returncode==0 and 'returned=1 completed=1' in r.stdout and 'fixed_elapsed=1ms virtual_elapsed=1ms' in r.stdout,(r.returncode,r.stdout,r.stderr)
   verdict='repair reproduced: requested 3, returned/completed 1, clocks 1ms'
  else:
   assert r.returncode==0,r.stderr
   assert (r.stdout!=expected) if name=='changed-seed' else (r.stdout==expected),(name,r.stdout)
   verdict='expected trace relationship reproduced'
  rows.append(dict(feature=feature,case=name,command=cmd,exit=r.returncode,result=verdict,log=log));print(feature,name,r.returncode,verdict,flush=True)
(out/'probe-results.json').write_text(json.dumps(rows,indent=2)+'\n')
