use aws_sdk_cloudwatchlogs::{Client, Error, Region, PKG_VERSION};

// TODO: Apparently rusoto is not the official aws sdk
// create an implementation of the same here using the sdk
async fn get_log_groups() {
    let client = Client::new(Default::default());
}
