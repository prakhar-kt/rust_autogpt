// use crate::models::general::llm::{  ChatCompletion, APIResponse };
use dotenv::dotenv;
// use reqwest::Client;
use std::env;
use reqwest::header::{ HeaderMap, HeaderValue };
use std::error::Error;


use async_openai::{
    types::{ ChatCompletionRequestMessageArgs, CreateCompletionRequestArgs, Role, CreateImageRequestArgs, ResponseFormat, ImageSize },
    Client, config::OpenAIConfig,
};

// use fieri::(chat, ChatMessageBuilder, ChatParamBuilder,
//     Client, Error,
// };




// Call Large Language Model (i.e. GPT-4)

pub async fn call_chatgpt(messages: Vec<String>) -> Result<String, Box<dyn Error + Send>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.

    dotenv().ok();

    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");
    let org_id = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");
    let config = OpenAIConfig::new()
                    .with_api_key(api_key)
                    .with_org_id(org_id);

    let client = Client::with_config(config);


    // Create request
    let request = CreateCompletionRequestArgs::default()
                                                    .model("text-davinci-003")
                                                    .prompt(messages)
                                                    .max_tokens(40_u16)
                                                    .build()
                                                    .map_err(|e| -> Box<dyn Error + Send> {Box::new(e)})?;


    // Call API 
    let response = client
                                                    .completions()
                                                    .create(request)
                                                    .await
                                                    .map_err(|e| -> Box<dyn Error + Send> {Box::new(e)})?;

    

    dbg!(&response.choices.first().unwrap().text);
    
    Ok(response.choices.first().unwrap().text.clone())
}


#[cfg(test)]

mod tests {

    use super::*;

    #[tokio::test]

    async fn tests_call_to_openai() {
        
        let message = "Hello. I need a short response".to_string();

        let messages = vec!(message);

        let res = call_chatgpt(messages).await;

        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            },
            Err(e) => {
                dbg!(e);
                assert!(false);

            }
        }
    }
}