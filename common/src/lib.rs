use std::env;

pub fn get_input_path(module: &str, file: &str) -> Option<std::path::PathBuf>
{
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(error) => panic!("Problem getting executable path: {:?}", error)
    };

    exe_path.parent().
    and_then(|val| val.parent()).
    and_then(|val| val.parent()).
    map(|p| p.join(module)).
    map(|p| p.join(file))
    
}
