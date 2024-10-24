use std::{thread::sleep, time::{Duration, Instant, SystemTime}};

fn main() {
    // std::time
    time_example();
    // std::time::Duration
    duration_example();
    // SystemTime
    system_time_example();
}

fn time_example() {
    {
        // 有多种方法可以创建 Duration:
        let five_seconds = Duration::from_secs(5);
        assert_eq!(five_seconds, Duration::from_millis(5_000));
        assert_eq!(five_seconds, Duration::from_micros(5_000_000));
        assert_eq!(five_seconds, Duration::from_nanos(5_000_000_000));

        let ten_seconds = Duration::from_secs(10);
        let seven_nanos = Duration::from_nanos(7);
        let total = ten_seconds + seven_nanos;
        assert_eq!(total, Duration::new(10, 7));
    }

    // Instant 对单调递减时钟的测量。
    {
        fn slow_function() {
            sleep(Duration::from_secs(1));
        }
        // 使用 Instant 计算函数运行所需的时间
        let now = Instant::now();
        slow_function();
        let elapsed_time = now.elapsed();
        println!("Running slow_function() took {} seconds.", elapsed_time.as_secs());
    }
}

fn duration_example() {
    let five_seconds = Duration::new(5, 0);
    let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);

    assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
    assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);

    let ten_millis = Duration::from_millis(10);
    println!("{:?}", ten_millis);
}


fn system_time_example() {
    {
        let now = SystemTime::now();

        // we sleep for 2 seconds
        sleep(Duration::new(2, 0));
        match now.elapsed() {
            Ok(elapsed) => {
                // it print '2'
                println!("{}", elapsed.as_secs());
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
            }
        }
    }

    // UNIX_EPOCH
    {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    // now
    {
        let sys_time = SystemTime::now();
        println!("{sys_time:?}");
    }

    // duration_since
    {
        let sys_time = SystemTime::now();
        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(sys_time)
            .expect("Clock may have gone backwards");
        println!("{difference:?}");
    }

    // elapsed
    {
        let sys_time = SystemTime::now();
        let one_sec = Duration::from_secs(1);
        sleep(one_sec);
        assert!(sys_time.elapsed().unwrap() >= one_sec);
    }


}

