use std::env;
use std::process::Command;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() > 1 && argv[1] == "create" {
        if argv.len() > 2 && argv[2] == "project" {
            if argv.len() > 4 {
                if argv[3] == "react" {
                    Command::new("npx")
                            .args(["create-react-app", &argv[4]]);
                    println!("Project created.");
                }
                if argv[3] == "svelte" {
                    Command::new("npm")
                            .args(["create", "svelte@latest", &argv[4]]);
                    println!("Project created.");
                }
                if argv[3] == "vue" {
                    Command::new("npm")
                            .args(["init", "vue@latest"]);
                    println!("Project created.");
                }
                if argv[3] == "next" {
                    Command::new("npx")
                            .args(["create-next-app@latest"]);
                    println!("Project created.");
                }
            } else {
                println!("\x1b[0;91m[Error]\x1b[0;0m Project argument provided, but no project type provided OR no project name provided. Project Types:");
                println!("\x1b[0;94mReact (react)");
                println!("\x1b[0;93mSvelte (svelte)");
                println!("\x1b[0;92mVue (vue)");
                println!("\x1b[0;92mNext.js (next)");
            }
        } else {
            println!("\x1b[0;91m[Error]\x1b[0;0m Nothing to create! Try project.")
        }
    } else {
        println!("\x1b[0;92mUsage\x1b[0;0m: \x1b[0;93mjscli \x1b[0;0mcreate project [FRAMEWORK] [PROJECT NAME]");
        println!("\x1b[4;94m[FRAMEWORK]\x1b[0;0m: \x1b[1;94mframework\x1b[0;0m to use");
        println!("\x1b[4;94m[PROJECT NAME]\x1b[0;0m: \x1b[1;94mname\x1b[0;0m for the project");
    }
}
