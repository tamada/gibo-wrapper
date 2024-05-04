package cmd

import "testing"

func TestBasic(t *testing.T) {
	list, err := findRegisteredBoilerplates([]string{"../testdata"})
	if err != nil {
		t.Errorf(err.Error())
	}
	if len(list) != 3 {
		t.Errorf("expected 3, but %d", len(list))
	}
	expectedItems := []string{"macOS", "Windows", "Linux"}
	for i, expected := range expectedItems {
		if list[i] != expected {
			t.Errorf("expected %s, but %s", expected, list[i])
		}
	}
}

func TestBasic2(t *testing.T) {
	list, err := findBoilerplatesInGitignoreFile("../testdata/.gitignore")
	if err != nil {
		t.Errorf(err.Error())
	}
	if len(list) != 3 {
		t.Errorf("expected 3, but %d", len(list))
	}
	expectedItems := []string{"macOS", "Windows", "Linux"}
	for i, expected := range expectedItems {
		if list[i] != expected {
			t.Errorf("expected %s, but %s", expected, list[i])
		}
	}
}
