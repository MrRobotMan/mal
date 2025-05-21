mod readline;
pub use readline::{RlHelper, readline};

pub fn rep<S: Into<String>>(cmd: S) -> String {
    let cmd = cmd.into();
    let cmd = read(cmd);
    let cmd = eval(cmd);
    print(cmd)
}

fn read(input: String) -> String {
    input
}
fn eval(input: String) -> String {
    input
}

fn print(input: String) -> String {
    input
}

#[cfg(test)]
mod tests;
