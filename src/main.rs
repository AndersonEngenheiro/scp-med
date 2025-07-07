use headless_chrome::{Browser, LaunchOptions};
use std::io; // Para ler a entrada do usuário


fn main() {
    let options = LaunchOptions {
        headless: false,
        
        // Adicione as configurações de proxy se necessário
        // proxy: Some(headless_chrome::Proxy::Http {
        //     host: "proxy.sgi.ms.gov.br".to_string(),
        //     port: 8081, // Substitua pela porta do seu proxy
        // }),
        ..Default::default()
    };

    let browser = Browser::new(options).unwrap();
    let tab = browser.new_tab().unwrap();

    // Tente navegar para uma URL diferente, se necessário
    tab.navigate_to("https://www.uol.com.br/").unwrap();
    tab.wait_until_navigated().unwrap();


        // Esperar pela entrada do usuário
        println!("Pressione Enter para fechar o navegador...");
        let _ = io::stdin().read_line(&mut String::new()).unwrap(); // Aguarda até o usuário pressionar Enter

}