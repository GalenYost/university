const { until } = require('selenium-webdriver');

class BasePage {
    static ELEMENTS = {};

    static async fillForm(driver, formData, timeout = 5000) {
        for (const [key, value] of Object.entries(formData)) {
            const field = this.ELEMENTS[key];
            if (!field) continue;

            const element = await driver.wait(
                until.elementIsVisible(driver.findElement(field.by)),
                timeout
            );

            if (field.type === 'input') {
                await element.clear();
                await element.sendKeys(value);
            }
        }
    }

    static async click(driver, elementKey) {
        const field = this.ELEMENTS[elementKey];
        if (!field) throw new Error(`Element key "${elementKey}" not found`);
        const element = await driver.findElement(field.by);
        await element.click();
    }

    static async getMessage(driver, elementKey = 'errorField', timeout = 3000) {
        try {
            const field = this.ELEMENTS[elementKey];
            const element = await driver.wait(
                until.elementIsVisible(driver.findElement(field.by)),
                timeout
            );
            return await element.getText();
        } catch {
            return '';
        }
    }
}

module.exports = BasePage;
