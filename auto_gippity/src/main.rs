mod ai_functions;
mod models;
mod helpers;
mod apis;




use helpers::command_line::{get_user_response};



fn main() {
    
    let user_req = get_user_response("What webserver would you like to build today ?");

    dbg!(user_req);
}



