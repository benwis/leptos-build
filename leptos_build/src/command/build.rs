use crate::Cli;
use eyre::Result;
use shlex::Shlex;
use xshell::{cmd, Shell};

pub async fn build_all(state: &Cli) -> Result<()> {
    println!("BUILDING!");

    if state.opts.lib_only && state.opts.bin_only {
        panic!("Cannot set both lib-only and bin-only, that doesn't make sense");
    } else if state.opts.lib_only {
        build_lib(state)?;
    } else if state.opts.bin_only {
        build_bin(state)?;
    } else {
        build_lib(state)?;
        build_bin(state)?;
    }
    Ok(())
}

pub fn build_bin(state: &Cli) -> Result<()> {
    let bin_opts = state.opts.bin_opts.clone();

    // We need to check if the bin-cargo-commands length is greater than one word and add the second word to the args if so
    let bin_cargo_cmd = bin_opts.bin_cargo_command.unwrap();

    let mut command_iter = Shlex::new(&bin_cargo_cmd);

    if command_iter.had_error {
        panic!("bin-cargo-command cannot contain escaped quotes. Not sure why you'd want to")
    }

    let bin_cmd = command_iter
        .next()
        .expect("Failed to get bin command. This should default to cargo");
    let mut extra_cmd_args: Vec<String> = command_iter.collect();
    let default_bin_cargo_args = vec![
        "build".to_string(),
        format!("--package={}", state.bin_crate_name.clone().unwrap()),
        format!("--bin={}", state.bin_crate_name.clone().unwrap()),
        "--no-default-features".to_string(),
        format!(
            "--target={}",
            state.opts.bin_opts.bin_target_triple.clone().unwrap()
        ),
    ];
    let bin_cargo_args = bin_opts.bin_cargo_args.unwrap_or(default_bin_cargo_args);

    extra_cmd_args.extend(bin_cargo_args);

    let sh = Shell::new()?;
    Ok(cmd!(sh, "{bin_cmd} {extra_cmd_args...}").run()?)
}

pub fn build_lib(state: &Cli) -> Result<()> {
    let lib_opts = state.opts.lib_opts.clone();

    // We need to check if the bin-cargo-commands length is greater than one word and add the second word to the args if so
    let lib_cargo_cmd = lib_opts.lib_cargo_command.unwrap();

    let mut command_iter = Shlex::new(&lib_cargo_cmd);

    if command_iter.had_error {
        panic!("lib-cargo-command cannot contain escaped quotes. Not sure why you'd want to")
    }

    let lib_cmd = command_iter
        .next()
        .expect("Failed to get lib command. This should default to cargo");
    let mut extra_cmd_args: Vec<String> = command_iter.collect();
    let default_lib_cargo_args = vec![
        "build".to_string(),
        format!("--package={}", state.lib_crate_name.clone().unwrap()),
        "--lib".to_string(),
        "--no-default-features".to_string(),
        format!("--target={}", state.opts.lib_opts.lib_target_triple),
    ];
    let lib_cargo_args = lib_opts.lib_cargo_args.unwrap_or(default_lib_cargo_args);

    extra_cmd_args.extend(lib_cargo_args);

    let sh = Shell::new()?;
    Ok(cmd!(sh, "{lib_cmd} {extra_cmd_args...}").run()?)
}
