use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{confirm_safe_code, PrintCommand},
        general::{
            ai_task_request, check_status_code, read_code_template_contents,
            read_exec_main_contents, save_api_endpoints, save_backend_code,
            WEB_SERVER_PROJECT_PATH,
        },
    },
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::agent_traits::{FactSheet, RouteObject, SpecialFunctions},
    },
};
use async_trait::async_trait;
use reqwest::Client;
use std::{
    fs,
    process::{Command, Stdio},
    time::Duration,
};
use tokio::time;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_cont: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develop backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_cont: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        let mut msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);

        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);

        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n THIS FUNCTION ONLY OUTPUT CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_backend_code(&ai_response);

        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code = read_exec_main_contents();

        let msg_context: String = format!("CODE_INPUT: {:?}", backend_code);

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }
}
#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_cont == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code unit Testing: Requesting user input",
                    );

                    let is_safe_code = confirm_safe_code();

                    if !is_safe_code {
                        panic!("Better go work on some AI aliment instead..")
                    }

                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position.as_str(),
                        "Backend Code unit Testing: building project...",
                    );

                    let build_backend_server: std::process::Output = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::piped())
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to run backend application");

                    if build_backend_server.status.success() {
                        self.bug_cont = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            &self.attributes.position.as_str(),
                            "Backend Code unit Testing: Test server build successful...",
                        );
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str: String = String::from_utf8(error_arr).unwrap();

                        self.bug_cont += 1;
                        self.bug_errors = Some(error_str);

                        if self.bug_cont > 2 {
                            PrintCommand::Issue.print_agent_message(
                                &self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs found in code.",
                            );
                            panic!("Error: Too many bugs");
                        }

                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_writing_backend_developer() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"
        {
            "project_description": "build a website that fetches and tracks fitness progress with timezone information",
            "project_scope": {
                "is_crud_required": true,
                "is_user_login_and_logout": true,
                "is_external_urls_required": true
            },
            "external_urls": [
                "http://worldtimeapi.org/api/timezone"
            ],
            "backend_code": null,
            "api_endpoint_schema": null
        }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}
