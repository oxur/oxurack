import os,subprocess,json,time
from pathlib import Path
root=Path('/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation')
out=Path('/tmp/oxurack-cdc-arc09')
commands=[('core',['cargo','test','--locked','-p','oxurack-core']),('umbrella',['cargo','test','--locked','-p','oxurack']),('umbrella-no-default',['cargo','test','--locked','-p','oxurack','--no-default-features']),('workspace-all',['cargo','test','--locked','--workspace','--all-features']),('workspace-no-default',['cargo','test','--locked','--workspace','--no-default-features']),('clippy',['cargo','clippy','--locked','--workspace','--all-features','--all-targets','--','-D','warnings']),('fmt',['cargo','fmt','--all','--','--check']),('docs',['cargo','doc','--locked','--workspace','--all-features','--no-deps']),('docs-no-default',['cargo','doc','--locked','-p','oxurack','--no-default-features','--no-deps'])]
rows=[]
for name,cmd in commands:
 env={**os.environ,'CARGO_NET_OFFLINE':'true'}
 if name.startswith('docs'):env['RUSTDOCFLAGS']='-Dwarnings'
 with (out/(name+'.log')).open('w') as log:
  r=subprocess.run(cmd,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
 rows.append(dict(name=name,command=cmd,cwd=str(root),source='642313a62db437ee2bf0b3d5b97fcdcffa16782a',exit=r.returncode,log=name+'.log',offline=True,strict_docs=name.startswith('docs')))
 (out/'checks.json').write_text(json.dumps(rows,indent=2)+'\n')
 print(name,r.returncode,flush=True)
