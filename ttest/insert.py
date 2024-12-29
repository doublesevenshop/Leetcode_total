#!/usr/bin/env python
import os
import sys

def create_file_and_update_mod(number):
    # 设定文件夹路径
    leetcode_folder = os.path.join('src', 'leetcode')
    leetcode_rs_path = os.path.join('src', 'leetcode.rs')
    
    # 创建文件夹，如果不存在
    if not os.path.exists(leetcode_folder):
        os.makedirs(leetcode_folder)
    
    # 生成文件名 p+数字.rs
    file_name = f"p{number}.rs"
    file_path = os.path.join(leetcode_folder, file_name)
    
    # 力扣刷题模板
    template = f"""pub struct Solution;

impl Solution {{
    // 在此添加解题方法
}}

#[cfg(test)]
mod test {{
    use super::*;
    #[test]
    fn test_p{number}() {{
        // 在此编写测试代码
    }}
}}
"""
    
    # 在文件夹内创建文件并写入模板
    with open(file_path, 'w') as f:
        f.write(template)
    
    # 修改 leetcode.rs 文件，添加 pub mod <文件名> 的行
    with open(leetcode_rs_path, 'a') as f:
        f.write(f"pub mod p{number};\n")
    
    print(f"文件 p{number}.rs 创建成功，并已更新 leetcode.rs")

if __name__ == '__main__':
    # 从命令行获取输入的数字
    if len(sys.argv) < 2:
        print("请提供题号。例如：python create_files.py 123")
        sys.exit(1)
    
    # 获取需要创建的题号
    numbers = sys.argv[1:]
    
    for num in numbers:
        try:
            # 处理输入的题号，去除非数字部分
            number = int(num.lstrip('p'))
            create_file_and_update_mod(number)
        except ValueError:
            print(f"无效的题号: {num}. 请确保题号格式正确（如 p123）。")
