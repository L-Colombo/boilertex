
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'boilertex' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'boilertex'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'boilertex' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate boilerplate from a template')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List the templates in your config')
            [CompletionResult]::new('preview', 'preview', [CompletionResultType]::ParameterValue, 'Preview the output of some template')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'boilertex;generate' {
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'Also initialize a Git repository with a default gitignore')
            [CompletionResult]::new('--git', '--git', [CompletionResultType]::ParameterName, 'Also initialize a Git repository with a default gitignore')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'boilertex;list' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'boilertex;preview' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'boilertex;help' {
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate boilerplate from a template')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List the templates in your config')
            [CompletionResult]::new('preview', 'preview', [CompletionResultType]::ParameterValue, 'Preview the output of some template')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'boilertex;help;generate' {
            break
        }
        'boilertex;help;list' {
            break
        }
        'boilertex;help;preview' {
            break
        }
        'boilertex;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
