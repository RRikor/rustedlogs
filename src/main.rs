use clap::Parser;
use rusoto_core::Region;
use rusoto_logs::{CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, LogStream};

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

    let log_group = "/aws/lambda/SpotrDomainApi-Worker-prd";
    print_log_stream(log_group).await;
}

async fn print_log_stream(log_group: &str) {
    let client = CloudWatchLogsClient::new(Region::EuCentral1);

    let req = DescribeLogStreamsRequest {
        log_group_name: log_group.to_owned(),
        ..Default::default()
    };

    match client.describe_log_streams(req).await {
        Ok(resp) => match resp.log_streams {
            Some(log_streams) => print_the_streams(log_streams).await,
            None => println!("Nothing found"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

async fn print_the_streams(streams: Vec<LogStream>) {
    for stream in streams.iter().take(5) {
        println!("Stream {:?}", stream);
        println!("")
    }
}
