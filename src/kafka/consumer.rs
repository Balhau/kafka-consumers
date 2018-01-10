use std::vec::Vec;
use std::process::Command;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaConsumerEntry {
    topic: String,
    partition: u16,
    current_offset: u64,
    log_end_offset: u64,
    lag: u64,
    consumer_id: String,
    host: String,
    client_id: String,
    group: String
}

pub trait Metric{
    fn toMetric(&self) -> String;
}

impl Metric for KafkaConsumerEntry{
    fn toMetric(&self) -> String {
        let consumer_offset = format!("consumer_offset,consumer_group={},consumer_id={},topic={},partition={} value={}",self.group,self.consumer_id,self.topic,self.partition,self.current_offset);
        let consumer_lag = format!("consumer_lag,consumer_group={},consumer_id={},topic={},partition={} value={}",self.group,self.consumer_id,self.topic,self.partition,self.lag);
        consumer_offset+"\n"+&consumer_lag
    }
}

pub struct KafkaCommands{}

impl KafkaCommands {
    pub fn groupDescribe(path: &str, server : &str,group : &str) -> Vec<KafkaConsumerEntry> {
        let output = Command::new(path)
            //.arg("Hello world")
            .arg("--bootstrap-server")
            .arg(server)
            .arg("--describe")
            .arg("--group")
            .arg(group)
            .output()
            .expect("Failed to execute command");

        let describe = String::from_utf8(output.stdout.as_slice().to_vec()).unwrap();

        KafkaConsumerEntry::parse(String::from(group),describe)
    }

    fn parseListConsumers(outputConsumers : String) -> Vec<String> {
        let l : Vec<String> = outputConsumers.split("\n").map(|e : &str| String::from(e)).collect();
        l[0..l.len()-1].to_vec()
    }

    pub fn listConsumers(path : &str, server: &str) -> Vec<String> {
        let output = Command::new(path)
            .arg("--bootstrap-server")
            .arg(server)
            .arg("--list")
            .output()
            .expect("Failed to execute command");
        let list_consumers = String::from_utf8(output.stdout.as_slice().to_vec()).unwrap();
        KafkaCommands::parseListConsumers(list_consumers)
    }
}


impl KafkaConsumerEntry {

    pub fn parse(group: String,shell_output: String) -> Vec<KafkaConsumerEntry> {

        let mut vec: Vec<KafkaConsumerEntry> = Vec::new();

        let lines: Vec<&str> = shell_output.split("\n").collect();

        let lines = &lines[2..lines.len()-1].to_vec();


        for line in lines {
            let re = Regex::new(r"\s+").unwrap();
            let line = re.replace_all(line, " ");
            let cols: Vec<&str> = line.split(" ").collect();
            let entry = KafkaConsumerEntry {
                topic: String::from(cols[0]),
                partition: String::from(cols[1]).parse::<u16>().unwrap(),
                current_offset: String::from(cols[2]).parse::<u64>().unwrap_or(0),
                log_end_offset: String::from(cols[3]).parse::<u64>().unwrap_or(0),
                lag: String::from(cols[4]).parse::<u64>().unwrap_or(0),
                consumer_id: String::from(cols[5]),
                host: String::from(cols[6]),
                client_id: String::from(cols[7]),
                group : group.clone()
            };

            vec.push(entry);
        }

        vec
    }
}
