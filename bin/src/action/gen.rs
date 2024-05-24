use tokio::io::AsyncWriteExt;

use crate::{
    model::daily_info::QuestionInfo,
    util::path::{get_solution_by_id, get_solution_lib},
};

pub async fn create_template_file(daily_info: QuestionInfo) {
    let question_id = daily_info.base_info.id;
    let file_path = get_solution_by_id(question_id).unwrap();

    if file_path.exists() {
        println!("file already exists: {}", file_path.to_str().unwrap());
        return;
    }

    let mut file = tokio::fs::File::create(&file_path).await.unwrap();

    file.write_all(daily_info.to_string().as_bytes())
        .await
        .unwrap();

    let lib_path = get_solution_lib().unwrap();
    let mut lib_file = tokio::fs::File::options()
        .append(true)
        .open(lib_path)
        .await
        .unwrap();

    lib_file
        .write(format!("pub mod solution{};\n", question_id).as_bytes())
        .await
        .unwrap();
    lib_file.flush().await.unwrap();
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn it_works() {
        let file = tokio::fs::File::create("../solution/solution.rs")
            .await
            .unwrap();
        println!("{file:?}")
    }
}
