const { By } = require('selenium-webdriver');
const BaseFormPage = require('./BaseFormPage');

class LoginPage extends BaseFormPage {
    static ELEMENTS = {
        email: {
            by: By.name('Email'),
            type: 'input'
        },
        password: {
            by: By.name('Password'),
            type: 'input'
        },
        loginBtn: {
            by: By.className('login-button'),
            type: 'button'
        },
        errorField: {
            by: By.css('.validation-summary-errors'),
            type: 'message'
        }
    };

    static async login(driver, email, password) {
        await this.fillForm(driver, { email, password });
        await this.click(driver, 'loginBtn');
    }
}

module.exports = LoginPage;
