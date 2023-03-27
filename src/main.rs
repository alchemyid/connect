
use std::process::Command;
use std::env;
use std::path::Path;
extern crate pbr;
use pbr::ProgressBar;
use std::thread;
use std::time::Duration;
use dialoguer::{theme::ColorfulTheme, Confirm};

// using vector
// fn command(c: &str, args: &str) {
//     let strings: Vec<&str> = args.split(" ").collect();
//     println!("{:?}", strings);
//     let output = Command::new(c)
//             .args(strings)
//             .output()
//             .expect("Command not found");  
//             println!("{}", String::from_utf8_lossy(&output.stdout));
// }

fn command(c: String) -> Vec<u8> {
    let output = Command::new("sh")
            .arg("-c")
            .arg(c.trim())
            .output()
            .expect("Command not found");
            // println!("{}", String::from_utf8_lossy(&output.stdout).trim());
            return output.stdout
        
}

fn help() -> String {
    let s = String::from(
    "
    Simplify switch kubernetes context!

    Commands Available:
    get         show full list contexts config kubernetes
    backup      backup existing kubectl to $HOME/.kube/backup   
    merge       merge kubeconfig with existing kubeconfig in $HOME/.kube/config
    delete      delete config from list
    rename      rename  context

    Usage:
    connect <string>                            define string connection
    connect get                                 show full list contexts
    connect get --simple                        show list context filter by name
    connect merge <fullpath file>               will execute backup first before merge
    connect delete <string>                     delete context, cluster, users
    connect backup                              will save file name with format config_$(date +%Y_%m_%d_%I_%M_%S_%p)
    connect rename <old context> <new context>  rename context

    Notes:
    <string> defined using 'grep' so you can type similar name context, cluster or user.
    ");
    s
    
}

fn bar (c: u64) {
    let count = c;
    let mut pb = ProgressBar::new(count);
    pb.format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
    pb.show_message = true;
    for _ in 0..count {
        pb.tick();
        thread::sleep(Duration::from_millis(1000));
    }
    return pb.finish_print("- done");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", help());
    }else {
        let query= &args[1];
        if query == "get" && args.len() == 2 {
            // command("kubectl","config get-contexts"); using vector
            let p = command("kubectl config get-contexts".to_string());
            println!("{}", String::from_utf8_lossy(&p).trim());
        
        } else if query == "get" && (&args[2] == "--simple"){
            let simple = command("kubectl config get-contexts --output=name".to_string());
            println!("{}", String::from_utf8_lossy(&simple).trim());
        } else if query == "backup" {
            command("mkdir -p $HOME/.kube/backup".to_string());
            command("cp $HOME/.kube/config $HOME/.kube/backup/config_$(date +%Y_%m_%d_%I_%M_%S_%p)".to_string());
            bar(1);
        } else if query == "merge" && args.len() == 3 {
            // let mut rs:bool=true;
            let rs = Path::new(&args[2].trim()).is_file();
            if rs == true{
                command("mkdir -p $HOME/.kube/backup".to_string());
                let backup = command("echo '- backup process.' && cp $HOME/.kube/config $HOME/.kube/backup/config_$(date +%Y_%m_%d_%I_%M_%S_%p)".to_string());
                println!("{}", String::from_utf8_lossy(&backup).trim());
                bar(1);
                // println!("\n");
                let merge = command("echo '- add new kubeconfig..' &&
                                            export KUBECONFIG=$HOME/.kube/config:".to_string()+&args[2].trim()+ " &&
                                            echo '- process merge..' &&
                                            kubectl config view --flatten > /tmp/config &&
                                            mv /tmp/config $HOME/.kube/config");
                println!("{}", String::from_utf8_lossy(&merge).trim());
                bar(2);
            }
            else{
                println!("{}", help())
            }
        } else if query == "delete" && args.len() == 3 {
            let q = &args[2];
            let get_context = command(format!("{}{}","kubectl config get-contexts --output name | grep ",q));
            let get_cluster = command(format!("{}{}","kubectl config get-clusters | grep ",q));
            let get_user = command(format!("{}{}","kubectl config get-users | grep ",q));

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue delete context ".to_owned()+String::from_utf8_lossy(&get_context).trim()+" ?")
                .interact()
                .unwrap()
            {
                let context = command(format!("{}{}","kubectl config delete-context ",String::from_utf8_lossy(&get_context).trim()));
                println!("{}",String::from_utf8_lossy(&context).trim());
            } else {
                println!("Nevermind then :)");
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue delete cluster ".to_owned()+String::from_utf8_lossy(&get_cluster).trim()+" ?")
                .interact()
                .unwrap()
            {
                let cluster = command(format!("{}{}","kubectl config delete-cluster ",String::from_utf8_lossy(&get_cluster).trim()));
                println!("{}",String::from_utf8_lossy(&cluster).trim());
            } else {
                println!("Nevermind then :)");
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue delete user ".to_owned()+String::from_utf8_lossy(&get_user).trim()+" ?")
                .interact()
                .unwrap()
            {
                let user = command(format!("{}{}","kubectl config delete-user ",String::from_utf8_lossy(&get_user).trim()));
                println!("{}",String::from_utf8_lossy(&user).trim());
            } else {
                println!("Nevermind then :)");
            }
        } else if query == "rename" && args.len() == 4{        
            let q = &args[2];
            let change = &args[3];
            let get_context = command(format!("{}{}","kubectl config get-contexts --output name | grep ",q));
            
            if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to continue rename context ".to_owned()+String::from_utf8_lossy(&get_context).trim()+" ?")
            .interact()
            .unwrap()
            {
                command("kubectl config rename-context ".to_string()+String::from_utf8_lossy(&get_context).trim()+ " "+change);
                let p = command("kubectl config get-contexts".to_string());
                println!("{}", String::from_utf8_lossy(&p).trim());

            } else {
                println!("Nevermind then :)");
            }

        } else if query == "--help" || query == "help" || query == "merge" {
            println!("{}", help());

        }else if args.len() == 2 {
            let c = command(format!("{}{}","kubectl config get-contexts --output name | grep ",query));
            let f = command(format!("{}{}","kubectl config use-context ",String::from_utf8_lossy(&c).trim()));
            println!("{}",String::from_utf8_lossy(&f).trim())
        }else{
            println!("{}", help());
        }
    }
}
