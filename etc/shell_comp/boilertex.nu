module completions {

  # Generate TeX project boilerplate
  export extern boilertex [
    --help(-h)                # Print help (see more with '--help')
    --version(-V)             # Print version
  ]

  # Generate boilerplate from a template
  export extern "boilertex generate" [
    template: string          # The name of the template to be produced
    --git(-g)                 # Also initialize a Git repository with a default gitignore
    --help(-h)                # Print help
  ]

  export extern "boilertex list" [
    --help(-h)                # Print help
  ]

  export extern "boilertex preview" [
    template: string          # The name of the template to be previewed
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "boilertex help" [
  ]

  # Generate boilerplate from a template
  export extern "boilertex help generate" [
  ]

  export extern "boilertex help list" [
  ]

  export extern "boilertex help preview" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "boilertex help help" [
  ]

}

export use completions *
