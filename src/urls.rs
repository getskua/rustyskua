use std::path::Path;


fn mirror(input: String, output: String, input_dir: String, output_dir: String) -> Result<String, FileConversionError> {
    let input_split: Vec<&str> = input.split(input_dir);
    if input_dir.len() == 3 {

    }
    else if input_dir.len() == 1 {

    }
    else {
        panic!("Error!");
    }
}