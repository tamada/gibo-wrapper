package cmd

import "github.com/spf13/cobra"

const VERSION = "0.5.2"

func init() {
	wrapperCmd.AddCommand(versionCmd)
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show the current version number of gibo and gibo-wrapper",
	RunE: func(cmd *cobra.Command, args []string) error {
		cmd.Printf("gibo-wrapper v%s\n", VERSION)
		return wrapperCmd.RunE(cmd, append([]string{"version"}, args...))
	},
}
