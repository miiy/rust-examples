// 定义通用行为的 trait

// Draw trait 的定义
pub trait Draw {
    fn draw(&self);
}

// 一个 Screen 结构体的定义，它带有一个字段 components，其包含实现了 Draw trait 的 trait 对象 vectory
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 在 Screen 上实现⼀个 run ⽅法，该⽅法在每个 component 上调⽤ draw ⽅法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}