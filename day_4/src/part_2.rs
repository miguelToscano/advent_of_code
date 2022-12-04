// .234.....  2-4
// .....678.  6-8

// .23......  2-3
// ...45....  4-5

// ....567..  5-7
// ......789  7-9

// .2345678.  2-8
// ..34567..  3-7

// .....6...  6-6
// ...456...  4-6

// .23456...  2-6
// ...45678.  4-8

#[derive(Debug, Clone)]
pub struct Assignment {
    pub section_a: Vec<usize>,
    pub section_b: Vec<usize>,
}

impl Assignment {
    fn new(input: &String) -> Self {
        let assignments = input.split(",").collect::<Vec<&str>>();

        let parsed_section_a = assignments[0]
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.to_owned().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let section_a = (parsed_section_a[0]..parsed_section_a[1] + 1).collect::<Vec<usize>>();

        let parsed_section_b = assignments[1]
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.to_owned().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let section_b = (parsed_section_b[0]..parsed_section_b[1] + 1).collect::<Vec<usize>>();

        return Assignment {
            section_a,
            section_b,
        };
    }

    pub fn has_section_fully_contained_in_another(&self) -> bool {
        let mut merged_sections = [self.section_a.clone(), self.section_b.clone()].concat();

        merged_sections.sort();
        merged_sections.dedup();

        if merged_sections.len() == self.section_a.len()
            || merged_sections.len() == self.section_b.len()
        {
            return true;
        }

        return false;
    }

    pub fn has_overlapping_sections(&self) -> bool {
        let mut merged_sections = [self.section_a.clone(), self.section_b.clone()].concat();

        let merged_sections_length = merged_sections.len();

        merged_sections.sort();
        merged_sections.dedup();

        if merged_sections.len() != merged_sections_length {
            return true;
        }

        return false;
    }
}

pub fn get_answer(inputs: &Vec<String>) {
    let mut assignments: Vec<Assignment> = vec![];

    for input in inputs {
        assignments.push(Assignment::new(&input));
    }

    println!(
        "Answer is: {:?}",
        assignments
            .iter()
            .filter(|&assignment| assignment.has_overlapping_sections())
            .collect::<Vec<&Assignment>>()
            .len()
    );
}
