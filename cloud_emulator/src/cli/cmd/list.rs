pub fn execute(cloud_provider: String) {
    println!("Listing services for cloud provider: {}", cloud_provider);
    match cloud_provider.as_str() {
        "aws" => {
            println!("Available AWS services:");
            println!("AWS - S3");
        }
        "gcp" => {
            println!("Available GCP services:");
            println!("- Cloud Storage");
        }
        "azure" => {
            println!("Available Azure services:");
            println!("- Blob Storage");
        }
        _ => println!("Unknown cloud provider"),
    }
}
