fn draw_tree(triangles: usize) {
    for t in 0..triangles {
        for i in 0..(t + 2) {
            let stars = "*".repeat(1 + i * 2);
            let spaces = " ".repeat(triangles * 2 - i);
            println!("{}{}", spaces, stars);
        }
    }
}

#[test]
fn test() {
    draw_tree(7);
}
