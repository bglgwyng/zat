package main

import "fmt"

func Greet(name string) string {
	return fmt.Sprintf("Hello, %s!", name)
}

func Add(a, b int) int {
	return a + b
}

type Config struct {
	Name  string
	Value int
}

type Reader interface {
	Read(p []byte) (n int, err error)
	Close() error
}

func (c *Config) String() string {
	return c.Name
}
