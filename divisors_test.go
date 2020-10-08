package main

import (
	"testing"
)

/**
go test -benchmem -run=^$ -bench .

goos: darwin
goarch: amd64
BenchmarkDivisors_channels-12    	     607	   1874624 ns/op	    1314 B/op	       5 allocs/op
BenchmarkDivisors_pointers-12    	     655	   1863368 ns/op	     187 B/op	       2 allocs/op
BenchmarkDivisors_simple-12      	   20252	     61944 ns/op	       0 B/op	       0 allocs/op
PASS
ok  	_/Users/smichael1/Documents/projects/Codewars	4.831s

*/

func BenchmarkDivisors_channels(b *testing.B) {
	for n := 0; n < b.N; n++ {
		DivisorsChannels(10000)
	}
}

func TestDivisors_channels(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		n    int
		want int
	}{
		{"small", 1, 1},
		{"med", 100, 9},
		{"large", 10000, 25},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := DivisorsChannels(tt.n); got != tt.want {
				t.Errorf("DivisorsChannels() = %v, want %v", got, tt.want)
			}
		})
	}
}

func BenchmarkDivisors_pointers(b *testing.B) {
	for n := 0; n < b.N; n++ {
		DivisorsPointers(10000)
	}
}

func TestDivisors_pointers(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		n    int
		want int
	}{
		{"small", 1, 1},
		{"med", 100, 9},
		{"large", 10000, 25},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := DivisorsPointers(tt.n); got != tt.want {
				t.Errorf("DivisorsPointers() = %v, want %v", got, tt.want)
			}
		})
	}
}

func BenchmarkDivisors_simple(b *testing.B) {
	for n := 0; n < b.N; n++ {
		DivisorsSimple(10000)
	}
}

func TestDivisors_simple(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		n    int
		want int
	}{
		{"small", 1, 1},
		{"med", 100, 9},
		{"large", 10000, 25},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := DivisorsSimple(tt.n); got != tt.want {
				t.Errorf("DivisorsSimple() = %v, want %v", got, tt.want)
			}
		})
	}
}
