// Removing duplication by extracting the function
pub fn run() {
    let list = vec![23, 45, 54, 65, 67];
    let result = largest_gen(&list);
    println!(" list is : {}", result);

    let list = vec!['y', 't', 'u'];

    // create other function with same logic
    let result = largest_gen(&list);
    println!(" list 2 is : {}", result);
}

// replacing below largest function with Generics types
fn largest_gen<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn _largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}
