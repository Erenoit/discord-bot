import { Message, MessageActionRow, MessageButton, MessageButtonOptions,
         MessageButtonStyle, MessageComponentInteraction,
         MessageEmbedOptions, MessageOptions } from "discord.js"
import { Variables } from "../Interfaces";

class Messager {
  private use_embed: boolean = false;
  private time_limit: number = 30 * 1000;
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
      time: this.time_limit
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

  public async send_selection(variables: Variables,
                              list: Array<{name: string, id: string, disabled: boolean}>,
                              call_func: Function, func_this: any,
                              title?: string, content?: string, end_text?: string) {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const msg_content = content ? content : "Select one of them:";
    const msg_title = title ? title : "Select";
    const main_row = new MessageActionRow()
                .addComponents(
                  ...list.map(({name, id, disabled}) => {
                    return this.create_button(id, name, "PRIMARY", disabled);
                  }));
    let msg: MessageOptions = {
      components: [main_row]
    };

    if (this.use_embed) {
      msg = {
        ...msg,
        embeds: [this.basic_embed(msg_title, msg_content, this.colors.normal)],
      };
    } else {
      msg = {
        ...msg,
        content: msg_content,
      };
    }

    await this.send(variables, msg);

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collector = channel.createMessageComponentCollector({
      filter,
      max: 1, // take input only once
      time: this.time_limit
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

      if (answer === "none") {
        return;
      } else if (answer === "all") {
        list.forEach(({name, id}) => {
          call_func.apply(func_this, [variables, id]);
        });
      } else {
        call_func.apply(func_this, [variables, answer]);
      }
    });
  }

  public async send_selection_from_list(variables: Variables,
                                        list: Array<{name: string, id: string, disabled: boolean}>,
                                        use_second_row: boolean,
                                        call_func: Function, func_this: any,
                                        title?: string, content?: string, end_text?: string) {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const msg_content = content ? content : "Select one of them:";
    const msg_title = title ? title : "Select";
    const main_row = new MessageActionRow()
                .addComponents(
                  ...list.map(({name, id, disabled}, index) => {
                    return this.create_button(id, (index + 1).toString(), "PRIMARY", disabled);
                  }));
    const secondary_row = new MessageActionRow()
                .addComponents(
                  this.create_button("all", "All", "SUCCESS"),
                  this.create_button("none", "None", "DANGER"));
    let msg: MessageOptions = {
      components: use_second_row ? [main_row, secondary_row] : [main_row]
    };

    if (this.use_embed) {
      msg = {
        ...msg,
        embeds: [this.embed_list(msg_title, msg_content, list.map((e) => {return e.name;}), true)],
      };
    } else {
      msg = {
        ...msg,
        content: this.normal_list(msg_content, list.map((e) => {return e.name;}), true),
      };
    }

    await this.send(variables, msg);

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collector = channel.createMessageComponentCollector({
      filter,
      max: 1, // take input only once
      time: this.time_limit
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

      if (answer === "none") {
        return;
      } else if (answer === "all") {
        list.forEach(({name, id}) => {
          call_func.apply(func_this, [variables, id]);
        });
      } else {
        call_func.apply(func_this, [variables, answer]);
      }
    });
  }

  public async send_list(variables: Variables, title: string, content: string, list: string[],
                         use_nums: boolean = false, start_number: number = 1,
                         select?: number) {
    const msg: MessageOptions = this.use_embed
                              ? { embeds: [this.embed_list(title, content, list,
                                                           use_nums, start_number,
                                                           select)] }
                              : { content: this.normal_list(content, list,
                                                            use_nums, start_number,
                                                            select) };
    this.send(variables, msg);
  }

  public async send_files(variables: Variables, content: string, files: string[]) {
    const msg: MessageOptions = {
      content,
      files
    };

    this.send(variables, msg);
  }

  public send_embed(variables: Variables, embed: MessageEmbedOptions) {
    this.send(variables, {embeds: [embed]});
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

  private normal_list(content: string, list: string[],
                      use_nums: boolean = false, start_number: number = 1,
                      select?: number): string {
    if (use_nums) {
      list = list.map((element, index) => {return (index + start_number) + ") " + element});
    }

    if (select) {
      const select_symbol = "⮞";
      list = list.map((element, id) => {
        if (id === select - 1) {
          return bold(select_symbol + "  " + element);
        } else {
          return "     " + element;
        }
      });
    }

    return content.concat("\n" + list.join("\n"));
  }

  private embed_list(title: string, content: string, list: string[],
                     use_nums: boolean = false, start_number: number = 1,
                     select?: number): MessageEmbedOptions {
    const new_content = this.normal_list(content, list, use_nums,
                                         start_number, select);
    return this.basic_embed(title, new_content, this.colors.normal);
  }

  private create_button(customId: string, label: string,
                        style: Exclude<MessageButtonStyle, "LINK">,
                        disabled: boolean = false,
                        emoji?: string): MessageButton {
    const button_options: MessageButtonOptions = {
      style,
      customId,
      label,
      disabled,
      emoji
    }

    return new MessageButton(button_options);
  }
}

//-------------------------
// Basic Markdown functions
//-------------------------

export const bold = (message: string) => {
  return `**${message}**`;
}

export const italic = (message: string) => {
  return `*${message}*`;
}

export const bold_italic = (message: string) => {
  return `***${message}***`;
}

export const highlight = (message: string) => {
  return `\`${message}\``;
}

export const block = (message: string, block_type: string = "") => {
  return `\`\`\`${block_type}\n${message}\n\`\`\``;
}

export default Messager;
