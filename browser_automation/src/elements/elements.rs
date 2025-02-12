use fantoccini::elements::Element;
use std::error::Error;
use serde_json::json;

use crate::session::session::BrowserSession;

pub struct WebElement
{
     pub element:Element,
}

impl WebElement
{
    //Click on the element simulation
    pub async fn click(&self)->Result<(),Box<dyn Error>>
    {
        self.element.click().await?;
        Ok(())
    }

    /// Types text into the element
    pub async fn type_text(&self, text: &str) -> Result<(), Box<dyn Error>> {
        self.element.send_keys(text).await?;
        Ok(())
    }

    /// Retrieves the text content of the element
    pub async fn get_text(&self) -> Result<String, Box<dyn Error>> {
        let text = self.element.text().await?;
        Ok(text)
    }

      /// Retrieves the value of a specific attribute (e.g., href, id)
      pub async fn get_attribute(&self, attr: &str) -> Result<Option<String>, Box<dyn Error>> {
        let value = self.element.attr(attr).await?;
        Ok(value)
    }

   /// Waits for an element to appear within a specified timeout (in seconds)
   pub async fn wait_for(&self, timeout: u64) -> Result<(), Box<dyn Error>> {
    let mut elapsed = 0;
    while elapsed < timeout {
        match self.element.attr("id").await {
            Ok(Some(_)) => return Ok(()),
            _ => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                elapsed += 1;
            }
        }
    }
    Err("Element not found within the timeout period".into())
}
    /// Checks if the element is visible
    pub async fn is_visible(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self.element.is_displayed().await?)
    }

    /// Checks if the element is enabled
    pub async fn is_enabled(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self.element.is_enabled().await?)
    }


    /// Clears text from the input field
    pub async fn clear(&self) -> Result<(), Box<dyn Error>> {
        self.element.clear().await?;
        Ok(())
    }

    pub async fn send_keys(&self, keys: &str) -> Result<(), Box<dyn Error>> {
        self.element.send_keys(keys).await?;
        Ok(())
    }

    
   /// Scroll the element into view using JavaScript
   pub async fn scroll_into_view(&self) -> Result<(), Box<dyn Error>> {
    let element_ref = self.element.element_id().clone();
    self.element.clone()
        .client()
        .execute(
            "arguments[0].scrollIntoView({ behavior: 'smooth', block: 'center' });",
            vec![json!(*element_ref)],
        )
        .await?;
    Ok(())
}


pub async fn is_displayed(&self) -> Result<bool, Box<dyn Error>> {
    let element_ref = self.element.element_id().to_string(); // Convert ElementRef to String
    let client = self.element.clone().client(); // Clone the element to avoid moving
    let result = client
        .execute(
            "return arguments[0].offsetWidth > 0 && arguments[0].offsetHeight > 0;",
            vec![json!(element_ref)],
        )
        .await?;

    if let Some(is_visible) = result.as_bool() {
        Ok(is_visible)
    } else {
        Ok(false) // Default to false if the script doesn't return a valid boolean
    }
}


}





#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
   async fn test_linkedin_login() {
    let mut session = BrowserSession::new("https://www.linkedin.com/login").await.unwrap();

    // Find and fill in the username field
    let username_field = session.find_element("input#username").await.unwrap();
    username_field.type_text("harshapalnati@gmail.com").await.unwrap();

    // Find and fill in the password field
    let password_field = session.find_element("input#password").await.unwrap();
    password_field.type_text("Harsha@0811").await.unwrap();

    // Click the sign-in button
    let sign_in_button = session.find_element("button.btn__primary--large").await.unwrap();
    sign_in_button.click().await.unwrap();

    session.close().await.unwrap();
}

}
