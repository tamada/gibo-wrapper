package cmd

import "github.com/spf13/cobra"

func init() {
	wrapperCmd.AddCommand(searchCmd)
}

var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Search for boilerplates",
	RunE: func(cmd *cobra.Command, args []string) error {
		return wrapperCmd.RunE(cmd, append([]string{"search"}, args...))
	},
}
