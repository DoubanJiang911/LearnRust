// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company { departments: HashMap::new() }
    }

    fn add_employee(&mut self, name: String, department: String) {
        self.departments.entry(department)
            .or_insert(Vec::new())
            .push(name)
    }

    fn get_department_employees(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }

    fn get_all_employees(&self) -> Vec<(&String, &Vec<String>)> {
        let mut departments: Vec<_> = self.departments.iter().collect();
        departments.sort_by(|a, b| a.0.cmp(b.0));
        departments.iter_mut().for_each(|(_, employees)| {
            let mut employees = employees.clone();
            employees.sort();
        });
        departments
    }
}

fn main() {
    let mut company = Company::new();

    company.add_employee("Sally".to_string(), "Engineering".to_string());
    company.add_employee("Amir".to_string(), "Sales".to_string());
    company.add_employee("Bob".to_string(), "Engineering".to_string());

    if let Some(employees) = company.get_department_employees("Engineering") {
        println!("Engineering: {:?}", employees);
    }

    let all_employees = company.get_all_employees();
    println!("All employees: \n{:?}", all_employees);
}
