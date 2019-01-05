mod common;

use snowchains::app::{App, Opt};
use snowchains::service::ServiceName;
use snowchains::terminal::{AnsiColorChoice, TermImpl};

use failure::Fallible;
use serde_derive::Deserialize;

use std::fs::File;
use std::io;
use std::path::Path;

#[test]
fn it_logins() {
    let _ = env_logger::try_init();
    let credentials = common::credentials_from_env_vars().unwrap();
    common::test_in_tempdir("it_logins", credentials, login).unwrap();
}

fn login(mut app: App<TermImpl<io::Empty, io::Sink, io::Sink>>) -> snowchains::Result<()> {
    app.run(Opt::Login {
        color_choice: AnsiColorChoice::Never,
        service: ServiceName::Atcoder,
    })
}

#[test]
fn it_scrapes_samples_from_practice() {
    let _ = env_logger::try_init();
    let credentials = common::credentials_from_env_vars().unwrap();
    common::test_in_tempdir(
        "it_scrapes_samples_from_practice",
        credentials,
        |mut app| -> snowchains::Result<()> {
            app.run(Opt::Download {
                open: false,
                only_scraped: true,
                service: Some(ServiceName::Atcoder),
                contest: Some("practice".to_owned()),
                problems: vec![],
                color_choice: AnsiColorChoice::Never,
            })?;
            let download_dir = app
                .working_dir
                .join("tests")
                .join("atcoder")
                .join("practice");
            check_yaml_md5(&download_dir, "a", "0ab89f8622931867945de172a7696adb");
            check_yaml_md5(&download_dir, "b", "ae82294bcef243485432ffd958867396");
            Ok(())
        },
    )
    .unwrap();
}

#[test]
fn it_scrapes_samples_from_abc100() {
    let _ = env_logger::try_init();
    let credentials = common::credentials_from_env_vars().unwrap();
    common::test_in_tempdir(
        "it_scrapes_samples_from_abc100",
        credentials,
        |mut app| -> snowchains::Result<()> {
            app.run(Opt::Download {
                open: false,
                only_scraped: true,
                service: Some(ServiceName::Atcoder),
                contest: Some("abc100".to_owned()),
                problems: vec![],
                color_choice: AnsiColorChoice::Never,
            })?;
            let download_dir = app.working_dir.join("tests").join("atcoder").join("abc100");
            check_yaml_md5(&download_dir, "a", "f837ca06ddb61a1fd16f16455d30dcdc");
            check_yaml_md5(&download_dir, "b", "fbe5193dc50506c2b19b2fe0f1e77ccb");
            check_yaml_md5(&download_dir, "c", "48032dd70e600a2a9f139800a21b2c10");
            check_yaml_md5(&download_dir, "d", "84b981ce83866152ccef18517255e7b9");
            Ok(())
        },
    )
    .unwrap();
}

#[test]
fn it_scrapes_samples_and_download_files_from_abc099_a() {
    let _ = env_logger::try_init();
    let credentials = common::credentials_from_env_vars().unwrap();
    common::test_in_tempdir(
        "it_scrapes_samples_and_download_files_from_abc099_a",
        credentials,
        |mut app| -> snowchains::Result<()> {
            app.run(Opt::Download {
                open: false,
                only_scraped: false,
                service: Some(ServiceName::Atcoder),
                contest: Some("abc099".to_owned()),
                problems: vec!["a".to_owned()],
                color_choice: AnsiColorChoice::Never,
            })?;
            // "ARC058_ABC042"
            let download_dir = app.working_dir.join("tests").join("atcoder").join("abc099");
            just_confirm_num_samples_and_timelimit(&download_dir, "a", 9, "2000ms");
            Ok(())
        },
    )
    .unwrap();
}

fn check_yaml_md5(dir: &Path, name: &str, expected: &str) {
    let path = dir.join(name).with_extension("yaml");
    let yaml = std::fs::read_to_string(&path).unwrap();
    assert_eq!(format!("{:x}", md5::compute(&yaml)), expected);
}

fn just_confirm_num_samples_and_timelimit(dir: &Path, name: &str, n: usize, t: &str) {
    #[derive(Deserialize)]
    #[serde(tag = "type", rename_all = "lowercase")]
    enum TestSuite {
        Simple {
            timelimit: String,
            cases: Vec<serde_yaml::Mapping>,
        },
        Interactive {
            timelimit: String,
            each_args: Vec<serde_yaml::Sequence>,
        },
    }
    let path = dir.join(name).with_extension("yaml");
    let file = File::open(&path).unwrap();
    match serde_yaml::from_reader::<_, TestSuite>(file).unwrap() {
        TestSuite::Simple { timelimit, cases } => {
            assert_eq!(t, timelimit);
            assert_eq!(n, cases.len())
        }
        TestSuite::Interactive {
            timelimit,
            each_args,
        } => {
            assert_eq!(t, timelimit);
            assert_eq!(n, each_args.len())
        }
    }
}

#[test]
#[ignore]
fn it_submits_to_practice_a() {
    let _ = env_logger::try_init();
    let credentials = common::credentials_from_env_vars().unwrap();
    common::test_in_tempdir(
        "it_submits_to_practice_a",
        credentials,
        |mut app| -> Fallible<()> {
            static CODE: &[u8] = br#"def main():
    (a, (b, c), s) = (int(input()), map(int, input().split()), input())
    print('{} {}'.format(a + b + c, s))


if __name__ == '__main__':
    main()
"#;
            std::fs::create_dir(&app.working_dir.join("py"))?;
            std::fs::write(&app.working_dir.join("py").join("a.py"), CODE)?;
            app.run(Opt::Submit {
                open: false,
                force_compile: false,
                only_transpile: false,
                no_judge: true,
                no_check_duplication: false,
                service: Some(ServiceName::Atcoder),
                contest: Some("practice".to_owned()),
                language: Some("python3".to_owned()),
                jobs: None,
                color_choice: AnsiColorChoice::Never,
                problem: "a".to_owned(),
            })
            .map_err(Into::into)
        },
    )
    .unwrap();
}
