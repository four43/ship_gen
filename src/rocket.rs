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
        ENGINE,
        EXHAUST,
    }

    #[derive(Debug)]
    pub struct Part {
        height: usize,
        top_width: usize,
        bottom_width: usize,
        shape: &'static str,
        type_: PartType,
        selection_weight: usize,
    }

    impl fmt::Display for Part {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.shape)
        }
    }

    pub const PARTS_BIN: [Part; 23] = [
        // Tips
        Part { top_width: 0, bottom_width: 0, height: 1, shape: "│", type_: PartType::TIP, selection_weight: 1 },
        Part { top_width: 0, bottom_width: 0, height: 2, shape: "│\n║", type_: PartType::TIP, selection_weight: 1 },

        // Transitions
        Part { top_width: 0, bottom_width: 1, height: 1, shape: "/'\\", type_: PartType::BODY, selection_weight: 2 },
        Part { top_width: 0, bottom_width: 1, height: 1, shape: "┌┴┐", type_: PartType::BODY, selection_weight: 2 },
        Part { top_width: 0, bottom_width: 1, height: 1, shape: "┌╩┐", type_: PartType::BODY, selection_weight: 1 },
        Part { top_width: 1, bottom_width: 3, height: 1, shape: "/   \\", type_: PartType::BODY, selection_weight: 2 },
        Part { top_width: 0, bottom_width: 3, height: 2, shape: "/'\\\n/   \\", type_: PartType::BODY, selection_weight: 1 },
        Part { top_width: 1, bottom_width: 3, height: 1, shape: "┌┘ └┐", type_: PartType::BODY, selection_weight: 1 },
        Part { top_width: 3, bottom_width: 1, height: 1, shape: "\\   /", type_: PartType::BODY, selection_weight: 1 },
        Part { top_width: 3, bottom_width: 1, height: 1, shape: "└┐ ┌┘", type_: PartType::BODY, selection_weight: 1 },

        // Body
        Part { top_width: 1, bottom_width: 1, height: 1, shape: "│ │", type_: PartType::BODY, selection_weight: 10 },
        Part { top_width: 1, bottom_width: 1, height: 1, shape: "│°│", type_: PartType::BODY, selection_weight: 5 },
        Part { top_width: 1, bottom_width: 1, height: 1, shape: "/│ │\\", type_: PartType::BODY, selection_weight: 1 },
        Part { top_width: 3, bottom_width: 3, height: 1, shape: "│   │", type_: PartType::BODY, selection_weight: 10 },
        Part { top_width: 3, bottom_width: 3, height: 1, shape: "│° °│", type_: PartType::BODY, selection_weight: 5 },
        Part { top_width: 3, bottom_width: 3, height: 1, shape: "│ O │", type_: PartType::BODY, selection_weight: 5 },
        Part { top_width: 3, bottom_width: 3, height: 2, shape: "/│ ^ │\\\n/_│ | │_\\", type_: PartType::BODY, selection_weight: 1 },

        // Engines
        Part { top_width: 1, bottom_width: 0, height: 1, shape: "'─'", type_: PartType::ENGINE, selection_weight: 1 },
        Part { top_width: 3, bottom_width: 1, height: 1, shape: "\\_/", type_: PartType::ENGINE, selection_weight: 1 },
        Part { top_width: 1, bottom_width: 0, height: 1, shape: "( )", type_: PartType::EXHAUST, selection_weight: 1 },
        Part { top_width: 0, bottom_width: 0, height: 1, shape: "·", type_: PartType::EXHAUST, selection_weight: 1 },
        Part { top_width: 0, bottom_width: 0, height: 1, shape: ".", type_: PartType::EXHAUST, selection_weight: 1 },
        Part { top_width: 0, bottom_width: 0, height: 1, shape: "'", type_: PartType::EXHAUST, selection_weight: 1 },
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
            self.bottom_width = part.bottom_width;
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
            let nose_cone = self.choose_next_part(&PARTS_BIN, &[PartType::BODY]);
            self.append_section(nose_cone);

            let mut rng = rand::thread_rng();
            let body_decor_ratio = rng.gen_range(0.2..0.4);

            // Add body or transition
            while (self.part_height_remaining() as f32 / self.height as f32) > body_decor_ratio && self.part_height_remaining() > 3 {
                let next_part = self.choose_next_part_buffer(&PARTS_BIN, &[PartType::BODY], 2);
                self.append_section(next_part);
            }
            // Finish up and add engine
            let engine_part = self.choose_next_part(&PARTS_BIN, &[PartType::ENGINE]);
            self.append_section(engine_part);

            // Add decoration (exhaust or nose)
            while self.part_height_remaining() > 0 {
                let decoration_part = self.choose_next_part(&PARTS_BIN, &[PartType::TIP, PartType::EXHAUST]);
                if decoration_part.type_ == PartType::TIP {
                    self.prepend_section(decoration_part);
                } else {
                    self.append_section(decoration_part);
                }
            }
        }

        fn choose_next_part_buffer(&self, parts_list: &'static[Part], part_types: &'static[PartType], height_buffer: usize) -> &'static Part {
            let mut rng = rand::thread_rng();
            let possible_parts = parts_list.iter().filter(|p| {
                part_types.contains(&p.type_)
                    && p.top_width == self.bottom_width
                    && p.height <= (self.part_height_remaining() - height_buffer)
            }).collect::<Vec<&'static Part>>();
            let dist = WeightedIndex::new(possible_parts.iter()
                .map(|x| x.selection_weight)).unwrap();

            possible_parts[dist.sample(&mut rng)]
        }

        fn choose_next_part(&self, parts_list: &'static[Part], part_types: &'static[PartType])-> &'static Part {
            self.choose_next_part_buffer(parts_list, part_types, 0)
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
