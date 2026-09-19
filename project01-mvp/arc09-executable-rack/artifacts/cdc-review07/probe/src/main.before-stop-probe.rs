use std::{collections::HashMap,path::Path};
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
