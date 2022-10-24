/// # Generic Search
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter2/GenericSearch.java
use classic_computer_science_problems::generic_search::{binary_contains, linear_contains};

fn main() {
    let xs = vec![1, 5, 15, 15, 15, 15, 20];
    let ys = vec!['a', 'd', 'e', 'f', 'z'];
    let zs = vec!["john", "mark", "ronald", "sarah"];
    let unsorted = vec!['b', 'a', 'c'];

    println!("Linear search on sorted lists");
    println!("Does xs contain `5`? {}", linear_contains(&xs, 5));
    println!("Does ys contain `'f'`? {}", linear_contains(&ys, 'f'));
    println!(
        "Does zs contain `\"sheila\"`? {}",
        linear_contains(&zs, "sheila")
    );

    println!("Linear search on unsorted lists");
    println!(
        "Does unsorted contain `'a'`? {}",
        linear_contains(&unsorted, 'a')
    );

    println!("Binary search on sorted lists");
    println!("Does xs contain `5`? {:?}", binary_contains(&xs, 5));
    println!("Does ys contain `'f'`? {:?}", binary_contains(&ys, 'f'));
    println!(
        "Does zs contain `\"sheila\"`? {:?}",
        binary_contains(&zs, "sheila")
    );

    println!("Binary search on unsorted lists");
    println!(
        "Does unsorted contain `'a'`? {:?}",
        binary_contains(&unsorted, 'a')
    );
}
