const { By } = require('selenium-webdriver');
const BaseFormPage = require('./BaseFormPage');

class RegisterPage extends BaseFormPage {
    static ELEMENTS = {
        firstName: {
            by: By.id('FirstName'),
            type: 'input'
        },
        lastName: {
            by: By.id('LastName'),
            type: 'input'
        },
        email: {
            by: By.id('Email'),
            type: 'input'
        },
        password: {
            by: By.id('Password'),
            type: 'input'
        },
        confirmPassword: {
            by: By.id('ConfirmPassword'),
            type: 'input'
        },
        registerBtn: {
            by: By.id('register-button'),
            type: 'button'
        },
        errorField: {
            by: By.css('.validation-summary-errors'),
            type: 'message'
        }
    };

    static async register(driver, firstName, lastName, email, password, confirmPassword = password) {
        await this.fillForm(driver, { firstName, lastName, email, password, confirmPassword });
        await this.click(driver, 'registerBtn');
    }
}

module.exports = RegisterPage;
