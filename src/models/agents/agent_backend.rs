use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::PrintCommand,
        general::{
            ai_task_request, check_status_code, read_code_template_contents,
            read_exec_main_contents, save_api_endpoints, save_backend_code,
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

    async fn call_initial_code(&mut self, factsheet: &mut FactSheet) {
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
