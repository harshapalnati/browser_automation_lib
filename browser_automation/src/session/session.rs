use fantoccini::{Client, ClientBuilder,Locator};
use std::error::Error;
use crate::elements::elements::WebElement;

pub struct BrowserSession
{
    client :Client,
}

impl BrowserSession
{

      /// Sets the browser window size
      pub async fn set_window_size(&self, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
        self.client
            .set_window_rect(0, 0, width, height) // Provide explicit values for all arguments
            .await?;
        Ok(())
    }
    
    pub async fn new(website_url: &str) -> Result<Self, Box<dyn Error>> {
        // Construct the capabilities map
        let mut chrome_options = serde_json::Map::new();
        chrome_options.insert("args".to_string(), serde_json::Value::Array(vec![]));
    
        let mut capabilities = serde_json::Map::new();
        capabilities.insert(
            "goog:chromeOptions".to_string(),
            serde_json::Value::Object(chrome_options),
        );
    
        // Connect to the WebDriver
        let client = ClientBuilder::native()
            .capabilities(capabilities) // Pass the capabilities as Map<String, Value>
            .connect("http://localhost:9515")
            .await
            .map_err(|e| {
                eprintln!("Failed to connect to WebDriver: {}", e);
                e
            })?;
    
        // Navigate to the specified URL
        client.goto(website_url).await?;
    
        // Wrap the Client in BrowserSession and return
        Ok(Self { client })
    }
    
    pub async fn  navigate(&mut self,url:&str)->Result<(),Box<dyn Error>>
    {
        self.client.goto(url).await?;
        Ok(())
    }

    pub async fn close(self) -> Result<(), Box<dyn Error>> {
        self.client.close().await?;
        Ok(())
    }

    pub async fn find_element(&mut self, selector: &str) -> Result<WebElement, Box<dyn Error>> {
        let element = self.client.find(Locator::Css(selector)).await?;
        Ok(WebElement { element })
    }
}


//test Cases

#[cfg(test)]
mod test
{
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_navigation()
    {
        let website_url = "https://indeed.com/?r=us";
       // Create a browser session
       let mut session = BrowserSession::new("https://indeed.com/?r=us").await.unwrap();

       // Navigate to a new page
       let navigate_result = session.navigate("https://www.linkedin.com/jobs/").await;
       assert!(navigate_result.is_ok(), "Failed to navigate to Rust website");

        
       // Close the session
       let close_result = session.close().await;
       assert!(close_result.is_ok(), "Failed to close the browser session");
    }
}