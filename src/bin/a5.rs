#![allow(unused)]
use std::fs::OpenOptions;
use std::io::*;

use agency::*;
use generic_container::*;
use regex::Regex;

//cargo run --bin a5 auto
//cargo run --bin a5

const FILE_NAME: &str = "vacancies3.json";

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "auto" {
        println!("Auto mod enabled\n\n");
        let default = default_vacancies();
        let generic = GenericContainer::new(default);
        let serialized_string = generic.serialize().expect("Error while serializing");
        println!("Serialized data: {}\n", serialized_string);

        let deserialized: GenericContainer<Vacancy> =
            GenericContainer::deserialize(serialized_string.as_bytes())
                .expect("Error while deserializing");
        println!("Deserialized data: {:?}\n", deserialized);

        let regex_teacher = Regex::new("(?i)(teacher|educator)").expect("Error parsing regex");
        let regex_english = Regex::new("(?i)(english)").expect("Error parsing regex");

        // let regex_teacher = Regex::new("(?i)(teacher|educator)").expect("Error parsing regex");

        let filtered = deserialized
            .filter(|vac| {
                regex_teacher.is_match(&vac.worker_specialization())
                    && vac.worker_owns_car()
                    && regex_english.is_match(&vac.worker_languages())
                    && vac.worker_exp_years() > 10
            })
            .collect::<Vec<_>>();
        println!(
            "------\nVacancies for teachers with >10yrs exp which know English and have a car: \n",
        );
        filtered.iter().for_each(|v| println!("{v}"))
    } else {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // .truncate(true)
            .open(FILE_NAME)
            .expect("Error opening .json file");
        let metadata = file.metadata().expect("Error geting metadata");
        if metadata.len() == 0 {
            // println!("{:?}",metadata.len());
            println!("Serializing data and writing to file");
            // write to file
            let vacancies = default_vacancies();
            let data = serde_json::to_vec(&vacancies).expect("Error serializing");
            file.write(&data).expect("Error writing to file");
            file.rewind().expect("Error rewinding");
        }
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .expect("Error reading file to buffer");

        let mut data: GenericContainer<Vacancy> =
            GenericContainer::deserialize(&buf).expect("Error while deserializing");

        // println!("Deserialized data from file:\n\n");
        // println!("{data:?}");
        process_input(&mut data);
    }
}

fn process_input(mut data: &mut GenericContainer<Vacancy>) {
    let input = std::io::stdin();
    let regex_match_menu = Regex::new("^[1-4]$").expect("Error in regex");
    let regex_match_number = Regex::new("^[1-90]*$").expect("Error in regex");

    'mainloop: loop {
        println!("Input option:\n1) List all vacancies\n2) Add vacancy\n3) Remove vacancy by id \n4)Exit and save");
        let mut read = String::new();
        input.read_line(&mut read).expect("Error reading line");

        let Some(matched_input) = regex_match_menu.find(read.trim()) else {
            continue 'mainloop;
        };

        match matched_input.as_str() {
            "1" => list_vacancies(&data),
            "2" => add_vacancy(&mut data),

            "3" => {
                println!("Enter index of company:");
                let mut read = String::new();
                input.read_line(&mut read).expect("Error reading line");

                let Some(matched_input) = regex_match_number.find(read.trim()) else {
                    println!("Not number entered");
                    continue 'mainloop;
                };

                let Ok(index) = matched_input.as_str().parse::<usize>() else {
                    unreachable!()
                    // println!("Error parsing index");
                    // continue 'mainloop;
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

            "4" => {
                println!("Saving all updates to file");
                std::fs::remove_file(FILE_NAME).expect("Error removing file");

                let data = data.serialize().expect("Error serializing");

                std::fs::write(FILE_NAME, &data.as_bytes()).expect("Error writing data to file");

                break;
            }
            _ => {
                unreachable!()
            }
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

    let regex_match_education = Regex::new("^(?i)(|school|university)$").expect("Error in regex");

    println!("Worked Education (empty, school,university): ");

    let mut education = String::new();
    let education_string = loop {
        input.read_line(&mut education).expect("Error reading line");
        if let Some(mat) = regex_match_education.find(&education.trim()) {
            break mat.as_str();
        } else {
            println!("Error matching string, enter only school,university or empty string");
        }
    };

    let vacancy = Vacancy::new(
        company_name.trim(),
        specialization.trim(),
        conditions.trim(),
        sallary_string.trim(),
        worker_specialization_name,
        work_exp_years_string.trim(),
        education_string.trim(),
        None,
        false,
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
    use serde::Deserialize;
    use serde::Serialize;
    use std::fmt::Display;

    // #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        fn worker_owns_car(&self) -> bool;
        fn worker_languages(&self) -> &str;
        fn worker_specialization(&self) -> &str;
        fn worker_exp_years(&self) -> u16;
    }
    impl VacancyTrait for Vacancy {
        fn company_name(&self) -> &str {
            &self.company_name
        }

        fn specialization(&self) -> &str {
            &self.specialization
        }

        fn worker_specialization(&self) -> &str {
            self.worker_requirements
                .specialization
                .as_ref()
                .map(|x| x.specialization_name.as_str())
                .unwrap_or_default()
        }
        fn worker_exp_years(&self) -> u16 {
            self.worker_requirements
                .specialization
                .as_ref()
                .map(|x| x.work_exp_years)
                .unwrap_or_default()
        }
        fn worker_owns_car(&self) -> bool {
            self.worker_requirements.car
        }

        fn worker_languages(&self) -> &str {
            self.worker_requirements
                .foreign_languages
                .as_ref()
                .map(|x| x.as_str())
                .unwrap_or_default()
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
            foreign_languages: Option<String>,
            car: bool,
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
                    foreign_languages,
                    car,
                },
            })
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct WorkerRequirements {
        specialization: Option<WorkerSpecialization>,
        education: Education,
        foreign_languages: Option<String>,
        car: bool,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct WorkerSpecialization {
        specialization_name: String,
        work_exp_years: u16,
    }
    #[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Serialize, Deserialize)]
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

    use crate::{agency::VacancyTrait, generic_container::GenericContainer};

    pub fn sort_by_company_name<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.company_name().cmp(&b.company_name()));

        GenericContainer::new(data)
    }
    pub fn sort_by_specialization<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.specialization().cmp(&b.specialization()));

        GenericContainer::new(data)
    }
    pub fn sort_by_education<T, Q>(data: &T) -> GenericContainer<Q>
    where
        T: Clone + Iterator<Item = Q>,
        Q: VacancyTrait + Clone,
    {
        let mut data = data.clone().collect::<Vec<_>>();

        data.sort_by(|a, b| a.education().cmp(&b.education()));

        GenericContainer::new(data)
    }
}

mod generic_container {
    use std::{collections::LinkedList, fmt::Debug, u8};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    pub struct GenericContainer<T: Clone> {
        container: LinkedList<T>,
    }

    impl<T> GenericContainer<T>
    where
        T: for<'a> Deserialize<'a> + Clone,
    {
        pub fn deserialize(input: &[u8]) -> Result<Self, serde_json::Error> {
            Ok(Self {
                container: serde_json::from_slice::<LinkedList<T>>(input)?,
            })
        }
    }

    impl<T: Serialize + Clone> GenericContainer<T> {
        pub fn serialize(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string(&self.container)
        }
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

        pub fn delete_by_number(&mut self, id: usize) -> Result<T, ()> {
            if self.container.len() < id || id == 0 {
                return Err(());
            }
            let mut container_vec = self.container.clone().into_iter().collect::<Vec<_>>();
            let removed_value = container_vec.remove(id - 1);
            self.container = container_vec.into_iter().collect();
            Ok(removed_value)
        }
        pub fn push_back(&mut self, input: T) {
            self.container.push_back(input);
        }
    }

    impl<T: Clone> Iterator for GenericContainer<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.container.pop_front()
        }
    }
}

fn default_vacancies() -> Vec<Vacancy> {
    vec![
        Vacancy::new(
            "Company2",
            "IT",
            "Conditions1",
            "149200",
            Some("Educator".to_owned()),
            "12",
            "",
            Some("English,Spanish,Chinese".to_owned()),
            true,
        )
        .expect("Bad vacancy input "),
        Vacancy::new(
            "Company1",
            "",
            "Conditions2",
            "69100",
            Some("Teacher".to_owned()),
            "1",
            "school",
            Some("English,Chinese".to_owned()),
            true,
        )
        .expect("Bad vacancy input "),
        Vacancy::new(
            "Company3",
            "PR",
            "Conditions3",
            "19100",
            Some("Teacher".to_owned()),
            "24",
            "university",
            Some("German,English".to_owned()),
            true,
        )
        .expect("Bad vacancy input"),
    ]
}
