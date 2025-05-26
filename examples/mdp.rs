use rand::Rng;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
enum State {
    S1, // 上个月买过
    S2, // 上个月没买
}

#[derive(Clone, Copy)]
enum Action {
    A1, // 什么都不做
    A2, // 小促销
    A3, // 大促销
}

fn get_cost(action: Action) -> f32 {
    match action {
        Action::A1 => 0.0,
        Action::A2 => 0.5,
        Action::A3 => 0.5,
    }
}

fn transition_prob(state: State, action: Action) -> (f32, f32) {
    match (state, action) {
        (State::S1, Action::A1) => (0.3, 0.7),
        (State::S1, Action::A2) => (0.5, 0.5),
        (State::S1, Action::A3) => (0.7, 0.3),
        (State::S2, Action::A1) => (0.1, 0.9),
        (State::S2, Action::A2) => (0.3, 0.7),
        (State::S2, Action::A3) => (0.5, 0.5),
    }
}

fn next_state(current: State, action: Action) -> State {
    let (p_s1, _) = transition_prob(current, action);
    let mut rng = rand::thread_rng();
    if rng.random_range(0.0..1.0) < p_s1 { // 修正为 gen_range
        State::S1
    } else {
        State::S2
    }
}

fn simulate_mdp(steps: usize) -> f32 {
    let mut current_state = State::S1; // 初始状态
    let actions = [Action::A1, Action::A2, Action::A3];
    let mut total_cost = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..steps {
        // 随机选择一个决策
        let action = actions[rng.random_range(0..actions.len())];
        total_cost += get_cost(action);
        current_state = next_state(current_state, action);
    }

    total_cost / steps as f32 // 返回平均成本
}

fn main() {
    let steps = 10000000; // 模拟步数
    let start_time = Instant::now(); // 记录开始时间
    let avg_cost = simulate_mdp(steps);
    let duration = start_time.elapsed(); // 计算耗时

    println!("模拟 {} 步后的平均成本: {:.3}", steps, avg_cost);
    println!("模拟耗时: {:?}", duration); // 输出耗时
}
