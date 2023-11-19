package cmd

import "testing"

func TestArgumentsForGibo(t *testing.T) {
	testdata := []struct {
		givenArgs []string
		wontArgs  []string
	}{
		{[]string{"macos", "linux", "windows"}, []string{"macos", "linux", "windows"}},                  // normal mode
		{[]string{"macos", "windows", "windows", "--remove-duplication"}, []string{"macos", "windows"}}, // remove duplication
		{[]string{"+windows", "linux", "macos"}, []string{"linux", "macos", "windows"}},                 // append mode

	}
	for _, td := range testdata {
		dumpCmd.ParseFlags(td.givenArgs)
		args, err := makeArgumentsForGibo(dumpCmd, td.givenArgs)
		if err != nil {
			t.Errorf(err.Error())
		}
		if len(args) != len(td.wontArgs) {
			t.Errorf("expected %d, but %d", len(td.wontArgs), len(args))
		}
		for i, arg := range args {
			if arg != td.wontArgs[i] {
				t.Errorf("expected %s, but %s", td.wontArgs[i], arg)
			}
		}
	}
}

func TestSplitArgs(t *testing.T) {
	testdata := []struct {
		givenArgs   []string
		wontOrdinal []string
		wontAppends []string
		wontRemoves []string
	}{
		{[]string{"macos", "+linux", "_windows"}, []string{"macos"}, []string{"linux"}, []string{"windows"}},
		{[]string{"macos", "linux", "windows"}, []string{"macos", "linux", "windows"}, []string{}, []string{}},
	}

	for _, td := range testdata {
		ordinals, appends, removes := splitArgs(td.givenArgs)
		if len(ordinals) != len(td.wontOrdinal) {
			t.Errorf("expected %d, but %d", len(td.wontOrdinal), len(ordinals))
		}
		if len(appends) != len(td.wontAppends) {
			t.Errorf("expected %d, but %d", len(td.wontAppends), len(appends))
		}
		if len(removes) != len(td.wontRemoves) {
			t.Errorf("expected %d, but %d", len(td.wontRemoves), len(removes))
		}
		for i, ordinal := range ordinals {
			if ordinal != td.wontOrdinal[i] {
				t.Errorf("expected %s, but %s", td.wontOrdinal[i], ordinal)
			}
		}
		for i, append := range appends {
			if append != td.wontAppends[i] {
				t.Errorf("expected %s, but %s", td.wontAppends[i], append)
			}
		}
		for i, remove := range removes {
			if remove != td.wontRemoves[i] {
				t.Errorf("expected %s, but %s", td.wontRemoves[i], remove)
			}
		}
	}
}
