use super::*;

#[test]
fn insert_first() {
    let list = List::new(1).insert_first(2);
    assert_eq!(list.current(), 2);
    assert_eq!(list.get_element_n(1).unwrap().current(), 1);
}

#[test]
fn insert_last() {
    let mut list = List::new(1);
    list.insert_last(2);
    assert_eq!(list.current(), 1);
    assert_eq!(list.get_element_n(1).unwrap().current(), 2);
}

#[test]
fn insert_after_n() {
    let mut list = List::new(1);
    list.insert_last(2);
    list.insert_last(3);
    list.insert_last(4);
    list.insert_last(5);

    list.insert_after_n(6, 1);
    list.insert_after_n(7, 3);
    list.insert_after_n(8, 4);

    assert_eq!(list.get_element_n(2).unwrap().current(), 6);
    assert_eq!(list.get_element_n(4).unwrap().current(), 7);
    assert_eq!(list.get_element_n(5).unwrap().current(), 8);
}

#[test]
fn move_for_n() {
    let mut list = List::new(1);
    list.insert_last(2);
    list.insert_last(3);
    list.insert_last(4);
    list.insert_last(5);

    list.move_for_n(2, 2);
    assert_eq!(list.get_element_n(4).unwrap().current(), 3);

    list.move_for_n(1, 3);
    assert_eq!(list.get_element_n(4).unwrap().current(), 2);

    list.move_for_n(0, 1);
    assert_eq!(list.get_element_n(1).unwrap().current(), 1);
}
