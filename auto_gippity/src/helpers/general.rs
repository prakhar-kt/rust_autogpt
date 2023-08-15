use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{helpers::command_line::PrintCommand, apis::call_request::call_chatgpt};

const CODE_TEMPLATE_PATH: &str = "/Users/hobbes/Documents/rust_autogpt/web_template/src/code_template.rs";

const EXEC_MAIN_PATH: &str = "/Users/hobbes/Documents/rust_autogpt/web_template/src/main.rs";

const API_SCHEMA_PATH: &str = "Users/hobbes/Documents/rust_autogpt/auto_gippity/schemas/api_schema.json";








pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> String {

    let ai_func_str = ai_func(func_input);

    let msg = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of a function. 
    Nothing Else. No commentary. Here is the input to the function: {}. 
    Print out what the function will return", ai_func_str, func_input);

    msg

}

//  Perform call to LLMs

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {

    // Extend AI function

    let extended_message = extend_ai_function(function_pass, &msg_context);

    // Print Command

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM Response

    let llm_response_res = call_chatgpt(vec![extended_message.clone()]).await;

    // Handle Success

    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_chatgpt(vec![extended_message.clone()])
                    .await
                    .expect("Failed twice to call OpenAI")

    }


    

}

// Perform a call to LLMs: Get Decoded Response

pub async fn ai_task_request_decoded<T:DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,

) -> T {

    let llm_response = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response = serde_json::from_str(llm_response.as_str())
                        .expect("Failed to decode ai response from serde_json");
    return decoded_response;

}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {

    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template

pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Could not read code template")
}

//  Save new backend

pub fn save_backend_code(contents: &String) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write to path");
}

// Save JSON API Endpoint Schema 

pub fn save_api_endpoints(api_endpoints: &String) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write api endpoints to file");

}

#[cfg(test)]

mod tests {

    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]

    fn tests_extend_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(extended_msg);
    }

    #[tokio::test]

    async fn tests_ai_task_request() {

        let ai_func_param = "Build me a webserver for making stock price api requests".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining User requirements",
            convert_user_input_to_goal
            ).await;

        dbg!(res);
    }
}