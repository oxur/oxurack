from pathlib import Path
import json
base=Path('/tmp/oxurack-cdc-arc09/probe');(base/'src').mkdir(parents=True,exist_ok=True)
src='/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation'
(base/'Cargo.toml').write_text(f'''[package]
name="cdc-arc09-probe"
version="0.0.0"
edition="2024"
[workspace]
[features]
default=["oxurack/graph"]
[dependencies]
oxurack={{path="{src}/crates/oxurack",default-features=false}}
oxurack-core={{path="{src}/crates/oxurack-core"}}
bevy_app={{version="=0.18.1",default-features=false,features=["std"]}}
bevy_ecs={{version="=0.18.1",default-features=false,features=["std"]}}
bevy_time={{version="=0.18.1",default-features=false,features=["std"]}}
''')
fixture='''(version:"1.0",master_seed:42,bpm:120.0,modules:[
(kind:"cdc_probe",instance_name:"tm_a",parameters:{"offset":Int(2)}),
(kind:"cdc_probe",instance_name:"tm_b",parameters:{"offset":Int(3)})],
cables:[(source:("tm_a","out"),target:("tm_b","in"))])'''
(base/'probe.ron').write_text(fixture)
(base/'reordered.ron').write_text(fixture.replace('(kind:"cdc_probe",instance_name:"tm_a",parameters:{"offset":Int(2)}),\n(kind:"cdc_probe",instance_name:"tm_b",parameters:{"offset":Int(3)})','(kind:"cdc_probe",instance_name:"tm_b",parameters:{"offset":Int(3)}),\n(kind:"cdc_probe",instance_name:"tm_a",parameters:{"offset":Int(2)})'))
(base/'changed.ron').write_text(fixture.replace('master_seed:42','master_seed:43'))
# Independent expected values from the approved byte stream, not Rust's derive_seed.
def seed(master,name):
 h=14695981039346656037
 b=name.encode()
 for c in b'oxurack.module-seed.v1\0'+master.to_bytes(8,'little')+len(b).to_bytes(8,'little')+b:h=((h^c)*1099511628211)&((1<<64)-1)
 return h
state=[seed(42,n)%1000 for n in ['tm_a','tm_b']];rows=[]
for i in range(3):
 outputs=state[:];inputs=[0,outputs[0]]
 state=[(outputs[j]*17+[2,3][j]+inputs[j])%1000 for j in range(2)]
 for j,n in enumerate(['tm_a','tm_b']):rows.append(f'{i}|{n}|{outputs[j]}|{inputs[j]}|{state[j]}')
(base/'expected.txt').write_text('\n'.join(rows)+'\n')
(base/'src/main.rs').write_text(r'''use std::{collections::HashMap,path::Path};
use bevy_app::{App,FixedUpdate};
use bevy_ecs::{prelude::*,schedule::{InternedScheduleLabel,IntoScheduleConfigs}};
use bevy_time::TimeUpdateStrategy;
use oxurack::{Oxurack,SimulationDriver};
use oxurack_core::*;
#[derive(Component)] struct Probe { input:Entity, output:Entity, state:u16, offset:u16 }
#[derive(Resource,Default)] struct Trace(Vec<String>);
struct Kind;
impl OxurackModule for Kind {
 const KIND:&'static str="cdc_probe"; const DISPLAY_NAME:&'static str="CDC composition probe";
 fn port_schema()->&'static [PortSchema] { &[
 PortSchema{name:"in",direction:PortDirection::Input,value_kind:ValueKind::Raw,merge_policy:MergePolicy::Reject,description:"input"},
 PortSchema{name:"out",direction:PortDirection::Output,value_kind:ValueKind::Raw,merge_policy:MergePolicy::Reject,description:"output"}] }
 fn parameter_schema()->&'static [ParameterSchema] { &[ParameterSchema{name:"offset",description:"offset",default:ParameterValue::Int(0)}] }
 fn spawn(_: &mut World,_:&str,_:&HashMap<String,ParameterValue>)->Result<Entity,CoreError>{panic!("context path required")}
 fn spawn_with_context(w:&mut World,c:&ModuleSpawnContext<'_>,p:&HashMap<String,ParameterValue>)->Result<Entity,CoreError>{
 let e=spawn_module_entity(w,Self::KIND,c.instance_name());
 let input=spawn_port_on_module(w,e,"in",PortDirection::Input,ValueKind::Raw,MergePolicy::Reject);
 let output=spawn_port_on_module(w,e,"out",PortDirection::Output,ValueKind::Raw,MergePolicy::Reject);
 let offset=match p.get("offset").unwrap(){ParameterValue::Int(v)=>*v as u16,_=>panic!()};
 w.entity_mut(e).insert(Probe{input,output,state:(c.instance_seed()%1000) as u16,offset});Ok(e)
 }
 fn register_systems(a:&mut App,s:InternedScheduleLabel){a.init_resource::<Trace>().add_systems(s,(produce.in_set(TickPhase::Produce),consume.in_set(TickPhase::Consume),observe.in_set(TickPhase::Observe)));}
}
fn raw(v:&CurrentValue)->u16{match v.0{Value::Raw(v)=>v,_=>panic!("wrong signal type")}}
fn produce(q:Query<&Probe>,mut v:Query<&mut CurrentValue>){for p in &q{v.get_mut(p.output).unwrap().0=Value::Raw(p.state);}}
fn consume(mut q:Query<&mut Probe>,v:Query<&CurrentValue>){for mut p in &mut q{p.state=(p.state*17+p.offset+raw(v.get(p.input).unwrap()))%1000;}}
fn observe(q:Query<(&Module,&Probe)>,v:Query<&CurrentValue>,t:Res<SampleTime>,mut log:ResMut<Trace>){let mut rows:Vec<_>=q.iter().map(|(m,p)|(m.instance_name.clone(),format!("{}|{}|{}|{}|{}",t.last_index().unwrap(),m.instance_name,raw(v.get(p.output).unwrap()),raw(v.get(p.input).unwrap()),p.state))).collect();rows.sort_by(|a,b|a.0.cmp(&b.0));log.0.extend(rows.into_iter().map(|(_,s)|s));}
fn main(){let args:Vec<_>=std::env::args().collect();let mut r=Oxurack::new();r.install::<Kind>().install::<Kind>();r.load_patch(Path::new(&args[2])).unwrap();let app=if args[1]=="manual"{
 r.set_driver_mode(SimulationDriver::Manual);assert_eq!(r.advance_samples(1).unwrap(),0);r.start();assert_eq!(r.advance_samples(2).unwrap(),2);r.try_install::<Kind>().unwrap();r.stop();assert_eq!(r.advance_samples(2).unwrap(),0);r.start();assert_eq!(r.advance_samples(1).unwrap(),1);assert_eq!(r.sample_time().completed_samples(),3);r.into_app()
 }else{r.start();let mut a=r.into_app();for n in [1,2]{a.insert_resource(TimeUpdateStrategy::FixedTimesteps(n));a.update();}assert_eq!(a.world().resource::<SampleTime>().completed_samples(),3);a};
 for row in &app.world().resource::<Trace>().0{println!("{row}");}
}
''')
print((base/'expected.txt').read_text())
