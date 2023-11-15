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
	RunE: func(cmd *cobra.Command, args []string) error {
		ordinals, appends, removes := splitArgs(cmd.Flags().Args())
		if len(appends) > 0 { // append mode
			results, err := findBoilerplatesInGitignoreFile(filepath.Join(".", gitignoreFileName))
			if err != nil {
				log.Fatal(err.Error())
			}
			ordinals = append(results, append(ordinals, appends...)...)
		}
		if len(removes) > 0 { // remove mode
			ordinals = removeSpecifiedBoilerplates(ordinals, removes)
		}
		return callGibo(append([]string{"dump"}, args...), cmd)
	},
}

func splitArgs(args []string) ([]string, []string, []string) {
	ordinals, appends, removes := []string{}, []string{}, []string{}
	for _, arg := range args {
		if strings.HasPrefix(arg, "_") {
			removes = append(removes, arg[1:])
		} else if strings.HasPrefix(arg, "+") {
			appends = append(appends, arg[1:])
		} else {
			ordinals = append(ordinals, arg)
		}
	}
	return ordinals, appends, removes
}

func removeBoilerplate(names []string, name string) []string {
	results := []string{}
	for _, n := range names {
		if n != name {
			results = append(results, n)
		}
	}
	return results
}

func removeSpecifiedBoilerplates(boilerplateNames []string, givenArgs []string) []string {
	for _, arg := range givenArgs {
		boilerplateNames = removeBoilerplate(boilerplateNames, arg)
	}
	return boilerplateNames
}
