use clap::Parser;
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, DescribeLogStreamsRequest,
    LogStream,
};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    log_group: String,
}

// To run this: cargo run -- --log-group bla
#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let aws_logs = AWSLogs::new();
    // aws_logs.get_log_streams(log_group).await;
    aws_logs.get_log_groups().await;
}

struct AWSLogs {
    client: CloudWatchLogsClient,
}

impl AWSLogs {
    fn new() -> AWSLogs {
        AWSLogs {
            client: CloudWatchLogsClient::new(Region::EuCentral1),
        }
    }
    async fn get_log_streams(&self, log_group: &str) {
        let req = DescribeLogStreamsRequest {
            log_group_name: log_group.to_owned(),
            ..Default::default()
        };

        match self.client.describe_log_streams(req).await {
            Ok(resp) => match resp.log_streams {
                Some(log_streams) => print_data::<LogStream>(log_streams).await,
                None => println!("Nothing found"),
            },
            Err(error) => {
                println!("Error: {:?}", error);
                if error.to_string() == "ThrottlingException" {
                    println!("boo")
                }
            }
        }
    }

    async fn get_log_groups(&self) {
        let mut finished = false;

        while !finished {
            let req = DescribeLogGroupsRequest::default();
            let resp = match self.client.describe_log_groups(req).await {
                Ok(response) => response,
                Err(error) => {
                    println!("Error found: {error}");
                    return;
                }
            };

            let groups = resp.log_groups.unwrap();

            for group in groups.iter() {
                let name = group.log_group_name.as_ref().unwrap();
                println! {"{}", name}
            }

            if resp.next_token.unwrap() == "" {
                finished = true
            }
        }
    }
}

async fn print_data<T: Debug>(streams: Vec<T>) {
    for stream in streams.iter().take(5) {
        println!("Stream {:?}", stream);
        println!("")
    }
}


