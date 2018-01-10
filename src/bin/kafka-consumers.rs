#[macro_use]
extern crate serde_derive;
extern crate kafkaconsumers;
extern crate hyper;
extern crate tokio_core;
extern crate futures;
extern crate serde_yaml;



use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use kafkaconsumers::kafka::consumer::KafkaConsumerEntry;
use kafkaconsumers::kafka::consumer::KafkaCommands;
use kafkaconsumers::kafka::consumer::Metric;

use hyper::{Method, Request, Client};
use futures::Future;
use futures::Stream;
use hyper::client::FutureResponse;
use hyper::header::{ContentLength, ContentType};
use hyper::Body;
use tokio_core::reactor::Core;

use std::thread;
use std::time;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaGroupsConfigs {
    pub command_path: String,
    pub kafka_server: String,
    pub influx_server: String
}

fn load_configs(c_path : String) -> KafkaGroupsConfigs {
    let mut yaml_str = String::new();
    let mut f = File::open(c_path).expect("file not found");
    f.read_to_string(&mut yaml_str)
        .expect("something went wrong reading the file");

    serde_yaml::from_str(&yaml_str).unwrap()
}

fn main() {


    let conf_file = String::from("<path_for_configs>");
    let configs = load_configs(conf_file);

    println! ("Configs: {:?}",configs);


    loop{
        let list_consumers = KafkaCommands::listConsumers(&(configs.command_path),&(configs.kafka_server));
        let cpath = configs.command_path.clone();
        let kserver = configs.kafka_server.clone();
        for consumer in list_consumers{
            let c_clone = consumer.clone();
            let c_command_path = cpath.clone();
            let c_server = kserver.clone();
            let influx = configs.influx_server.clone();
            thread::spawn( move|| {
                let mut core = Core::new().unwrap();
                let client = Client::new(&core.handle());
                let stats = KafkaCommands::groupDescribe(&c_command_path,&c_server,&c_clone);
                for entry in stats.iter() {
                    let c_uri_influx = influx.parse().unwrap();
                    let mut req = Request::new(Method::Post, c_uri_influx);
                    req.set_body(entry.toMetric());

                    let post = client.request(req).and_then(|res| {
                        //println!("POST: {}", res.status());
                        res.body().concat2()
                    });

                    core.run(post);
                }
                println!("Metrics sent for {}",c_clone);
            });
        };

        thread::sleep(time::Duration::from_secs(5));
    }
}
