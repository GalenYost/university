const { By, Builder, Browser, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');

const URL = "https://pastebin.com";
const BROWSER = Browser.FIREFOX;

const PASTE_TITLE = "TEST_TITLE_123";
const PASTE_TEXT = "TEST_123";

const TEXT_AREA_ID = "postform-text";
const TITLE_FIELD_ID = "postform-name";
const EXPIRATION_ID = "select2-postform-expiration-container";
const CREATE_BUTTON_XPATH = "//button[@type='submit']";

async function task1() {
    const opts = new firefox.Options();
    opts.addArguments('-private');

    const driver = await new Builder()
        .forBrowser(BROWSER)
        .setFirefoxOptions(opts)
        .build();

    try {
        await driver.get(URL);

        let textArea = await driver.findElement(By.id(TEXT_AREA_ID));
        let titleField = await driver.findElement(By.id(TITLE_FIELD_ID));
        
        await textArea.sendKeys(PASTE_TEXT);
        await titleField.sendKeys(PASTE_TITLE);

        let expOption = await driver.findElement(By.id(EXPIRATION_ID));
        await expOption.click();

        const tenMinsOptionXpath = "//li[contains(@class, 'select2-results__option') and text()='10 Minutes']";
        let expOptionSelect = await driver.wait(
            until.elementLocated(By.xpath(tenMinsOptionXpath)), 
            5000
        );
        await expOptionSelect.click();

        let createButton = await driver.findElement(By.xpath(CREATE_BUTTON_XPATH));
        await createButton.click();
    } catch (err) {
        console.error(err);
    } finally {
        process.stdin.resume();
        await new Promise(resolve => process.stdin.once('data', resolve));
        await driver.quit();
    }
}

task1();
