fn main() {
    let csv_data = "

name,age


    name1,  1
name2, 2

    name3, 3

name4, 4

    name5,  5


name6,  6


";

    let records = csv_data.lines();

    // 有效数据行号
    let mut line_num = 0;
    for record in records {
        let is_empty_line = record.trim().len() == 0;

        if is_empty_line {
            continue;
        }
        // 跳过表头
        if line_num == 0 {
            line_num += 1;
            continue;
        }

        let parsed_data = record
            .split(',')
            .map(|field| field.trim())
            .collect::<Vec<_>>();

        let name = parsed_data[0];
        let age = parsed_data[1];
        println!("{line_num}: name is {name}, age is {}", age);

        line_num += 1;
    }
}
