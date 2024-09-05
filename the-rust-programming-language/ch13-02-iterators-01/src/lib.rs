// Iterator trait 和 next 方法
#[cfg(test)]
mod tests {
    // 在迭代器上（直接）调用 next 方法
    #[test]
    fn iterator_daemonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // 消费迭代器的方法
    // 调用 sum 方法获取迭代器所有项的总和
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}
