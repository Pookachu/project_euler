use project_euler::sequences::CollatzSequence;

fn main() {
    let mut longest_chain = 0;
    let mut best_start = 0;
    for i in 0..1_000_000 {
        let length = CollatzSequence::new(i).count();

        if length > longest_chain {
            longest_chain = length;
            best_start = i;
        }
    }

    println!(
        "Starting number {} produces a chain of length {}",
        best_start, longest_chain
    );
}
