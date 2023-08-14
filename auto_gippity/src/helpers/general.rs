pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> String {

    let ai_func_str = ai_func(func_input);

    let msg = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of a function. 
    Nothing Else. No commentary. Here is the input to the function: {}. 
    Print out what the function will return", ai_func_str, func_input);

    msg

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
}