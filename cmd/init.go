package cmd

import (
	"bytes"
	"errors"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

func init() {
	wrapperCmd.AddCommand(initCmd)
}

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Prepare the gibo and gibo-wrapper in your shell environment",
	RunE:  doInit,
}

func doInit(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		return errors.New("init command requires the shell name")
	}
	shellName := filepath.Base(args[0])
	switch strings.ToLower(shellName) {
	case "sh", "bash", "zsh", "fish", "powershell":
		return initForShell(cmd, shellName)
	default:
		return fmt.Errorf("%s: invalid shell name", shellName)
	}
}

func initForShell(cmd *cobra.Command, shellName string) error {
	completion, err := generateCompletion(cmd, shellName)
	if err != nil {
		return err
	}
	cmd.Println(completion)
	cmd.Println("alias gibo='gibo-wrapper $@'")
	return nil
}

func generateCompletion(cmd *cobra.Command, shellName string) (string, error) {
	buffer := bytes.NewBuffer([]byte{})
	switch shellName {
	case "sh", "bash":
		if err := wrapperCmd.GenBashCompletionV2(buffer, true); err != nil {
			return "", err
		}
		fmt.Fprintln(buffer, `if [[ $(type -t compopt) = "builtin" ]]; then
    complete -o default -F __start_gibo-wrapper gibo
else
    complete -o default -o nospace -F __start_gibo-wrapper gibo
fi`)
	case "zsh":
		fmt.Fprintln(buffer, "#compdef _gibo")
		fmt.Fprintln(buffer, "compdef _gibo-wrapper gibo")
		if err := wrapperCmd.GenZshCompletion(buffer); err != nil {
			return "", err
		}
	case "fish":
		if err := wrapperCmd.GenFishCompletion(buffer, true); err != nil {
			return "", err
		}
		fmt.Fprintln(buffer, "complete -c gibo -n '__gibo_wrapper_clear_perform_completion_once_result'")
	case "powershell":
		if err := wrapperCmd.GenPowerShellCompletionWithDesc(buffer); err != nil {
			return "", err
		}
		fmt.Fprintln(buffer, "Register-ArgumentCompleter -CommandName 'gibo' -ScriptBlock ${__gibo_wrapperCompleterBlock}")
	}
	return buffer.String(), nil
}
