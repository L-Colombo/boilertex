const completion: Fig.Spec = {
  name: "boilertex",
  description: "Generate TeX project boilerplate",
  subcommands: [
    {
      name: ["generate", "g", "gen"],
      description: "Generate boilerplate from a template",
      options: [
        {
          name: ["-g", "--git"],
          description: "Also initialize a Git repository with a default gitignore",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "template",
      },
    },
    {
      name: ["list", "l"],
      description: "List the templates in your config",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: ["preview", "p"],
      description: "Preview the output of some template",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
      args: {
        name: "template",
      },
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "generate",
          description: "Generate boilerplate from a template",
        },
        {
          name: "list",
          description: "List the templates in your config",
        },
        {
          name: "preview",
          description: "Preview the output of some template",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help (see more with '--help')",
    },
    {
      name: ["-V", "--version"],
      description: "Print version",
    },
  ],
};

export default completion;
