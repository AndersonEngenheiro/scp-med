use headless_chrome::{Browser, LaunchOptions};
use std::io; // Para ler a entrada do usuário
use std::time::Duration;

fn uol() -> Result<(), Box<dyn std::error::Error + 'static>> {
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

    tab.wait_for_element (".email__link.email__link--loggedOut")?.click()?;
    tab.wait_for_element_with_custom_timeout(".email__link.email__link--loggedOut", Duration::from_millis(2000));
    tab.type_str("leandroribeiro1999@yahoo.com")?.press_key("Enter")?;

    // Esperar pela entrada do usuário
    println!("Pressione Enter para fechar o navegador...");
    let _ = io::stdin().read_line(&mut String::new()).unwrap(); // Aguarda até o usuário pressionar Enter

    Ok(())
}

fn main(){
    uol();
}