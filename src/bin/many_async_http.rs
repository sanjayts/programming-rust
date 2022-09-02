fn main() {
    let urls = [
        "https://www.google.co.uk".to_string(),
        "https://www.bhalor.co.uk".to_string(),
        "https://sanjayts.net".to_string(),
    ];
    let results = async_std::task::block_on(request_many(&urls));
    for (res, req) in results.iter().zip(urls.iter()) {
        println!("URL: {} and response: {:?}", req, res);
    }
}

async fn request_many(urls: &[String]) -> Vec<Result<String, surf::Error>> {
    let client = surf::Client::new();
    let mut handles = vec![];

    for url in urls {
        let req = client.get(url).recv_string();
        let handle = async_std::task::spawn(req);
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}
