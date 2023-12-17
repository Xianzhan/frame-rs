#[cfg(test)]
mod tests {

    #[test]
    fn test_sort() {
        // 整数排序
        let mut v = vec![1, 5, 10, 2, 15];
        v.sort();
        assert_eq!(v, vec![1, 2, 5, 10, 15]);

        // 浮点数排序
        let mut v = vec![1.1, 1.15, 5.5, 1.123, 2.0];
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(v, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Person {
            pub fn new(name: String, age: u8) -> Self {
                Person { name, age }
            }
        }

        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
        ];

        // 根据获得的自然顺序（name 和 age）对 people 进行排序
        people.sort();

        assert_eq!(
            people,
            vec![
                Person::new("Al".to_string(), 60),
                Person::new("John".to_string(), 1),
                Person::new("Zoe".to_string(), 25),
            ]
        );

        // 根据 age 值对 people 进行排序
        people.sort_by(|a, b| b.age.cmp(&a.age));

        assert_eq!(
            people,
            vec![
                Person::new("Al".to_string(), 60),
                Person::new("Zoe".to_string(), 25),
                Person::new("John".to_string(), 1),
            ]
        );
    }
}
