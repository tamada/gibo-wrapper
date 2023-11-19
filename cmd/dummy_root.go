package cmd

import "github.com/spf13/cobra"

func init() {
	wrapperCmd.AddCommand(rootCmd)
}

var rootCmd = &cobra.Command{
	Use:   "root",
	Short: "Show the directory where gibo stores its boilerplates",
	RunE: func(cmd *cobra.Command, args []string) error {
		return wrapperCmd.RunE(cmd, append([]string{"root"}, args...))
	},
}
