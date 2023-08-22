use super::*;

const TEST_DB_PATH: &str = "./_tests/db_tests/dbs/db_test.json";
const TEST_DB_BAD_FORMATED_PATH: &str = "./_tests/db_tests/dbs/db_bad_formated.json";

#[test]
fn get_tabs_test_ok() -> Result<(), Box<dyn std::error::Error>> {
    let tabs = match Db::get_tabs(TEST_DB_PATH) {
        Ok(tabs) => tabs,
        Err(e) => return Err(Box::new(e)),
    };

    assert_eq!(tabs.tabs[0].name, "Main".to_string());
    assert_eq!(tabs.tabs[1].name, "Tab1".to_string());
    Ok(())
}

#[test]
fn get_tabs_test_err_db_not_found() -> Result<(), Box<dyn std::error::Error>> {
    match Db::get_tabs("Incorrect path") {
        Ok(_) => assert!(false),
        Err(e) => match e {
            DbError::DbNotFound(path) => {
                assert_eq!(path, "Incorrect path");
            }
            DbError::DbBadFormat => assert!(false),
        },
    };

    Ok(())
}

#[test]
fn get_tabs_test_err_db_bad_format() -> Result<(), Box<dyn std::error::Error>> {
    match Db::get_tabs(TEST_DB_BAD_FORMATED_PATH) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, DbError::DbBadFormat),
    };

    Ok(())
}
