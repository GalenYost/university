#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn adding_students() {
        let mut group1 = Group::new("1-2-3");
        let s1 = Student::new("a", "a", "123456").rating(91);
        let s2 = Student::new("b", "b", "12345").rating(100);
        group1.add(s1).add(s2);
    }

    #[test]
    #[should_panic]
    fn remove_panic() {
        let mut group1 = Group::new("1-2-3");
        let removed = group1.remove(SearchCriteria::Index(0));
        println!("{:?}", removed);
    }

    #[test]
    fn remove() {
        let mut group1 = Group::new("1-2-3");

        let s1 = Student::new("a", "a", "123456").rating(91);
        let s2 = Student::new("b", "b", "12345").rating(100);

        group1.add(s1).add(s2);

        group1.remove(SearchCriteria::Name("a".into()));
    }
}
