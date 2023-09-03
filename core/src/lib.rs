mod engine;
pub use engine::config::Config;
use regex::Regex;
use thirtyfour::prelude::*;

pub struct Task {
    pub addr: String,
    pub value: String,
    config: Config,
}

impl Task {
    #[inline]
    pub fn new(addr: String, value: String) -> Self {
        let config = Config::new().unwrap();
        Self {
            addr,
            value,
            config,
        }
    }

    pub async fn embark(&self) -> WebDriver {
        let caps = DesiredCapabilities::chrome();
        let _conn = self.config.conn();
        WebDriver::new(_conn, caps).await.unwrap()
    }

    pub async fn enter_frame(&self, driver: &WebDriver) -> WebDriverResult<()> {
        let _ = driver.goto(&self.addr).await?;
        let _ = driver
            .find(By::ClassName("Home_iframe__T3nfU"))
            .await?
            .enter_frame()
            .await?;
        let x = driver.find(By::Id("QR~QID65~6")).await?;
        x.wait_until().displayed().await?;
        x.send_keys(&self.value).await?;
        let _ = click_shadow(driver, "NextButton").await?;
        Ok(())
    }

    pub async fn post(&self, driver: &WebDriver) -> WebDriverResult<String> {
        for data in self.config.get_data() {
            for d in data {
                let _ = click_shadow(driver, &format!("{}-label", d)).await?;
            }
            let _ = click_shadow(driver, "NextButton").await?;
        }
        let _ = click_shadow(driver, "QR~QID45").await?;
        let _ = click_shadow(driver, "NextButton").await?;
        let _ = click_shadow(driver, "EndOfSurvey").await?;
        let x = driver.find(By::Tag("strong")).await?.text().await?;
        let res = Regex::new(r"\d+").unwrap().find(&x).unwrap().as_str();
        Ok(res.to_owned())
    }

    pub async fn auto(&self) -> WebDriverResult<()> {
        let dr = self.embark().await;
        let _ = self.enter_frame(&dr).await;
        let res = self.post(&dr).await;
        dr.quit().await?;
        print!("ref={:?}", res.unwrap());
        Ok(())
    }
}

pub async fn run(addr: String, key: String, value: String) -> WebDriverResult<()> {
    let _k = key.as_bytes().to_vec();
    // let c = engine::cipher::encode(&addr, &_k);
    // print!("_en_addr={:}", c);
    let _addr = engine::cipher::decode(&addr, &_k);
    // print!("{:?}", _addr);
    let value = if value.is_empty() { engine::gen_value() } else { value };
    let _ = Task::new(_addr, value).auto().await;
    Ok(())
}

pub async fn click_shadow(driver: &WebDriver, id: &str) -> WebDriverResult<()> {
    let x = driver.query(By::Id(id)).first().await?;
    let args = vec![x.to_json()?];
    let _ = driver.execute("arguments[0].click();", args).await;
    Ok(())
}
