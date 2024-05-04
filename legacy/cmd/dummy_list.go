package cmd

import "github.com/spf13/cobra"

func init() {
	wrapperCmd.AddCommand(listCmd)
}

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List available boilerplates",
	RunE: func(cmd *cobra.Command, args []string) error {
		return wrapperCmd.RunE(cmd, append([]string{"list"}, args...))
	},
}
