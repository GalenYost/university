const { By, Builder, Browser, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');
const assert = require('node:assert');

const URL = "http://demowebshop.tricentis.com/";
const BROWSER = Browser.FIREFOX;
const BROWSER_BIN = "/home/user/waterfox/waterfox";

const TEST_MAIL = "test@mailinator.com";
const TEST_PASS = "TEST123_123";

const EMAIL_FIELD = By.name('Email');
const PASSWORD_FIELD = By.name('Password');
const MY_ACCOUNT_ANCHOR = By.className('account');
const LOGIN_BTN = By.className('login-button');
const ERROR_MESSAGE = By.xpath('//div[@class="validation-summary-errors"]/span');

const EXPECTED_OUTPUT = "Login was unsuccessful";

const BROWSER_OPTIONS = new firefox.Options();
BROWSER_OPTIONS.setBinary(BROWSER_BIN);
BROWSER_OPTIONS.addArguments('-private');

async function task2() {
    const driver = await new Builder()
        .forBrowser(BROWSER)
        .setFirefoxOptions(BROWSER_OPTIONS)
        .build();

    try {
        await driver.get(URL);

        let myAccountAnchor = await driver.findElement(By.className(MY_ACCOUNT_ANCHOR));
        await myAccountAnchor.click();

        let emailField = await driver.findElement(By.name(EMAIL_FIELD));
        await emailField.sendKeys(TEST_MAIL);

        let passwordField = await driver.findElement(By.name(PASSWORD_FIELD));
        await passwordField.sendKeys(TEST_PASS);

        let loginButton = await driver.findElement(By.className(LOGIN_BTN));
        await loginButton.click();
        
        let errorElement = await driver.wait(
            until.elementLocated(By.xpath(ERROR_MESSAGE)), 
            5000
        );

        let actualText = await errorElement.getText();

        console.log(actualText);
        assert.ok(
            actualText.includes(EXPECTED_OUTPUT),
            `Expected text to contain '${EXPECTED_OUTPUT}', but got '${actualText}'`
        );
    } catch (err) {
        console.error(err);
    } finally {
        process.stdin.resume();
        await new Promise(resolve => process.stdin.once('data', resolve));
        await driver.quit();
    }
}

task2();
