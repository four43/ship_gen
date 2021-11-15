pub mod rocket {
    use std::cmp::max;
    use std::fmt;
    use rand;
    use rand::distributions::WeightedIndex;
    use rand::prelude::*;

    #[derive(PartialEq, Debug)]
    pub enum PartType {
        TIP,
        BODY,
        TRANSITION(usize),
        // From width
        ENGINE(usize),
        // From width
        EXHAUST(usize),
    }

    #[derive(Debug)]
    pub struct Part {
        height: usize,
        width: usize,
        shape: &'static str,
        type_: PartType,
        selection_weight: usize,
    }

    impl fmt::Display for Part {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.shape)
        }
    }

    pub const PARTS_BIN: [Part; 21] = [
        // Tips
        Part { width: 0, height: 1, shape: "│", type_: PartType::TIP, selection_weight: 1 },
        Part { width: 0, height: 2, shape: "│\n║", type_: PartType::TIP, selection_weight: 1 },

        // Transitions
        Part { width: 1, height: 1, shape: "/'\\", type_: PartType::TRANSITION(0), selection_weight: 2 },
        Part { width: 1, height: 1, shape: "┌┴┐", type_: PartType::TRANSITION(0), selection_weight: 2 },
        Part { width: 1, height: 1, shape: "┌╩┐", type_: PartType::TRANSITION(0), selection_weight: 1 },
        Part { width: 3, height: 1, shape: "/   \\", type_: PartType::TRANSITION(1), selection_weight: 2 },
        Part { width: 3, height: 2, shape: "/'\\\n/   \\", type_: PartType::TRANSITION(0), selection_weight: 1 },
        Part { width: 3, height: 1, shape: "┌┘ └┐", type_: PartType::TRANSITION(1), selection_weight: 1 },

        // Body
        Part { width: 1, height: 1, shape: "│ │", type_: PartType::BODY, selection_weight: 10 },
        Part { width: 1, height: 1, shape: "│°│", type_: PartType::BODY, selection_weight: 5 },
        Part { width: 1, height: 1, shape: "/│ │\\", type_: PartType::BODY, selection_weight: 1 },
        Part { width: 3, height: 1, shape: "│   │", type_: PartType::BODY, selection_weight: 10 },
        Part { width: 3, height: 1, shape: "│° °│", type_: PartType::BODY, selection_weight: 5 },
        Part { width: 3, height: 1, shape: "│ O │", type_: PartType::BODY, selection_weight: 5 },
        Part { width: 3, height: 2, shape: "/│ ^ │\\\n/_│ | │_\\", type_: PartType::BODY, selection_weight: 1 },

        // Engines
        Part { width: 0, height: 1, shape: "'─'", type_: PartType::ENGINE(1), selection_weight: 1 },
        Part { width: 1, height: 1, shape: "\\_/", type_: PartType::ENGINE(3), selection_weight: 1 },
        Part { width: 0, height: 1, shape: "( )", type_: PartType::EXHAUST(1), selection_weight: 1 },
        Part { width: 0, height: 1, shape: "·", type_: PartType::EXHAUST(0), selection_weight: 1 },
        Part { width: 0, height: 1, shape: ".", type_: PartType::EXHAUST(0), selection_weight: 1 },
        Part { width: 0, height: 1, shape: "'", type_: PartType::EXHAUST(0), selection_weight: 1 },
    ];

    pub struct Rocket {
        pub max_height: usize,
        pub max_width: usize,

        sections: Vec<&'static Part>,
        height: usize,
        bottom_width: usize,
    }

    impl Default for Rocket {
        fn default() -> Self {
            Rocket { max_height: 3, max_width: 3, sections: Vec::new(), height: 0, bottom_width: 0 }
        }
    }

    impl Rocket {
        pub fn new(max_height: usize) -> Rocket {
            let mut rocket = Rocket {
                max_height,
                ..Rocket::default()
            };
            rocket.build();
            return rocket;
        }

        fn append_section(&mut self, part: &'static Part) {
            if part.height + self.height > self.max_height {
                panic!("Cannot add part because it would make the rocket too tall")
            }
            self.sections.push(part);
            self.height += part.height;
            self.bottom_width = part.width;
        }

        fn prepend_section(&mut self, part: &'static Part) {
            if part.height + self.height > self.max_height {
                panic!("Cannot add part because it would make the rocket too tall")
            }
            self.sections.insert(0, part);
            self.height += part.height;
        }

        fn part_height_remaining(&self) -> usize {
            self.max_height - self.height
        }

        fn build(&mut self) {
            if self.max_height < 3 {
                panic!("Cannot build a rocket shorter than 3 sections")
            }
            let nose_cone = Rocket::choose_part(&PARTS_BIN, |x| { x.type_ == PartType::TRANSITION(0) });
            self.append_section(nose_cone);

            let mut rng = rand::thread_rng();
            let body_decor_ratio = rng.gen_range(0.2..0.4);

            // Add body or transition
            while (self.part_height_remaining() as f32 / self.height as f32) > body_decor_ratio && self.part_height_remaining() > 2 {
                let next_part = Rocket::choose_part(&PARTS_BIN, |x| {
                    (x.type_ == PartType::TRANSITION(self.bottom_width)
                        || (x.type_ == PartType::BODY && x.width == self.bottom_width))
                        && x.height <= (self.part_height_remaining() - 2)
                });
                self.append_section(next_part);
            }
            // Finish up and add engine
            let engine_part = Rocket::choose_part(&PARTS_BIN, |x| {
                x.type_ == PartType::ENGINE(self.bottom_width)
                    && x.height <= self.part_height_remaining()
            });
            self.append_section(engine_part);

            // Add decoration (exhaust or nose)
            while self.part_height_remaining() > 0 {
                let decoration_part = Rocket::choose_part(&PARTS_BIN, |x| {
                    (x.type_ == PartType::TIP
                        || (x.type_ == PartType::EXHAUST(self.bottom_width)))
                        && x.height <= self.part_height_remaining()
                });
                if decoration_part.type_ == PartType::TIP {
                    self.prepend_section(decoration_part);
                } else {
                    self.append_section(decoration_part);
                }
            }
        }

        fn choose_part<P>(parts_list: &[Part], predicate: P) -> &Part
            where
                P: FnMut(&&Part) -> bool
        {
            let mut rng = rand::thread_rng();
            let possible_parts = parts_list.iter().filter(predicate).collect::<Vec<&Part>>();

            let dist = WeightedIndex::new(possible_parts.iter()
                .map(|x| x.selection_weight)).unwrap();

            possible_parts[dist.sample(&mut rng)]
        }
    }

    impl fmt::Display for Rocket {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut output = String::new();
            let rocket_width = self.sections.iter()
                .fold(0, |a, x| {
                    let mut max_width = a;
                    for line in x.shape.lines() {
                        max_width = max(max_width, line.chars().count());
                    }
                    return max_width;
                });
            for section in &self.sections {
                for line in section.shape.lines() {
                    let spacing: usize = ((rocket_width - line.chars().count()) as f32 / 2.0).ceil() as usize;
                    output.push_str(&" ".repeat(spacing));
                    output.push_str(&line);
                    output.push_str("\n");
                }
            }
            write!(f, "{}", output)
        }
    }
}
