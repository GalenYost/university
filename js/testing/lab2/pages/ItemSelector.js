const { By, until } = require('selenium-webdriver');

class ItemSelector {
    static async addToCart(driver, productId, timeout = 5000) {
        const btnSelector = By.css(`.product-item[data-productid="${productId}"] .product-box-add-to-cart-button`);
        
        const element = await driver.wait(until.elementLocated(btnSelector), timeout);
        const button = await driver.wait(until.elementIsVisible(element), timeout);
        await button.click();
    }

    static async getMessage(driver, timeout = 5000) {
        const msgSelector = By.css('#bar-notification .content');

        const element = await driver.wait(until.elementLocated(msgSelector), timeout);
        await driver.wait(until.elementIsVisible(element), timeout);
        return await element.getText();
    }
}

module.exports = ItemSelector;