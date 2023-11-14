package cmd

import (
	"log"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
)

func init() {
	wrapperCmd.AddCommand(dumpCmd)
}

var dumpCmd = &cobra.Command{
	Use:   "dump",
	Short: "Dump a boilerplate",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		newArgs := args
		if isAppendMode(args) {
			results, err := findBoilerplatesInGitignoreFile(filepath.Join(".", gitignoreFileName))
			if err != nil {
				log.Fatal(err.Error())
			}
			newArgs = concatNames(results, args)
		}
		callGibo(append([]string{"dump"}, newArgs...), cmd)
	},
}

func concatNames(results []string, args []string) []string {
	for _, arg := range args {
		if strings.HasPrefix(arg, "+") {
			results = append(results, arg[1:])
		} else {
			results = append(results, arg)
		}
	}
	return results
}

func isAppendMode(args []string) bool {
	for _, arg := range args {
		if strings.HasPrefix(arg, "+") {
			return true
		}
	}
	return false
}
