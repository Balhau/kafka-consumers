# Kafka Consumers

## Because...


[Apache Kafka](https://kafka.apache.org/) is the de facto journal engine of the industry. Currently most of the distributed systems in place somehow use Kafka as a means to cope with the scalability need and as a robust way to decouple components that are the building blocks of very complex systems. 


### Some concepts

![Apache Logo](https://kafka.apache.org/images/kafka_diagram.png)

The logo above can be seen on the official page of kafka and kind of sums up the core ideas. Kafka is basically an engine that exposes a set of interfaces called topics. These interfaces can be though of like a set of channels for which people write data and from which people read data. Furthermore topics (you can visualize them as conduits of data) are composed of a set of partitions. In this sense partitions are the real conduits and topics just a logical aggregation o partitions. Think of it like a [rj45 cable](https://en.wikipedia.org/wiki/Modular_connector#8P8C)

![RJ45 Cable](https://upload.wikimedia.org/wikipedia/commons/6/61/RJ-45_TIA-568A_Left.png)


The topic can be seen as the whole cable and the partitions as one of each of the eight independent wires. The same properties of the cable apply to topics. There is no concept of ordering in the topic because data can change order between the wires. But on a per wire view there is ordering, electric pulses are send in a serialized form on the **rj45** cable, the same applies to topic partitions.

Now that we know the building blocks of kafka, topics and partitions, we can move forward. 
RJ45 cables alone don't do much, actually they are pretty useless. But when we put computers in the equation RJ45 are a pretty useful tool to connect them. Another nice feature of creating a network of computers is that a nice set of features arise. If for instance one computer in the network goes down the others continue to run as if nothing happens. This is why the internet is so reliable and resilient. Because is formed of billions of decoupled systems that somehow interact in a collaborative way. 

This same principle also applies to large systems. Big projects are composed of thousand of smaller components that live and work independently. Topics are just a means for them to communicate.


### So why kafka-consumers

When we comunicate we usually do that by talking. When we talk the ratio at which one speak is more or less matched by the person that is listening. 

![Exorcist](https://i.pinimg.com/736x/40/96/6f/40966f820c46ffea84843d7b0f06dd9d--exorcist-movie-the-exorcist.jpg)

But imagine this possessed girl trashing out a thousand words per minute. It would be virtually impossible to tackle that speed. Some seconds and you would be completely out of context, even if the pretty demon speaks with coherence. So you can apply a strategy you call 60 friends and each of one is responsible to track one second a time of information. How this resembles kafka, topics and partitions? You can think of this 60 friends as 60 topic partitions. And send each second of information to each partition within the topic **little.daemon.girl**. At the end of each partition would be a person reading the content of each partition that enters at a human velocity.

Kafka-Consumers is a **Rust** command tool that aims to extract and publish, mainly for time series databases, information regarding