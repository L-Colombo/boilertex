
use builtin;
use str;

set edit:completion:arg-completer[boilertex] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'boilertex'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'boilertex'= {
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand generate 'Generate boilerplate from a template'
            cand list 'list'
            cand preview 'preview'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'boilertex;generate'= {
            cand -g 'Also initialize a Git repository with a default gitignore'
            cand --git 'Also initialize a Git repository with a default gitignore'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'boilertex;list'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'boilertex;preview'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'boilertex;help'= {
            cand generate 'Generate boilerplate from a template'
            cand list 'list'
            cand preview 'preview'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'boilertex;help;generate'= {
        }
        &'boilertex;help;list'= {
        }
        &'boilertex;help;preview'= {
        }
        &'boilertex;help;help'= {
        }
    ]
    $completions[$command]
}
