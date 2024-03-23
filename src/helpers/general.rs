use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use super::command_line::PrintCommand;
use crate::{apis::call_request::call_gpt, models::general::llm::Message};

const CODE_TEMPLATE_PATH: &str =
    "/Users/por-livinginsider/Desktop/living-projects/basic_auto_gpt/web_gpt_template/src/code_template.rs";

pub const EXEC_MAIN_PATH: &str =
    "/Users/por-livinginsider/Desktop/living-projects/basic_auto_gpt/web_gpt_template/src/main.rs";

pub const WEB_SERVER_PROJECT_PATH: &str =
    "/Users/por-livinginsider/Desktop/living-projects/basic_auto_gpt/web_gpt_template/";

const API_SCHEMA_PATH: &str =
    "/Users/por-livinginsider/Desktop/living-projects/basic_auto_gpt/schemas/api_schema.rs";

pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    let msg: String = format!(
        "FUNCTION: {} 
        INSTRUCTION: You are a function printer.
        You ONLY print the result of functions. Nothing else, No commerces.
        Here is  the input to the function: {}.
        Print out what the function will return.",
        ai_function_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call GPT"),
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decoded ai response from serde_json");

    decoded_response
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template.")
}

pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template.")
}

pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file.")
}

pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints  file.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dimmy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api request.".to_string();
        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;
        assert!(res.len() > 20);
    }
}
