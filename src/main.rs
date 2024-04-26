struct User
{
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

impl User
{
    fn print_nome(&self)
    {
        println!("Nome: {}", self.username);
    }

    fn print_ativo(&self)
    {
        println!("Ativo: {}", self.ativo);
    }
}

fn main()
{
    let pessoa = User{username: String::from("teste123"), email: String::from("teste123@a.com"), ativo: false, genero: String::from("feminino")};
    pessoa.print_nome();
    pessoa.print_ativo();
}
