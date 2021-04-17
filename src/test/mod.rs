use crate::db::*;
use lib::model::{Note, Storage};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_delete_note() {
        let res = delete_note();
        assert!(res.is_ok());
    }

    #[test]
    fn test_insert_note() {
        let mut db = state.database.get().unwrap();
        let parsed_title = title.url_decode().expect("Failed to decode title.");
        let res = insert_note('{"title":"Alien 4", "status":"1"}',&mut db,1);
        assert!(res.is_ok());
    }

}