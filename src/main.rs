use std::cmp;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn read_file_lines(ruta_de_archivo: &str) -> Vec<String> {
    let path = Path::new(ruta_de_archivo);
    let display = path.display();

    let file = match File::open(&path) {
        Err(_why) => panic!("No se pudo abrir el archivo {}: ", display),
        Ok(file) => file,
    };
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("No se pudo parsear la linea"))
        .collect()
}

fn print_longest_common_subsequence_diff(x: String, y: String) {
    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();

    let grid = build_lcs_grid(&x_chars, &y_chars);
    print_diff(&grid, &x_chars, &y_chars, x_chars.len(), y_chars.len());
}

#[allow(dead_code)]
fn get_longest_common_subsequence_diff(x: String, y: String) -> String {
    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();

    let grid = build_lcs_grid(&x_chars, &y_chars);
    build_diff(
        &grid,
        &x_chars,
        &y_chars,
        x_chars.len(),
        y_chars.len(),
        String::new(),
    )
}

fn print_diff(grid: &Vec<Vec<i32>>, x_chars: &Vec<char>, y_chars: &Vec<char>, i: usize, j: usize) {
    if i > 0 && j > 0 && x_chars[i - 1] == y_chars[j - 1] {
        print_diff(grid, x_chars, y_chars, i - 1, j - 1);
        print!("{}", x_chars[i - 1]);
    } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
        print_diff(grid, x_chars, y_chars, i, j - 1);
        print!("\x1B[42m{}\x1b[0m", y_chars[j - 1]);
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        print_diff(grid, x_chars, y_chars, i - 1, j);
        print!("\x1B[41m{}\x1b[0m", x_chars[i - 1]);
    } else {
        print!("");
    }
}

#[allow(dead_code)]
fn build_diff(
    grid: &Vec<Vec<i32>>,
    x_chars: &Vec<char>,
    y_chars: &Vec<char>,
    i: usize,
    j: usize,
    mut diff: String,
) -> String {
    if i > 0 && j > 0 && x_chars[i - 1] == y_chars[j - 1] {
        diff = build_diff(grid, x_chars, y_chars, i - 1, j - 1, diff);
        diff.push_str(&format!("{}", x_chars[i - 1]));
    } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
        diff = build_diff(grid, x_chars, y_chars, i, j - 1, diff);
        diff.push_str(&format!("+{}", y_chars[j - 1]));
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        diff = build_diff(grid, x_chars, y_chars, i - 1, j, diff);
        diff.push_str(&format!("-{}", x_chars[i - 1]));
    }

    return diff;
}

fn build_lcs_grid(x_chars: &Vec<char>, y_chars: &Vec<char>) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; y_chars.len() + 1]; x_chars.len() + 1];

    for i in 0..x_chars.len() {
        for j in 0..y_chars.len() {
            if x_chars[i] == y_chars[j] {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else {
                grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
            }
        }
    }

    return grid;
}

fn main() {
    let texto_uno = read_file_lines("src/ejemplo1.txt");
    let texto_dos = read_file_lines("src/ejemplo2.txt");

    let frase_uno = &texto_uno[0];
    let frase_dos = &texto_dos[0];

    println!("Frase vieja: {}", frase_uno);
    println!("Frase nueva: {}", frase_dos);
    print!("Diff: ");
    print_longest_common_subsequence_diff(String::from(frase_uno), String::from(frase_dos));
    println!("");
}

#[cfg(test)]
mod test_module {

    use super::get_longest_common_subsequence_diff;

    #[test]
    fn test_01_random_str() {
        let x = "abcd";
        let y = "adbc";

        let diff = get_longest_common_subsequence_diff(String::from(x), String::from(y));
        assert_eq!(&diff, "a+dbc-d");
    }

    #[test]
    fn test_02_same_str() {
        let x = "buenos dias";
        let y = "buenos dias";

        let diff = get_longest_common_subsequence_diff(String::from(x), String::from(y));
        assert_eq!(&diff, "buenos dias");
    }

    #[test]
    fn test_03_add_completly_new_str() {
        let x = "";
        let y = "buenos dias";

        let diff = get_longest_common_subsequence_diff(String::from(x), String::from(y));
        assert_eq!(&diff, "+b+u+e+n+o+s+ +d+i+a+s");
    }

    #[test]
    fn test_04_remove_all_the_str() {
        let x = "buenos dias";
        let y = "";

        let diff = get_longest_common_subsequence_diff(String::from(x), String::from(y));
        assert_eq!(&diff, "-b-u-e-n-o-s- -d-i-a-s");
    }
}
