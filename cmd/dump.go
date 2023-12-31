package cmd

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
)

func init() {
	wrapperCmd.AddCommand(dumpCmd)
	dumpCmd.Flags().BoolP("keep-prologue", "k", false, "keep the prologue of the .gitignore")
	dumpCmd.Flags().BoolP("remove-duplication", "r", false, "remove the duplicated boilerplate names")
}

var dumpCmd = &cobra.Command{
	Use:   "dump",
	Short: "Dump a boilerplate",
	Args:  cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		stdout := cmd.OutOrStdout()
		stderr := cmd.OutOrStderr()
		args4gibo, err := makeArgumentsForGibo(cmd, args)
		if err != nil {
			return err
		}
		return callGibo(append([]string{"dump"}, args4gibo...), stdout, stderr)
	},
}

func makeArgumentsForGibo(cmd *cobra.Command, args []string) ([]string, error) {
	ordinals, appends, removes := splitArgs(cmd.Flags().Args())
	if len(appends) > 0 { // append mode
		results, err := findBoilerplatesInGitignoreFile(filepath.Join(".", gitignoreFileName))
		if err != nil {
			return nil, err
		}
		ordinals = append(results, append(ordinals, appends...)...)
	}
	if len(removes) > 0 { // remove mode
		ordinals = removeSpecifiedBoilerplates(ordinals, removes)
	}
	results := removeDuplicationIfRequested(ordinals, cmd.Flags())
	err := dumpPrologue(cmd)
	if err != nil {
		return nil, err
	}
	return results, nil
}

func removeDuplicationIfRequested(names []string, flags *pflag.FlagSet) []string {
	removeDuplicationRequested, _ := flags.GetBool("remove-duplication")
	if !removeDuplicationRequested {
		return names
	}
	results := []string{}
	for _, name := range names {
		if !contains(results, name) {
			results = append(results, name)
		}
	}
	return results
}

func contains(names []string, name string) bool {
	for _, n := range names {
		if strings.ToLower(n) == name {
			return true
		}
	}
	return false
}

func dumpPrologue(cmd *cobra.Command) error {
	keepPrologueFlag, err := cmd.Flags().GetBool("keep-prologue")
	if err != nil {
		return err
	}
	if keepPrologueFlag {
		err := readAndWritePrologue(cmd)
		if err != nil {
			return err
		}
	}
	return nil
}

func readAndWritePrologue(cmd *cobra.Command) error {
	in, err := os.Open(gitignoreFileName)
	if err != nil {
		return err
	}
	defer in.Close()
	reader := bufio.NewReader(in)
	dest := cmd.OutOrStdout()
	for {
		lineByte, _, err := reader.ReadLine()
		line := string(lineByte)
		if err == io.EOF {
			break
		}
		if strings.HasPrefix(line, "###") {
			break
		}
		fmt.Fprintln(dest, line)
	}
	return nil
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
