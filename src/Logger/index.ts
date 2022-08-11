import chalk from "chalk";

class Logger {
  public info(main_str: string, secondary_str?: string) {
    console.info(chalk.blue("[I]"), chalk.blue(main_str));
    if (secondary_str)
      this.secondary_info(secondary_str);
  }

  public log(main_str: string, secondary_str?: string) {
    console.log(chalk.green("[L]"), chalk.green(main_str));
    if (secondary_str)
      this.secondary_log(secondary_str);
  }

  public warn(main_str: string, secondary_str?: string) {
    console.warn(chalk.yellow("[W]"), chalk.yellow(main_str));
    if (secondary_str)
      this.secondary_warn(secondary_str);
  }

  public error(main_str: string, secondary_str?: string) {
    console.error(chalk.red("[E]"), chalk.red(main_str));
    if (secondary_str)
      this.secondary_error(secondary_str);
  }

  public secondary_info(secondary_str: string) {
    secondary_str = "\t".concat(secondary_str.replaceAll("\n", "\n\t"));
    console.info(secondary_str);
  }

  public secondary_log(secondary_str: string) {
    secondary_str = "\t".concat(secondary_str.replaceAll("\n", "\n\t"));
    console.log(secondary_str);
  }

  public secondary_warn(secondary_str: string) {
    secondary_str = "\t".concat(secondary_str.replaceAll("\n", "\n\t"));
    console.warn(secondary_str);
  }

  public secondary_error(secondary_str: string) {
    secondary_str = "\t".concat(secondary_str.replaceAll("\n", "\n\t"));
    console.error(secondary_str);
  }
}

export default Logger;
