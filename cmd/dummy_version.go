package cmd

import "github.com/spf13/cobra"

func init() {
	wrapperCmd.AddCommand(versionCmd)
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show the current version number of gibo and gibo-wrapper",
	RunE: func(cmd *cobra.Command, args []string) error {
		cmd.Println("gibo-wrapper v0.5.0")
		return wrapperCmd.RunE(cmd, append([]string{"version"}, args...))
	},
}
