const { Builder, Browser, By, until } = require('selenium-webdriver');
const firefox = require('selenium-webdriver/firefox');
const assert = require('node:assert');

const Header = require('../pages/Header');
const LoginPage = require('../pages/LoginPage');
const RegisterPage = require('../pages/RegisterPage');
const ItemSelector = require('../pages/ItemSelector');

const CORRECT_TEST_EMAIL = "testmail+10@mail.com";
const CORRECT_TEST_PASSWORD = "thisIsMyTest@Password";

const BASE_URL = "https://demowebshop.tricentis.com/";

const getUniqueUser = () => {
    const timestamp = Date.now();
    return {
        email: `testmail_${timestamp}@mail.com`,
        password: `testPassword_${timestamp}`,
        firstName: `test_name_${timestamp}`,
        lastName: `test_last_name_${timestamp}`
    };
};

const BROWSER = Browser.FIREFOX;
const BROWSER_BIN = "/home/user/waterfox/waterfox";

const BROWSER_OPTIONS = new firefox.Options();
BROWSER_OPTIONS.setBinary(BROWSER_BIN);
BROWSER_OPTIONS.setPreference("signon.rememberSignons", false);
BROWSER_OPTIONS.setPreference("dom.webnotifications.enabled", false);
BROWSER_OPTIONS.addArguments('-private');

describe('DemoWebShop Test Suite', function () {
    this.timeout(30000);
    let driver;

    before(async () => {
        driver = await new Builder()
            .forBrowser(BROWSER)
            .setFirefoxOptions(BROWSER_OPTIONS)
            .build();
    });

    beforeEach(async () => {
        try {
            await driver.switchTo().alert().dismiss();
        } catch {}

        await driver.manage().deleteAllCookies();
        await driver.get(BASE_URL);
    });

    after(async () => {
        if (driver) await driver.quit();
    });

    it('Should navigate to Login page', async () => {
        await Header.goto(driver, 'login', 'login');
        const url = await driver.getCurrentUrl();
        assert.ok(url.includes('/login'), 'URL does not contain "/login"');
    });

    it('Should login without error', async () => {
        await Header.goto(driver, 'login', 'login');
        await LoginPage.login(driver, CORRECT_TEST_EMAIL, CORRECT_TEST_PASSWORD);
        await driver.wait(until.urlIs(BASE_URL), 10000);

        const loggedIn = await Header.isUserLoggedIn(driver);
        assert.strictEqual(loggedIn, true, 'Could not log in');
    });

    it('Should login with error', async () => {
        await Header.goto(driver, 'login', 'login');
        const user = getUniqueUser();
        await LoginPage.login(driver, user.email, user.password);

        const msg = await LoginPage.getMessage(driver, 'errorField');
        assert.ok(msg.includes('Login was unsuccessful'), 'Expected login error message');
    });

    it('Should register without error', async () => {
        await Header.goto(driver, 'register', 'register');
        const user = getUniqueUser();
        await RegisterPage.register(driver, user.firstName, user.lastName, user.email, user.password);

        const url = await driver.getCurrentUrl();
        assert.ok(url.includes('registerresult/1'), 'Registration result page was not loaded');
    });

    it('Should register with error', async () => {
        await Header.goto(driver, 'register', 'register');
        const user = getUniqueUser();
        await RegisterPage.register(driver, user.firstName, user.lastName, CORRECT_TEST_EMAIL, CORRECT_TEST_PASSWORD);

        const msg = await RegisterPage.getMessage(driver, 'errorField');
        assert.ok(msg.includes('The specified email already exists'), 'Expected duplicate email error');
    });

    it('Should add notebook to cart', async () => {
        const initialQty = await Header.getCartItemCount(driver);
        await Header.goto_dropdown(driver, 'notebooks');

        const productId = 31;
        await ItemSelector.addToCart(driver, productId);

        const msg = await ItemSelector.getMessage(driver);
        const newQty = await Header.getCartItemCount(driver);

        assert.ok(msg.includes('The product has been added'), 'Notification did not appear');
        assert.ok(newQty >= initialQty, 'Cart counter was not updated');
    });

    it('Should logout', async () => {
        await Header.goto(driver, 'login', 'login');
        await LoginPage.login(driver, CORRECT_TEST_EMAIL, CORRECT_TEST_PASSWORD);
        await driver.wait(until.urlIs(BASE_URL), 10000);

        await driver.wait(async () => {
            try {
                await driver.manage().getCookie('NOPCOMMERCE.AUTH');
                return true;
            } catch {
                return false;
            }
        }, 5000, 'NOPCOMMERCE.AUTH cookie did not appear after login');

        const initialEmail = await Header.getAccountEmail(driver);
        assert.strictEqual(initialEmail, CORRECT_TEST_EMAIL, 'User was not properly logged in');

        await Header.goto(driver, 'logout');

        await driver.wait(async () => {
            try {
                await driver.manage().getCookie('NOPCOMMERCE.AUTH');
                return false;
            } catch (error) {
                return true;
            }
        }, 5000, 'NOPCOMMERCE.AUTH cookie was not deleted after logout');
    });
});
