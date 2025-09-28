use framework::prelude::*;
use framework::command;

#[command]
/// Adds two numbers together
async fn add(a: f64, b: f64) -> CommandResult {
    let res = (a + b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["sub"])]
/// Subtracts two numbers together
async fn subtract(a: f64, b: f64) -> CommandResult {
    let res = (a - b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["mul", "mult"])]
/// Multiplies two numbers together
async fn multiply(a: f64, b: f64) -> CommandResult {
    let res = (a * b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["div"])]
/// Divides two numbers together
async fn divide(a: f64, b: f64) -> CommandResult {
    let res = (a / b).to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Returns the value of pi
async fn pi() -> impl Into<CommandResult> {
    let res = std::f64::consts::PI.to_string();
    CommandOk::from_same(res)
}

#[command]
/// Calculates the sine of a number
async fn sin(x: f64) -> CommandResult {
    let res = x.sin().to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Calculates the cosine of a number
async fn cos(x: f64) -> CommandResult {
    let res = x.cos().to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Calculates the tangent of a number
async fn tan(x: f64) -> CommandResult {
    let res = x.tan().to_string();
    CommandOk::from_same(res).into()
}

