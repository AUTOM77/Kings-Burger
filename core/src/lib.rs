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
        let _conn = self.config.conn();
        let mut caps = DesiredCapabilities::chrome();
        let _ = caps.add_chrome_arg("--headless");
        let _ = caps.add_chrome_arg("--no-sandbox");
        let _ = caps.add_chrome_arg("--disable-gpu");
        let _ = caps.add_chrome_arg("--disable-dev-shm-usage");
        let _ = caps.add_chrome_option("excludeSwitches", vec!["enable-automation"]);
        WebDriver::new(_conn, caps).await.unwrap()
    }

    pub async fn enter_frame(&self, driver: &WebDriver) -> WebDriverResult<()> {
        let _ = driver.goto(&self.addr).await?;
        let _ = driver
            .find(By::ClassName(self.config.frame()))
            .await?
            .enter_frame()
            .await?;
        let x = driver.find(By::Id("QR~QID65~6")).await?;
        x.wait_until().displayed().await?;
        x.send_keys(&self.value).await?;
        Ok(())
    }

    pub async fn post(&self, driver: &WebDriver) -> WebDriverResult<()> {
        for data in self.config.get_data() {
            let _ = click_shadow(driver, "NextButton").await?;
            for d in data {
                let _ = click_shadow(driver, &format!("{}-label", d)).await?;
            }
        }
        for r in self.config.get_roll() {
            let _ = click_shadow(driver, r).await?;
        }
        Ok(())
    }

    pub async fn trivial(&self, driver: &WebDriver) -> WebDriverResult<String> {
        let _ = self.enter_frame(&driver).await;
        let _ = self.post(&driver).await;
        let x = driver.find(By::Tag("strong")).await?.text().await?;
        let res = Regex::new(r"\d+").unwrap().find(&x).unwrap().as_str();
        Ok(res.to_owned())
    }

    pub async fn auto(&self) -> WebDriverResult<()> {
        let _chrome = self.embark().await;
        let res = self.trivial(&_chrome).await;
        _chrome.quit().await?;
        print!("ref={:?} ", res.unwrap());
        Ok(())
    }
}

pub async fn run(addr: String, key: String, value: String) -> WebDriverResult<()> {
    let _k = key.as_bytes().to_vec();
    // let c = engine::cipher::encode(&addr, &_k);
    // print!("_en_addr={:}", c);
    let _addr = engine::cipher::decode(&addr, &_k);
    let value = if value.is_empty() { engine::gen_value() } else { value };
    print!("value={:} ", value);
    let _ = Task::new(_addr, value)
        .auto()
        .await;
    Ok(())
}

pub async fn click_shadow(driver: &WebDriver, id: &str) -> WebDriverResult<()> {
    let x = driver.query(By::Id(id)).first().await?;
    let args = vec![x.to_json()?];
    let _ = driver.execute("arguments[0].click();", args).await;
    Ok(())
}
