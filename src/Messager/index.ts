import { ActionRowData, APIEmbed, ButtonComponentData, ButtonInteraction,
         ButtonStyle, Interaction, InteractionReplyOptions, Message,
         MessageComponentInteraction, MessageOptions, TextBasedChannel,
         ComponentType } from "discord.js"
import { Collection } from "typescript";
import { Variables } from "../Interfaces";
import Logger from "../Logger";

class Messager {
  private logger:    Logger = new Logger();
  private use_embed: boolean = false;
  private time_limit: number = 30 * 1000;
  private colors = {
    sucsess: 0x00ff00,
    normal:  0x0000ff,
    error:   0xff0000,
  };

  public async send_sucsess(variables: Variables, content: string): Promise<Message> {
    const title = "Sucsess";
    return this.send_message(variables, title, content, this.colors.sucsess);
  }

  public async send_normal(variables: Variables, title: string, content: string): Promise<Message> {
    return this.send_message(variables, title, content, this.colors.normal);
  }

  public async send_err(variables: Variables, content: string): Promise<Message> {
    const title = "Error";
    return this.send_message(variables, title, content, this.colors.error);
  }

  public async send_message(variables: Variables, title: string,
                      content: string, color: number): Promise<Message> {
    const msg: MessageOptions  = this.use_embed ? { embeds: [this.basic_embed(title, content, color)] }
                                                : { content };

    return this.send(variables, msg);
  }

  // TODO: find a way to not use (or better way to use) function pointers
  public async send_confirm(variables: Variables,
                      call_func: Function, func_this: any, func_params: any[],
                      additional_text?: string, end_text?: string): Promise<Message | undefined> {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const default_message = "Are you sure?";
    const row: ActionRowData<ButtonComponentData> = {
      type: ComponentType.ActionRow,
      components: [
        this.create_button("confirm_yes", "Yes", ButtonStyle.Success),
        this.create_button("confirm_no", "No", ButtonStyle.Danger)
      ]
    }
                               
    const msg: MessageOptions = {
      content: additional_text ? additional_text + " " + default_message
                               : default_message,
      components: [row]
    };

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collect_fun = (interaction: ButtonInteraction) => {
      if (interaction.customId === "confirm_yes") {
        call_func.apply(func_this, func_params);
      }
    };

    return this.handle_collector(variables, msg, channel, collect_fun, filter, 1, end_text);
  }

  public async send_selection(variables: Variables,
                              list: Array<{name: string, id: string, disabled: boolean}>,
                              call_func: Function, func_this: any,
                              title?: string, content?: string, end_text?: string): Promise<Message | undefined> {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const msg_content = content ? content : "Select one of them:";
    const msg_title = title ? title : "Select";
    const main_row: ActionRowData<ButtonComponentData> = {
      type: ComponentType.ActionRow,
      components: [
        ...list.map(({name, id, disabled}) => {
          return this.create_button(id, name, ButtonStyle.Primary, disabled);
        })
      ]
    }
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

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collect_fun = (interaction: ButtonInteraction) => {
      if (interaction.customId === "none") {
        return;
      } else if (interaction.customId === "all") {
        list.forEach(({id}) => {
          call_func.apply(func_this, [variables, id]);
        });
      } else {
        call_func.apply(func_this, [variables, interaction.customId]);
      }
    };
    
    return this.handle_collector(variables, msg, channel, collect_fun, filter, 1, end_text);
  }

  public async send_selection_from_list(variables: Variables,
                                        list: Array<{name: string, id: string, disabled: boolean}>,
                                        use_second_row: boolean,
                                        call_func: Function, func_this: any,
                                        title?: string, content?: string, end_text?: string): Promise<Message | undefined> {
    const channel = variables.type === "Old" ? variables.message.channel
                                             : variables.interaction.channel;
    if (!channel) {
      this.send_err(variables, "An error accured.");
      return;
    }

    const msg_content = content ? content : "Select one of them:";
    const msg_title = title ? title : "Select";
    const main_row: ActionRowData<ButtonComponentData> = {
      type: ComponentType.ActionRow,
      components: [
        ...list.map(({id, disabled}, index) => {
          return this.create_button(id, (index + 1).toString(), ButtonStyle.Primary, disabled);
        })
      ]
    }
    const secondary_row: ActionRowData<ButtonComponentData> = {
      type: ComponentType.ActionRow,
      components: [
        this.create_button("all", "All", ButtonStyle.Success),
        this.create_button("none", "None", ButtonStyle.Danger)
      ]
    }
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

    const filter = (interaction: MessageComponentInteraction) => {
      const user_id = variables.type === "Old" ? variables.message.member?.user.id
                                               : variables.interaction.user.id;
      return interaction.user.id === user_id;
    };

    const collect_fun = (interaction: ButtonInteraction) => {
      if (interaction.customId === "none") {
        return;
      } else if (interaction.customId === "all") {
        list.forEach(({id}) => {
          call_func.apply(func_this, [variables, id]);
        });
      } else {
        call_func.apply(func_this, [variables, interaction.customId]);
      }
    };

    return this.handle_collector(variables, msg, channel, collect_fun, filter, 1, end_text);
  }

  public async send_list(variables: Variables, title: string, content: string, list: string[],
                         use_nums: boolean = false, start_number: number = 1,
                         select?: number): Promise<Message> {
    const msg: MessageOptions = this.use_embed
                              ? { embeds: [this.embed_list(title, content, list,
                                                           use_nums, start_number,
                                                           select)] }
                              : { content: this.normal_list(content, list,
                                                            use_nums, start_number,
                                                            select) };
    return this.send(variables, msg);
  }

  public async send_files(variables: Variables, content: string, files: string[]): Promise<Message> {
    const msg: MessageOptions = {
      content,
      files
    };

    return this.send(variables, msg);
  }

  public async send_embed(variables: Variables, embed: APIEmbed): Promise<Message> {
    return this.send(variables, {embeds: [embed]});
  }

  private async send(variables: Variables, msg: MessageOptions): Promise<Message> {
    if (variables.type === "New") {
      if (variables.interaction.replied) {
        return await variables.interaction.followUp({...(msg as InteractionReplyOptions),
                                                     fetchReply: true}) as Message;
      } else {
        return await variables.interaction.reply({...(msg as InteractionReplyOptions),
                                                  fetchReply: true}) as Message;
      }
    } else {
      return await variables.message.reply(msg);
    }
  }

  private basic_embed(title: string, description: string, color: number): APIEmbed {
    const embed: APIEmbed = {
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
                     select?: number): APIEmbed {
    const new_content = this.normal_list(content, list, use_nums,
                                         start_number, select);
    return this.basic_embed(title, new_content, this.colors.normal);
  }

  private create_button(customId: string, label: string,
                        style: Exclude<ButtonStyle, ButtonStyle.Link>,
                        disabled: boolean = false,
                        emoji?: string): ButtonComponentData {
    const button_options: ButtonComponentData = {
      type: ComponentType.Button,
      style,
      customId,
      label,
      disabled,
      emoji
    }

    return button_options;
  }

  private async handle_collector(variables: Variables,
                                 message: MessageOptions, channel: TextBasedChannel,
                                 collector_func: ((interaction: Interaction) => void)
                                               | ((interction: ButtonInteraction) => void),
                                 filter?: (interaction: MessageComponentInteraction) => boolean,
                                 max_interaction?: number,
                                 end_text?: string,
                                 custom_end_func?: (collection: Collection<Interaction>, reason: string) => void
                                ): Promise<Message> {
    const sent_msg = await this.send(variables, message);

    const collector = channel.createMessageComponentCollector({
      filter,
      max: max_interaction,
      time: this.time_limit
    });

    collector.on("collect", collector_func);

    collector.on("end", custom_end_func || ((_collection, reason) => {
      if (reason === "time") {
        sent_msg.edit({
          content: "Interaction timed out.",
          components: []
        });
      } else if (reason === "limit") {
        sent_msg.edit({
          content: end_text || "An action has already been taken.",
          components: []
        });
      } else {
        this.logger.warn("Unknown Collector End Reason", "Reason is: " + reason);
      }
    }));

    return sent_msg;
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
