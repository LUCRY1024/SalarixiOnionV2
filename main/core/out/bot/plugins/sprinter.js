import { updateBotTask } from '../base.js';
export class SprinterPlugin {
    async flow(username, bot, options) {
        if (options.state === 'start') {
            await this.enableSprinter(username, bot);
        }
        else {
            await this.disableSprinter(username, bot);
        }
    }
    async enableSprinter(username, bot) {
        updateBotTask(username, 'sprinter', true, 2.1);
        bot.physics.sprintSpeed = bot.physics.sprintSpeed + 0.8;
        bot.physics.yawSpeed = bot.physics.yawSpeed + 0.8;
    }
    async disableSprinter(username, bot) {
        updateBotTask(username, 'sprinter', false);
        bot.physics.sprintSpeed = bot.physics.sprintSpeed - 0.8;
        bot.physics.yawSpeed = bot.physics.yawSpeed - 0.8;
    }
}
