package cmd

import (
	"os/exec"

	"github.com/spf13/cobra"
)

var wrapperCmd = &cobra.Command{
	Use:           "gibo-wrapper",
	Short:         "gibo-wrapper is a wrapper for gibo to manage .gitignore file",
	SilenceErrors: true,
	SilenceUsage:  true,
	Args:          cobra.ArbitraryArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		return callGibo(args, cmd)
	},
}

func callGibo(args []string, command *cobra.Command) error {
	newArgs := append([]string{"gibo"}, args...)
	cmd := exec.Command("command", newArgs...)
	cmd.Stdout = command.OutOrStdout()
	cmd.Stderr = command.ErrOrStderr()
	return cmd.Run()
}

func Execute() error {
	return wrapperCmd.Execute()
}
