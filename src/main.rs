use clap::*;
use hyeong::state::{State, UnOptState};
use hyeong::{build, debug, execute, interpreter, io, optimize};
use std::io::{stderr, stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

/// Main function of this program
///
/// ```text
/// hyeong 0.1.0
/// hyeo-ung programming language tool
///
/// USAGE:
///     hyeong [SUBCOMMAND]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// SUBCOMMANDS:
///     build        Compiles hyeong code
///     check        Parse your code and check if you are right
///     debug        Debug your code command by command
///     help         Prints this message or the help of the given subcommand(s)
///     install      Install hyeong before build (need once)
///     run          Run hyeong code directly
///     uninstall    Uninstall hyeong before build
/// ```
#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("hyeong")
        .version("0.1.0")
        .about("hyeo-ung programming language tool")
        .subcommand(
            App::new("build")
                .about("Compiles hyeong code")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to compile"),
                )
                .arg(
                    Arg::with_name("optimize")
                        .value_name("optimize")
                        .takes_value(true)
                        .short("O")
                        .long("optimize")
                        .help("optimize level (0: no optimize, 1: basic optimize, 2: hard optimize")
                        .default_value("2"),
                )
                .arg(
                    Arg::with_name("output")
                        .value_name("output")
                        .takes_value(true)
                        .short("o")
                        .long("output")
                        .help("binary output file (filename by default)"),
                ),
        )
        .subcommand(
            App::new("check")
                .about("Parse your code and check if you are right")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to check"),
                ),
        )
        .subcommand(
            App::new("debug")
                .about("Debug your code command by command")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to debug"),
                )
                .arg(
                    Arg::with_name("from")
                        .value_name("from")
                        .takes_value(true)
                        .short("f")
                        .long("from")
                        .help("place to start debugging from")
                        .default_value("0"),
                ),
        )
        .subcommand(
            App::new("run")
                .about("Run hyeong code directly")
                .arg(
                    Arg::with_name("input")
                        .value_name("input_file")
                        .takes_value(true)
                        .required(true)
                        .help("input file to run"),
                )
                .arg(
                    Arg::with_name("optimize")
                        .value_name("optimize")
                        .takes_value(true)
                        .short("O")
                        .long("optimize")
                        .help(
                            "optimize level (0: no optimize, 1: basic optimize, 2: hard optimize)",
                        )
                        .default_value("2"),
                ),
        )
        .subcommand(App::new("install").about("Install hyeong before build (need once)"))
        .subcommand(App::new("uninstall").about("Uninstall hyeong before build"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("build") {
        let file = matches.value_of("input").unwrap();
        let un_opt_code = io::read_file(file);
        let level_str = matches.value_of("optimize").unwrap();
        let level = io::handle_error(level_str.parse::<usize>());
        let output_file = match matches.value_of("output") {
            Some(v) => v.to_string(),
            None => {
                let v = file.split(".").collect::<Vec<_>>();
                v[..v.len() - 1].join(".")
            }
        };

        let source = if level >= 1 {
            let (state, opt_code) = optimize::optimize(un_opt_code, level);
            io::print_log("compiling to rust");
            build::build_source(state, &opt_code, level)
        } else {
            let state = UnOptState::new();
            io::print_log("compiling to rust");
            build::build_source(state, &un_opt_code, 0)
        };
        if !Path::new(&*io::get_build_path()).exists() {
            io::print_log("making temporary crate");
            io::execute_command_stderr(
                &*format!(
                    "cargo new {} --color always --vcs none",
                    io::get_build_path()
                ),
                &*format!(
                    "cargo new {} --color always --vcs none",
                    io::get_build_path()
                ),
            );
        }
        io::save_to_file(&*(io::get_build_path() + "/src/main.rs"), source);
        io::print_log("compiling rust code");
        io::execute_command_stderr(
            &*format!(
                "cargo build --manifest-path={}\\Cargo.toml --release --color always",
                io::get_build_path()
            ),
            &*format!(
                "cargo build --manifest-path={}/Cargo.toml --release --color always",
                io::get_build_path()
            ),
        );
        io::print_log("moving binary to current directory");
        if cfg!(target_os = "windows") {
            io::handle_error(Command::new("cmd").arg("/C").arg(format!(
                "copy %USERPROFILE%\\.hyeong\\hyeong-build\\target\\release\\hyeong-build.exe {}.exe",
                output_file
            )).output())
        } else {
            io::handle_error(
                Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "cp \"$HOME\"/.hyeong/hyeong-build/target/release/hyeong-build {}",
                        output_file
                    ))
                    .output(),
            )
        };
        io::print_log("done!");
    } else if let Some(ref matches) = matches.subcommand_matches("check") {
        let file = matches.value_of("input").unwrap();
        let code = io::read_file(file);
        for c in code.iter() {
            println!("{}:{}", file, c.to_string())
        }
    } else if let Some(ref matches) = matches.subcommand_matches("debug") {
        let file = matches.value_of("input").unwrap();
        let code = io::read_file(file);
        let from = io::handle_error(matches.value_of("from").unwrap().parse::<usize>());
        debug::run(code, from);
    } else if let Some(ref matches) = matches.subcommand_matches("run") {
        let file = matches.value_of("input").unwrap();
        let un_opt_code = io::read_file(file);
        let level_str = matches.value_of("optimize").unwrap();

        let level = io::handle_error(level_str.parse::<usize>());
        let mut stdout = stdout();
        let mut stderr = stderr();

        if level >= 1 {
            let (mut state, opt_code) = optimize::optimize(un_opt_code, level);
            io::print_log("running code");

            if !state.get_stack(1).is_empty() {
                for num in state.get_stack(1).iter() {
                    io::write(
                        &mut stdout,
                        &*format!("{}", num.floor().to_int() as u8 as char),
                    );
                }
                io::handle_error(stdout.flush());
                state.get_stack(1).clear();
            }

            if !state.get_stack(2).is_empty() {
                for num in state.get_stack(2).iter() {
                    io::write(
                        &mut stderr,
                        &*format!("{}", num.floor().to_int() as u8 as char),
                    );
                }
                io::handle_error(stderr.flush());
                state.get_stack(2).clear();
            }
            for code in opt_code {
                state = execute::execute(&mut stdin(), &mut stdout, &mut stderr, state, &code);
            }
        } else {
            let mut state = UnOptState::new();
            io::print_log("running code");
            for code in un_opt_code {
                state = execute::execute(&mut stdin(), &mut stdout, &mut stderr, state, &code);
            }
        }
    } else if let Some(ref _m) = matches.subcommand_matches("install") {
        io::print_log("installing hyeong");
        io::execute_command_stderr(
            "\
            mkdir %USERPROFILE%\\.hyeong\n\
            cd %USERPROFILE%\\.hyeong && cargo new hyeong-build --vcs none\n\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/number.rs\" > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\number.rs;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/big_number.rs\" > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\big_number.rs;\
            echo pub mod big_number;pub mod number; > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\lib.rs",
            "\
            mkdir -p ~/.hyeong;\
            cd ~/.hyeong && cargo new hyeong-build --vcs none --color always;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/number.rs\" > ~/.hyeong/hyeong-build/src/number.rs;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/big_number.rs\" > ~/.hyeong/hyeong-build/src/big_number.rs;\
            printf \"pub mod big_number;\npub mod number;\" > ~/.hyeong/hyeong-build/src/lib.rs"
        );
        io::print_log("test build");
        io::execute_command_stderr(
            "\
            cargo build --manifest-path=%USERPROFILE%\\.hyeong\\hyeong-build\\Cargo.toml --release",
            "\
            cargo build --manifest-path=\"$HOME\"/.hyeong/hyeong-build/Cargo.toml --release --color always",
        );
        io::print_log("done!");
        io::print_note("to uninstall, run `hyeong uninstall`");
    } else if let Some(ref _m) = matches.subcommand_matches("uninstall") {
        io::print_log("uninstalling hyeong");
        io::execute_command_stdout("rmdir /S %USERPROFILE%\\.hyeong", "rm -rf ~/.hyeong");
        io::print_log("done!");
    } else {
        interpreter::run();
    }
}
