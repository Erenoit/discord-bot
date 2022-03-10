import { Message, MessageActionRow, MessageButton, MessageButtonOptions,
         MessageButtonStyle, MessageComponentInteraction,
         MessageEmbedOptions, MessageOptions } from "discord.js"
import { Variables } from "../Interfaces";

class Messager {
  private use_embed: boolean = false;
  private colors = {
    sucsess: 0x00ff00,
    normal:  0x0000ff,
    error:   0xff0000,
  };

  public async send_sucsess(variables: Variables, content: string, log_text?: string) {
    const title = "Sucsess";
    await this.send_message(variables, title, content, this.colors.sucsess, log_text);
  }

  public async send_normal(variables: Variables, title: string, content: string, log_text?: string) {
    await this.send_message(variables, title, content, this.colors.normal, log_text);
  }

  public async send_err(variables: Variables, content: string, log_text?: string) {
    const title = "Error";
    await this.send_message(variables, title, content, this.colors.error, log_text);
  }

  public async send_message(variables: Variables, title: string,
                      content: string, color: number,
                      log_text?: string) {
    const msg: MessageOptions  = this.use_embed ? { embeds: [this.basic_embed(title, content, color)] }
                                                : { content };

    await this.send(variables, msg);

    if (log_text) {
      console.log(log_text);
    }
  }

  // TODO: find a way to not use (or better way to use) function pointers
  public async send_confirm(variables: Variables,
                      call_func: Function, func_this: any, func_params: any[],
                      additional_text?: string, end_text?: string) {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const default_message = "Are you sure?";
    const row = new MessageActionRow()
                .addComponents(this.create_button("confirm_yes", "Yes", "SUCCESS"),
                               this.create_button("confirm_no", "No", "DANGER"));
    const msg: MessageOptions = {
      content: additional_text ? additional_text + " " + default_message
                               : default_message,
      components: [row]
    };

    await this.send(variables, msg);

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collector = channel.createMessageComponentCollector({
      filter,
      max: 1, // take input only once
      time: 10 * 1000 // 10sec time limit
    });

    // TODO: use reason variable to respond specific to reasons
    collector.on("end", (collection, reason) => {
      const btn_inter = collection.first();
      if (!btn_inter) { return; }
      const answer = btn_inter.customId;
      const btn_msg = btn_inter.message as Message;

      btn_msg.edit({
        content: end_text || "An action has already been taken",
        components: []
      });

      if (answer === "confirm_yes") {
        call_func.apply(func_this, func_params);
      }
    });
  }

  private async send(variables: Variables, msg: MessageOptions) {
    const main = variables.type === "Old" ? variables.message : variables.interaction;

    if (variables.type === "New" && variables.interaction.replied) {
      await variables.interaction.followUp(msg);
    } else {
      await main.reply(msg);
    }
  }

  private basic_embed(title: string, description: string, color: number): MessageEmbedOptions {
    const embed: MessageEmbedOptions = {
      color,
      title,
      description,
    } 
    return embed;
  }

  private create_button(customId: string, label: string,
                        style: Exclude<MessageButtonStyle, "LINK">,
                        emoji?: string): MessageButton {
    const button_options: MessageButtonOptions = {
      style,
      customId,
      label,
      emoji
    }

    return new MessageButton(button_options);
  }
}

export default Messager;
