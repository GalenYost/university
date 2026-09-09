const { By, Builder, Browser, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');
const assert = require('node:assert');

const URL = "http://demowebshop.tricentis.com/";
const BROWSER = Browser.FIREFOX;
const BROWSER_BIN = "/home/user/waterfox/waterfox";

const TEST_MAIL = "test@mailinator.com";
const TEST_PASS = "TEST123_123";

const MY_ACCOUNT_ANCHOR_CLASS = "account";
const EMAIL_FIELD_NAME = "Email";
const PASSWORD_FIELD_NAME = "Password";
const LOGIN_BTN_CLASS = "login-button";
const ERROR_XPATH = "//div[@class='validation-summary-errors']/span";

const EXPECTED_OUTPUT = "Login was unsuccessful";

async function task2() {
    const opts = new firefox.Options();
    opts.setBinary(BROWSER_BIN);
    opts.addArguments('-private');

    const driver = await new Builder()
        .forBrowser(BROWSER)
        .setFirefoxOptions(opts)
        .build();

    try {
        await driver.get(URL);

        let myAccountAnchor = await driver.findElement(By.className(MY_ACCOUNT_ANCHOR_CLASS));
        await myAccountAnchor.click();

        let emailField = await driver.findElement(By.name(EMAIL_FIELD_NAME));
        await emailField.sendKeys(TEST_MAIL);

        let passwordField = await driver.findElement(By.name(PASSWORD_FIELD_NAME));
        await passwordField.sendKeys(TEST_PASS);

        let loginButton = await driver.findElement(By.className(LOGIN_BTN_CLASS));
        await loginButton.click();
        
        let errorElement = await driver.wait(
            until.elementLocated(By.xpath(ERROR_XPATH)), 
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
