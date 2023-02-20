mod list;


#[cfg(test)]
mod tests {
    use super::list::single::List;

    #[test]
    fn should_find_element() {
        let mut list = List::new();
        list.insert(3);
        list.insert(7);
        list.insert(0);

        assert_eq!(7, list.search(7).unwrap().elem);
    }

    #[test]
    fn should_return_empty_if_element_does_not_exist() {
        let mut list = List::new();
        list.insert(3);
        list.insert(0);

        assert_eq!(true, list.search(7).is_none());
    }
}
