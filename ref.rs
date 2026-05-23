fn main(){
    {
        let text = String::from("Hola Farnaz");
        let copia = &text;
        println!("Este es el nombre de mi polola: {:?}", text);
        println!("Esto es una copia {:?}", *copia);
        println!("{:p}", &text);
        println!("{:p}", copia);
    }
}