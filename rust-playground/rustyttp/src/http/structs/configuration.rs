pub struct Config {
    adress: Adress
}

impl Config {

    pub fn adress(&self) -> String {
        return format!("{}:{}", self.adress.0, self.adress.1);
    }
}

pub struct ConfigBuilder {
    adress: String,
    port: i16,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        return ConfigBuilder {
            adress: "127.0.0.1".to_string(),
            port: 7878
        }
    }

    pub fn adress(&self, adress: String) -> ConfigBuilder {
        return ConfigBuilder {
            adress: adress,
            ..*self
        }
    }

    pub fn port(&self, port: i16) -> ConfigBuilder {
        ConfigBuilder {
            port: port,
            adress: self.adress.clone()
        }
    }

    pub fn build(&self ) -> Config {
        return Config {
            adress: Adress(self.adress.clone(), self.port)
        }
    } 
}


struct Adress(String, i16);