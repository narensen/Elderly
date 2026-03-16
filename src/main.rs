#[derive(Debug)]
enum Pod {
    Regular(String),
    Elderly(String),
    Emergency(String),
}

impl Pod {
    fn pod_type(&self) -> String {
        match self {
            Pod::Regular(name) => format!("{} is a regular pod.", name),
            Pod::Elderly(name) => format!("{} is an elderly pod.", name),
            Pod::Emergency(name) => format!("{} is an emergency pod.", name),
        }
    }
}

struct Descent {
    velocity_down: f32,
    velocity_up: f32,
    g_felt: f32,
}

impl Descent {
    fn new(pod_type: &Pod, base: &Descent) -> Self {
        match pod_type {
            Pod::Regular(_) => Descent {
                velocity_down: base.velocity_down,
                velocity_up: base.velocity_up,
                g_felt: base.g_felt,
            },
            Pod::Elderly(_) => Descent {
                velocity_down: base.velocity_down * 0.7,
                velocity_up: base.velocity_up * 0.7,
                g_felt: base.g_felt * 0.6,
            },
            Pod::Emergency(_) => Descent {
                velocity_down: base.velocity_down * 0.5,   // faster descent
                velocity_up: base.velocity_up * 0.5,
                g_felt: base.g_felt * 3.0,                 // much higher g
            },
        }
    }

    fn calculate_descent(&self) -> String {
        if self.g_felt > 2.5 {
            format!("Descent is DANGEROUS — peak g: {:.1}g, velocity down: {:.1} m/s",
                    self.g_felt, self.velocity_down)
        } else if self.g_felt > 1.5 {
            format!("Descent is too fast — peak g: {:.1}g, velocity down: {:.1} m/s",
                    self.g_felt, self.velocity_down)
        } else {
            format!("Descent is SAFE — peak g: {:.1}g, velocity down: {:.1} m/s",
                    self.g_felt, self.velocity_down)
        }
    }
}

fn main() {
    let regular_pod = Pod::Regular(String::from("Pod A"));
    let elderly_pod = Pod::Elderly(String::from("Pod B"));
    let emergency_pod = Pod::Emergency(String::from("Pod C")); // added for testing

    let base = Descent {
        velocity_down: 10.0,
        velocity_up: 5.0,
        g_felt: 2.0,
    };

    let descent_regular = Descent::new(&regular_pod, &base);
    let descent_elderly = Descent::new(&elderly_pod, &base);
    let descent_emergency = Descent::new(&emergency_pod, &base);

    println!("{}", regular_pod.pod_type());
    println!("{}", elderly_pod.pod_type());
    println!("{}", emergency_pod.pod_type());

    println!("{}", descent_regular.calculate_descent());
    println!("{}", descent_elderly.calculate_descent());
    println!("{}", descent_emergency.calculate_descent());
}