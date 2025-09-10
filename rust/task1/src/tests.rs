use super::*;

#[test]
fn insert_first() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let mut list = List::new(student1.clone());
    list.insert_first(student2.clone());
    assert_eq!(list.current(), &student2);
    assert_eq!(list.get_element_n(1).unwrap().current(), &student1);
}

#[test]
fn insert_last() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    assert_eq!(list.current(), &student1);
    assert_eq!(list.get_element_n(1).unwrap().current(), &student2);
}

#[test]
fn insert_after_n() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());

    list.insert_value_after_n(student1.clone(), 1);
    list.insert_value_after_n(student5.clone(), 3);
    list.insert_value_after_n(student3.clone(), 4);

    assert_eq!(list.get_element_n(2).unwrap().current(), &student1);
    assert_eq!(list.get_element_n(4).unwrap().current(), &student5);
    assert_eq!(list.get_element_n(5).unwrap().current(), &student3);
}

#[test]
fn insert_after_n_extra() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());

    list.insert_value_after_n(student1.clone(), 1);
}

#[test]
fn move_for_n() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());

    list.move_for_n(2, 2);
    assert_eq!(list.get_element_n(4).unwrap().current(), &student3);

    list.move_for_n(1, 3);
    assert_eq!(list.get_element_n(4).unwrap().current(), &student2);

    list.move_for_n(0, 1);
    assert_eq!(list.get_element_n(1).unwrap().current(), &student1);
}

#[test]
fn remove_nth() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());

    list.remove_nth(3);
    assert_eq!(list.get_element_n(3).unwrap().current(), &student5);

    list.remove_nth(3);
    assert_eq!(list.get_element_n(3).unwrap().current(), &student6);

    list.remove_nth(1);
    assert_eq!(list.get_element_n(1).unwrap().current(), &student3);

    list.remove_nth(0);
    assert_eq!(list.get_element_n(0).unwrap().current(), &student3);
}

#[test]
#[should_panic]
fn remove_nth_panic() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());

    list.remove_nth(3);
    list.remove_nth(3);
    list.remove_nth(3);

    list.get_element_n(3).unwrap();
}

#[test]
fn remove_every_nth() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");
    let student7 = Student::new("g", "g", "g", "2-3-2");
    let student8 = Student::new("h", "h", "h", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());
    list.insert_last(student7.clone());
    list.insert_last(student8.clone());

    list.remove_every_n(3);

    assert_eq!(list.get_element_n(2).unwrap().current(), &student4);
    assert_eq!(list.get_element_n(5).unwrap().current(), &student8);
}

#[test]
fn sorting_by_name() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");
    let student7 = Student::new("g", "g", "g", "2-3-2");
    let student8 = Student::new("h", "h", "h", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());
    list.insert_last(student7.clone());
    list.insert_last(student8.clone());

    list.sort(SearchCriteria::Name("".into()), false);
    assert_eq!(list.get_element_n(0).unwrap().current(), &student8);

    list.sort(SearchCriteria::Name("".into()), true);
    assert_eq!(list.get_element_n(0).unwrap().current(), &student1);
}

#[test]
fn sorting_by_rating() {
    let student1 = Student::new("a", "a", "a", "1-2-3").rating(91);
    let student2 = Student::new("b", "b", "b", "1-2-3").rating(100);
    let student3 = Student::new("c", "c", "c", "3-2-1").rating(81);
    let student4 = Student::new("d", "d", "d", "1-2-3").rating(79);
    let student5 = Student::new("e", "e", "e", "1-3-2").rating(95);
    let student6 = Student::new("f", "f", "f", "2-3-2").rating(74);
    let student7 = Student::new("g", "g", "g", "2-3-2").rating(100);
    let student8 = Student::new("h", "h", "h", "2-3-2").rating(97);

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());
    list.insert_last(student7.clone());
    list.insert_last(student8.clone());

    list.sort(SearchCriteria::Rating(0), false);
    assert_eq!(list.get_element_n(0).unwrap().current(), &student2);
    assert_eq!(list.get_element_n(1).unwrap().current(), &student7);
}

#[test]
#[should_panic]
fn clearing() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");
    let student7 = Student::new("g", "g", "g", "2-3-2");
    let student8 = Student::new("h", "h", "h", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());
    list.insert_last(student5.clone());
    list.insert_last(student6.clone());
    list.insert_last(student7.clone());
    list.insert_last(student8.clone());

    list.clear();
    list.get_element_n(1).unwrap();
}

#[test]
fn appending_two_lists() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("e", "e", "e", "1-3-2");
    let student6 = Student::new("f", "f", "f", "2-3-2");
    let student7 = Student::new("g", "g", "g", "2-3-2");
    let student8 = Student::new("h", "h", "h", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());

    let mut list2 = List::new(student5.clone());
    list2.insert_last(student6.clone());
    list2.insert_last(student7.clone());
    list2.insert_last(student8.clone());

    list.append(list2);

    assert_eq!(list.get_element_n(0).unwrap().current(), &student1);
    assert_eq!(list.get_element_n(4).unwrap().current(), &student5);
    assert_eq!(list.get_element_n(7).unwrap().current(), &student8);
}

#[test]
fn itersection() {
    let student1 = Student::new("a", "a", "a", "1-2-3");
    let student2 = Student::new("b", "b", "b", "1-2-3");
    let student3 = Student::new("c", "c", "c", "3-2-1");
    let student4 = Student::new("d", "d", "d", "1-2-3");
    let student5 = Student::new("a", "a", "a", "1-2-3");
    let student6 = Student::new("f", "f", "f", "2-3-2");
    let student7 = Student::new("b", "b", "b", "1-2-3");
    let student8 = Student::new("h", "h", "h", "2-3-2");

    let mut list = List::new(student1.clone());
    list.insert_last(student2.clone());
    list.insert_last(student3.clone());
    list.insert_last(student4.clone());

    let mut list2 = List::new(student5.clone());
    list2.insert_last(student6.clone());
    list2.insert_last(student7.clone());
    list2.insert_last(student8.clone());

    let intersectional_list = List::intersection(&list, &list2).unwrap();
    assert_eq!(
        intersectional_list.get_element_n(0).unwrap().current(),
        &student1
    );
    assert_eq!(
        intersectional_list.get_element_n(1).unwrap().current(),
        &student2
    );
}
