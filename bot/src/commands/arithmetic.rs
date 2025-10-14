use framework::prelude::*;
use framework::command;

#[command]
/// Adds two numbers together
pub async fn add(a: f64, b: f64) -> CommandResult {
    let res = (a + b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["sub"])]
/// Subtracts two numbers together
pub async fn subtract(a: f64, b: f64) -> CommandResult {
    let res = (a - b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["mul", "mult"])]
/// Multiplies two numbers together
pub async fn multiply(a: f64, b: f64) -> CommandResult {
    let res = (a * b).to_string();
    CommandOk::from_same(res).into()
}

#[command(aliases = ["div"])]
/// Divides two numbers together
pub async fn divide(a: f64, b: f64) -> CommandResult {
    let res = (a / b).to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Returns the value of pi
pub async fn pi() -> impl Into<CommandResult> {
    let res = std::f64::consts::PI.to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Calculates the sine of a number
pub async fn sin(x: f64) -> CommandResult {
    let res = x.sin().to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Calculates the cosine of a number
pub async fn cos(x: f64) -> CommandResult {
    let res = x.cos().to_string();
    CommandOk::from_same(res).into()
}

#[command]
/// Calculates the tangent of a number
pub async fn tan(x: f64) -> CommandResult {
    let res = x.tan().to_string();
    CommandOk::from_same(res).into()
}

