// AveragedCollection 结构体维护了一个整型列表及其所有元素的平均值
// pub struct AveragedCollection {
//     list: Vec<i32>,
//     average: f64,
// }

// 在 AveragedCollection 结构体上实现了 add、remove 和 average 公有犯法
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        return self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut a = AveragedCollection{
            list: vec![],
            average: 0.0,
        };
        a.add(1);
        a.add(2);
        a.add(3);
        a.remove();
        assert_eq!(1.5, a.average())
    }
}