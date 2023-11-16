package cmd

import "github.com/spf13/cobra"

func init() {
	wrapperCmd.AddCommand(updateCmd)
}

var updateCmd = &cobra.Command{
	Use:   "update",
	Short: "Update the gitignore boilerplate repository",
	RunE: func(cmd *cobra.Command, args []string) error {
		return wrapperCmd.RunE(cmd, append([]string{"update"}, args...))
	},
}
