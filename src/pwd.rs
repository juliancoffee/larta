use std::path::{self, PathBuf};
use std::env;

extern crate dirs;

pub fn current_path(need_short: bool) -> String {
    let cur_dir = env::current_dir().expect("failed to get current directory");
    
    let sep = get_sep();
    if need_short {
        short_pwd(cur_dir, sep)
    } else {
        pwd(cur_dir)
    }
}


fn pwd(current_dir: PathBuf) -> String {
    let path = current_dir.to_str()
        .expect("failed to parse current path").to_string();
    let home = home();
    let home = home.to_str().expect("failed to parse home directory");
    path.replacen(&home, "~", 1)
}

fn short_pwd(current_dir: PathBuf, sep: String) -> String {
    let home_path = home();
    let separator = sep;

    if current_dir == home_path {
        return String::from("~")
    }
    if current_dir == PathBuf::from(&separator) {
        return separator;
    }

    let home = home_path.to_str().expect("failed to parse home directory");
    let sep = path::MAIN_SEPARATOR;

    let path = current_dir.to_str().expect("failed to get current path");
    let path_str = path.replacen(&home, "~", 1);
    let head = path_str.split(sep).last().expect("failed to get head of path")
        .to_string();

    let short_path: String = path_str.split(sep).enumerate()
        .map(|(i, s)| match (i, s) {
            (0, s) => format!("{}", s),
            (_, s) => if s == head {
                format!("{}", head)
            } else { 
                format!("{}", s.chars().nth(0)
                    .expect("failed to get first character of str"))
            }
        })
        .collect::<Vec<String>>()
        .join(&separator);

    short_path
}

fn get_sep() -> String {
    path::MAIN_SEPARATOR.to_string()
}

fn home() -> PathBuf {
    dirs::home_dir().expect("failed to get home directory")
}

#[cfg(test)]
mod test_short {
    use super::*;

    #[test]
    fn test_home() {
        assert_eq!(short_pwd(home(), get_sep()), String::from("~"));
    }

    mod linux {
        use super::*;

        #[test]
        fn root() {
            assert_eq!(
                short_pwd(
                    PathBuf::from("/"),
                    "/".to_string(),
                    )
                , "/");
        }

        #[test]
        fn one_dir() {
            assert_eq!(
                short_pwd(
                    PathBuf::from("/usr"),
                    "/".to_string(),
                    )
                , "/usr");
        }
    }

    mod windows {
        use super::*;

        #[test]
        fn windows_drive() {
            assert_eq!(
                short_pwd(
                    PathBuf::from(r"C:\"),
                    r"\".to_string(),
                    )
                , r"C:\");
        }

        #[test]
        fn one_dir() {
            assert_eq!(
                short_pwd(
                    PathBuf::from(r"C:\Users"),
                    r"\".to_string(),
                    )
                , r"C:\Users");
        }
    }
}

#[cfg(test)]
mod test_long {
    use super::*;

    #[test]
    fn test_home() {
        assert_eq!(pwd(home()), String::from("~"));
    }

    #[test]
    fn linux_root() {
        assert_eq!(pwd(PathBuf::from(get_sep())), get_sep());
    }

    #[test]
    fn windows_drive() {
        assert_eq!(
            pwd(
                PathBuf::from(r"C:\"))
            , r"C:\");
    }
}
