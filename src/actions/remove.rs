use crate::args::RemoveArgs;
use crate::config::UserConfig;

pub fn mprs_remove(args: &RemoveArgs, config: &UserConfig) {
    println!("{:?}", config);
    println!("{:?}", args);
}
