package cmd

import (
	"io"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var wrapperCmd = &cobra.Command{
	Use:   "gibo-wrapper",
	Short: "gibo-wrapper is a wrapper for gibo to manage .gitignore file",
	Long: "gibo-wrapper acts like gibo and improves gibo by adding the following features\n" +
		"  1. list-ignore command for extracting the boilerplates in the .gitignore file\n" +
		"  2. dump command improvements\n" +
		"     * append mode: appending the boilerplates into the .gitignore file\n" +
		"     * remove mode: removing the boilerplates from the .gitignore file\n" +
		"     * keep-prologue option keeps the prologue in the .gitignore.\n" +
		"     * remove-duplication option removes the duplicated boilerplates names by dumping\n" +
		"  3. init commands for preparing the gibo and gibo-wrapper in your shell environment\n" +
		"     * write `eval $(gibo-wrapper init $SHELL)` in your shell configuration file\n",
	SilenceErrors: true,
	SilenceUsage:  true,
	Args:          cobra.ArbitraryArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		return callGibo(args, cmd.OutOrStdout(), cmd.ErrOrStderr())
	},
}

func callGibo(args []string, stdout, stderr io.Writer) error {
	newArgs := append([]string{"gibo"}, args...)
	cmd := exec.Command("command", newArgs...)
	cmd.Stdout = stdout
	cmd.Stderr = stderr
	return cmd.Run()
}

func Execute() error {
	wrapperCmd.SetOut(os.Stdout)
	wrapperCmd.CompletionOptions.DisableDefaultCmd = true
	return wrapperCmd.Execute()
}
