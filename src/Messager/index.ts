import { MessageEmbedOptions } from "discord.js"
import { Variables } from "../Interfaces";

class Messager {
  private use_embed: boolean = false;
  private colors = {
    sucsess: 0x00ff00,
    normal:  0x0000ff,
    error:   0xff0000,
  };

  public send_sucsess(variables: Variables, content: string, log_text?: string) {
    const title = "Sucsess";
    this.send_message(variables, title, content, this.colors.sucsess, log_text);
  }

  public send_normal(variables: Variables, title: string, content: string, log_text?: string) {
    this.send_message(variables, title, content, this.colors.normal, log_text);
  }

  public send_err(variables: Variables, content: string, log_text?: string) {
    const title = "Error";
    this.send_message(variables, title, content, this.colors.error, log_text);
  }

  public send_message(variables: Variables, title: string,
                      content: string, color: number,
                      log_text?: string) {
    const main = variables.type === "Old" ? variables.message : variables.interaction;
    const msg  = this.use_embed ? { embeds: [this.basic_embed(title, content, color)] }
                                : { content };

    if (variables.type === "New" && variables.interaction.replied) {
      variables.interaction.followUp(msg);
    } else {
      main.reply(msg);
    }

    if (log_text) {
      console.log(log_text);
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
}

export default Messager;
