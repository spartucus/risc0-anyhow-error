use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // !!! Guest panicked: sys_getenv is disabled; can be enabled with the sys-getenv feature flag on risc0-zkvm-platform !!!
    // bar_with_anyhow(input).unwrap();

    // This code works!
    bar_without_anyhow(input).unwrap();

    // write public output to the journal
    env::commit(&input);
}


fn foo(x: u32) -> Option<u32> {
    Some(x)
}

fn bar_with_anyhow(x: u32) -> anyhow::Result<u32> {
    let f = foo(x).ok_or(anyhow::anyhow!("I'm just a anyhow error!!!"))?;
    Ok(f)
}

fn bar_without_anyhow(x: u32) -> Result<u32, String> {
    let f = foo(x).ok_or(format!("Im just a normal error!!!"))?;
    Ok(f)
}