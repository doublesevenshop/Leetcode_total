use crate::student_manager::StudentManager;
use crate::model::Student;
use std::fs::{read, File};
use std::error::Error;


pub fn read_from_csv(file_path: &str) -> Result<Vec<Student>, Box<dyn Error>> {

    let mut reader = csv::Reader::from_path(file_path)?;
    let mut students = Vec::new();
    for result in reader.deserialize() {
        students.push(result?);
    }
    Ok(students)
}

pub fn execute_command(
    command: String, 
    manager: &mut StudentManager) 
    -> Result<String, Box<dyn Error>> {
    
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.as_slice() {
        ["import", "csv", file_path] => {
            let students = read_from_csv(file_path)?;
            *manager = StudentManager::new(students);
            Ok(format!("导入成功，已加载{}条数据", manager.count()))
        }
        ["find", "ID", ids @ ..] => {
            let ids: Vec<u32> = ids.
                iter().
                filter_map(|id|id.parse().ok()).
                collect();
            if ids.is_empty() {
                return Ok("未提供有效ID".into());
            }
            let results: Vec<String> = manager
                .find_by_ids(&ids)
                .into_iter()
                .map(|opt|match opt {
                    Some(student) => format!("{:?}", student),
                    None => "未找到学生".to_string(),
                })
                .collect();
            Ok(format!("查询结果:\n{}", results.join("\n")))
        }
        ["find", "name", names @..] => {
            let results: Vec<String> = manager
                .find_by_names(&names)
                .into_iter()
                .map(|opt| match opt {
                    Some(student) => format!("{:?}", student),
                    None => "未找到学生".to_string(),
                })
                .collect();
            Ok(format!("查询结果：\n{}", results.join("\n")))
        }
        ["status", "gender"] => {
            let (male, female) = manager.count_by_gender();
            Ok(format!("性别统计：\n男:{} 人\n女:{} 人", male, female))
        }
        
        ["export", "json", file_path] => {
            manager.export_to_json(file_path)?;
            Ok(format!("数据导出成功，文件已保存到{}", file_path))
        }
        _ => Ok("无效命令".into()),
    }
}
