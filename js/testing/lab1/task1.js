const { By, Builder, Browser, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');

const URL = "https://pastebin.com";
const BROWSER = Browser.FIREFOX;
const BROWSER_BIN = "/home/user/waterfox/waterfox";

const BROWSER_OPTIONS = new firefox.Options();
BROWSER_OPTIONS.setBinary(BROWSER_BIN);
BROWSER_OPTIONS.addArguments('-private');

const PASTE_TITLE = "TEST_TITLE_123";
const PASTE_TEXT = "TEST_123";

const TEXT_AREA = By.id('postform-text');
const TITLE_FIELD = By.id('postform-name');
const EXPIRATION = By.id('select2-postform-expiration-container');
const TEN_MINS_EXP_OPTION = By.xpath('//li[contains(@class, "select2-results__option") and text()="10 Minutes"]');

async function task1() {
    const driver = await new Builder()
        .forBrowser(BROWSER)
        .setFirefoxOptions(BROWSER_OPTIONS)
        .build();

    try {
        await driver.get(URL);

        let textArea = await driver.findElement(TEXT_AREA);
        let titleField = await driver.findElement(TITLE_FIELD);
        
        await textArea.sendKeys(PASTE_TEXT);
        await titleField.sendKeys(PASTE_TITLE);

        let expOption = await driver.findElement(EXPIRATION);
        await expOption.click();

        let expOptionSelect = await driver.wait(
            until.elementLocated(TEN_MINS_EXP_OPTION), 
            5000
        );
        await expOptionSelect.click();
    } catch (err) {
        console.error(err);
    } finally {
        process.stdin.resume();
        await new Promise(resolve => process.stdin.once('data', resolve));
        await driver.quit();
    }
}

task1();
