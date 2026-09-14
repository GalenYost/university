const { By, until } = require("selenium-webdriver");

class Header {
    static CART_QTY = By.className('cart-qty');
    static ACCOUNT_EMAIL = By.className('account');

    static Links = {
        login: By.className('ico-login'),
        register: By.className('ico-register'),
        logout: By.className('ico-logout'),
        cart: By.className('cart-label'),
    };

    static TopBarDropdowns = {
        computers: By.css('.top-menu a[href="/computers"]'),
    };

    static TopBarDropdownsOptions = {
        notebooks: {
            parent: this.TopBarDropdowns.computers,
            element: By.css('.top-menu a[href="/notebooks"]'),
        },
    };

    static async goto(driver, linkKey, expectedUrl = '') {
        const link = this.Links[linkKey];
        if (!link) throw new Error(`No key "${linkKey}" found in header links`);
        await driver.findElement(link).click();
        await driver.wait(until.urlContains(`/${expectedUrl}`), 5000);
    }

    static async goto_dropdown(driver, optionKey) {
        const option = this.TopBarDropdownsOptions[optionKey];
        if (!option) throw new Error(`No key "${optionKey}" found in dropdown options`);

        const parent = await driver.findElement(option.parent);
        await driver.actions().move({ origin: parent }).perform();

        const target = await driver.findElement(option.element);
        await target.click();
    }

    static async getCartItemCount(driver) {
        const text = await driver.findElement(this.CART_QTY).getText(); // Отримуємо рядок, наприклад "(3)"
        const match = text.match(/\d+/);
        return match ? parseInt(match[0], 10) : 0;
    }

    static async getAccountEmail(driver) {
        return await driver.findElement(this.ACCOUNT_EMAIL).getText();
    }

    static async isUserLoggedIn(driver) {
        const profileElements = await driver.findElements(this.ACCOUNT_EMAIL);
        return profileElements.length > 0;
    }
};

module.exports = Header;
