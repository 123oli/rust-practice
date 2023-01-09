use whatlang::detect;

fn main() {
    let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";

    let info = detect(text).unwrap();

    println!("{:?}", info.lang());
}