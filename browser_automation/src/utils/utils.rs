use std::future::Future;
use std::time::Duration;
use rand::Rng;
use std::error::Error;
use fantoccini::Client;
use std::fs::File;
use std::io::Write;
use crate::elements::elements::WebElement;
use serde_json::Value;

//Retry 

pub async fn retry_with_backoff<F, T>(
    mut task: F,
    max_retries: u32,
    backoff: Duration,
) -> Result<T, Box<dyn Error>>
where
    F: FnMut() -> Box<dyn Future<Output = Result<T, Box<dyn Error>>> + Unpin>,
{
    let mut attempts = 0;
    let mut delay = backoff;

    while attempts < max_retries {
        match task().await {
            Ok(result) => return Ok(result),
            Err(_) if attempts < max_retries - 1 => {
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
                attempts += 1;
            }
            Err(e) => return Err(e), // Return the last error
        }
    }

    Err("Retry limit reached".into())
}


//log helper
pub fn log_action(action: &str) {
    println!("[INFO] {}", action);
}

pub fn log_error(error: &str) {
    eprintln!("[ERROR] {}", error);
}

//Wait for Element
pub async fn wait_for_element<F>(
    task: F,
    timeout: Duration,
    poll_interval: Duration,
) -> Result<WebElement, Box<dyn Error>>
where
    F: Fn() -> Box<dyn Future<Output = Result<WebElement, Box<dyn Error>>> + Unpin>,
{
    let mut elapsed = Duration::ZERO;

    while elapsed < timeout {
        match task().await {
            Ok(element) => return Ok(element),
            Err(_) => {
                tokio::time::sleep(poll_interval).await;
                elapsed += poll_interval;
            }
        }
    }

    Err("Element not found within the timeout period".into())
}

//Random delays
pub async fn random_delay(min_ms: u64, max_ms: u64) {
    let delay = rand::thread_rng().gen_range(min_ms..=max_ms);
    tokio::time::sleep(Duration::from_millis(delay)).await;
}

//screen shot
pub async fn take_screenshot(client: &Client, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let screenshot = client.screenshot().await?;
    let mut file = File::create(filename)?;
    file.write_all(&screenshot)?;
    Ok(())
}



//JavaScript Executor
pub async fn execute_js(client: &Client, script: &str, args: Vec<Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let result = client.execute(script, args).await?;
    Ok(result)
}