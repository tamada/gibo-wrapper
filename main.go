package main

import (
	"fmt"
	"os"

	"github.com/tamada/gibo-wrapper/cmd"
)

func goMain(args []string) int {
	if err := cmd.Execute(); err != nil {
		fmt.Println(err.Error())
		return 1
	}
	return 0
}

func main() {
	status := goMain(os.Args[1:])
	os.Exit(status)
}
