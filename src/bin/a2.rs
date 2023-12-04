#![allow(unused)]
use agency::*;
use generic_container::*;

fn main() {
    let vacancies = vec![
        Vacancy::new(
            "Company2",
            "IT",
            "Conditions1",
            "149200",
            Some("IT".to_owned()),
            "12",
            "",
        )
        .expect("Bad vacancy input "),
        Vacancy::new(
            "Company1",
            "Waiter",
            "Conditions2",
            "69100",
            None,
            "1",
            "school",
        )
        .expect("Bad vacancy input "),
    ];
    let vacancy3 = Vacancy::new(
        "Company3",
        "Teacher",
        "Conditions3",
        "19100",
        None,
        "24",
        "university",
    )
    .expect("Bad vacancy input");

    let mut generic = GenericContainer::new(vacancies);
    generic.push_back(vacancy3);
    println!("\n-------\nINITIAL:");
    generic
        .clone()
        .into_iter()
        .for_each(|val| println!("{val}"));

    println!("\n-------\nSORT BY COMPANY NAME:");
    let sorted_by_company_name = utils::sort_by_company_name(&generic);
    sorted_by_company_name
        .clone()
        .into_iter()
        .for_each(|val| println!("{val}"));

    println!("\n-------\nSORT BY EDUCATION REQUIRED:");
    let sort_by_edu = utils::sort_by_education(&generic);
    sort_by_edu
        .clone()
        .into_iter()
        .for_each(|val| println!("{val}"));

    println!("\n-------\nSORT BY SPECIALIZATION:");
    let sort_by_spec = utils::sort_by_specialization(&generic);
    sort_by_spec
        .clone()
        .into_iter()
        .for_each(|val| println!("{val}"));

    process_input(&mut generic);

    // generic.for_each(f)
}

fn process_input(mut data: &mut GenericContainer<Vacancy>) {
    let input = std::io::stdin();

    'mainloop: loop {
        println!(
            "Input option:\n1) List all vacancies\n2) Add vacancy\n3) Remove vacancy by id\n4) Exit"
        );
        let mut read = String::new();
        input.read_line(&mut read).expect("Error reading line");

        match read.trim() {
            "1" => list_vacancies(&data),
            "2" => add_vacancy(&mut data),

            "3" => {
                println!("Enter index of company:");
                let mut read = String::new();
                input.read_line(&mut read).expect("Error reading line");

                let Ok(index) = read.trim().parse::<usize>() else {
                    println!("Error parsing index");
                    continue 'mainloop;
                };
                if index > data.len() {
                    println!("Amount of vacancies is less then input index");
                    continue 'mainloop;
                } else if index == 0 {
                    println!("0 is bad index");
                    continue 'mainloop;
                }
                if let Err(_) = data.delete_by_number(index - 1) {
                    println!("Error removing data");
                } else {
                    println!("Data removed");
                }
            }

            "4" => break,
            _ => continue,
        }
    }
}

fn list_vacancies(data: &GenericContainer<Vacancy>) {
    data.clone()
        .enumerate()
        .for_each(|(num, v)| println!("({}) {v}", num + 1))
}

fn add_vacancy(data: &mut GenericContainer<Vacancy>) {
    let input = std::io::stdin();
    println!("Company name: ");
    let mut company_name = String::new();
    input
        .read_line(&mut company_name)
        .expect("Error reading line");

    println!("Company specialization: ");
    let mut specialization = String::new();
    input
        .read_line(&mut specialization)
        .expect("Error reading line");
    println!("Company conditions: ");
    let mut conditions = String::new();
    input
        .read_line(&mut conditions)
        .expect("Error reading line");
    println!("Salary: ");
    let mut sallary_string = String::new();
    input
        .read_line(&mut sallary_string)
        .expect("Error reading line");
    println!("Worked specialization name (or empty): ");
    let mut worker_specialization_string = String::new();
    input
        .read_line(&mut worker_specialization_string)
        .expect("Error reading line");

    let work_exp_years_string = if !worker_specialization_string.trim().eq("") {
        println!("Worked experience years: ");
        let mut work_exp_years_string = String::new();
        input
            .read_line(&mut work_exp_years_string)
            .expect("Error reading line");
        work_exp_years_string
    } else {
        "0".to_owned()
    };
    let worker_specialization_name = if worker_specialization_string.eq("") {
        None
    } else {
        Some(worker_specialization_string.trim().to_owned())
    };

    println!("Worked Education (empty, school,university): ");

    let mut education = String::new();
    input.read_line(&mut education).expect("Error reading line");

    let education_string = education.trim();
    //     if let Some(mat) = regex_match_education.find(&education.trim()) {
    //         break mat.as_str();
    //     } else {
    //         println!("Error matching string, enter only school,university or empty string");
    //     }
    // };

    let vacancy = Vacancy::new(
        company_name.trim(),
        specialization.trim(),
        conditions.trim(),
        sallary_string.trim(),
        worker_specialization_name,
        work_exp_years_string.trim(),
        education_string.trim(),
    );
    let vacancy = match vacancy {
        Ok(vacancy) => vacancy,
        Err(e) => {
            println!("Error creating new vacancy: {e:?}");
            return;
        }
    };
    data.push_back(vacancy);
    println!("Vacancy added");
}

mod agency {
    use std::fmt::Display;

    #[derive(Debug, Clone)]
    pub struct Vacancy {
        company_name: String,
        specialization: String,
        conditions: String,
        sallary: usize,
        worker_requirements: WorkerRequirements,
    }

    impl Display for Vacancy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}: {}({}) - {}$ {}",
                self.company_name,
                self.specialization,
                self.conditions,
                self.sallary,
                self.worker_requirements.education
            )
        }
    }

    pub trait VacancyTrait {
        fn company_name(&self) -> &str;
        fn specialization(&self) -> &str;
        fn education(&self) -> &Education;
    }
    impl VacancyTrait for Vacancy {
        fn company_name(&self) -> &str {
            &self.company_name
        }

        fn specialization(&self) -> &str {
            &self.specialization
        }

        fn education(&self) -> &Education {
            &self.worker_requirements.education
        }
    }

    #[derive(Debug)]
    pub enum VacancyCreationError {
        ParsingEducationString(String),
        ParsingSallary(String),
        ParsingExperience(String),
    }
    impl Vacancy {
        pub fn new(
            company_name: &str,
            specialization: &str,
            conditions: &str,
            sallary_string: &str,
            worker_specialization_name: Option<String>,
            work_exp_years_string: &str,
            education_string: &str,
        ) -> Result<Self, VacancyCreationError> {
            let work_exp_years: u16 = work_exp_years_string.parse().map_err(|_| {
                VacancyCreationError::ParsingExperience(work_exp_years_string.to_owned())
            })?;

            let worker_specialization =
                if let Some(specialization_name) = worker_specialization_name {
                    Some(WorkerSpecialization {
                        specialization_name,
                        work_exp_years,
                    })
                } else {
                    None
                };
            let sallary: usize = sallary_string
                .parse()
                .map_err(|_| VacancyCreationError::ParsingSallary(sallary_string.to_owned()))?;
            let education = match &*education_string.to_lowercase() {
                "" => Education::None,
                "school" => Education::School,
                "university" => Education::University,
                _ => {
                    return Err(VacancyCreationError::ParsingEducationString(
                        education_string.to_owned(),
                    ))
                }
            };
            Ok(Self {
                company_name: company_name.to_owned(),
                specialization: specialization.to_owned(),
                conditions: conditions.to_owned(),
                sallary,
                worker_requirements: WorkerRequirements {
                    specialization: worker_specialization,
                    education,
                },
            })
        }
    }

    #[derive(Debug, Clone)]
    struct WorkerRequirements {
        specialization: Option<WorkerSpecialization>,
        education: Education,
    }
    #[derive(Debug, Clone)]
    struct WorkerSpecialization {
        specialization_name: String,
        work_exp_years: u16,
    }
    #[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
    pub enum Education {
        None,
        School,
        University,
    }

    impl Display for Education {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Education::*;
            let edu_str = match self {
                None => "",
                School => "school",
                University => "university",
            };
            write!(f, "{edu_str}")
        }
    }
}

mod utils {
    use std::fmt::Display;

    use crate::{agency::VacancyTrait, generic_container::GenericContainer};

    pub fn sort_by_company_name<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Display + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.company_name().cmp(&b.company_name()));

        GenericContainer::new(data)
    }
    pub fn sort_by_specialization<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Display + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.specialization().cmp(&b.specialization()));

        GenericContainer::new(data)
    }
    pub fn sort_by_education<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Display + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.education().cmp(&b.education()));

        GenericContainer::new(data)
    }
}

mod generic_container {
    use std::{collections::LinkedList, fmt::Display};

    #[derive(Debug, Clone)]
    pub struct GenericContainer<T> {
        container: LinkedList<T>,
    }
    impl<T: Clone> GenericContainer<T> {
        pub fn new(input: Vec<T>) -> Self {
            Self {
                container: input.into_iter().collect(),
            }
        }
        pub fn len(&self) -> usize {
            self.container.len()
        }
        pub fn push_back(&mut self, input: T) {
            self.container.push_back(input);
        }
        pub fn delete_by_number(&mut self, id: usize) -> Result<T, ()> {
            if self.container.len() < id || id == 0 {
                return Err(());
            }
            let mut container_vec = self.container.clone().into_iter().collect::<Vec<_>>();
            let removed_value = container_vec.remove(id - 1);
            self.container = container_vec.into_iter().collect();
            Ok(removed_value)
        }
    }
    impl<T: Clone> Iterator for GenericContainer<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.container.pop_front()
        }
    }
}
