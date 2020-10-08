package main

import (
	"fmt"
	"sync"
)

func main() {
	n := 100

	fmt.Println("channels result: ", DivisorsChannels(n))

	fmt.Println("pointer result: ", DivisorsPointers(n))

	fmt.Println("simple result: ", DivisorsSimple(n))
}

// ## 1 ##
func check1(i int, n int, c chan int, wg *sync.WaitGroup) {
	if n%i == 0 {
		c <- 1 // <- is the "send" operator when a channel appears on the left.
	}
	wg.Done()
}

func monitorWorker(wg *sync.WaitGroup, c chan int) {
	wg.Wait()
	close(c)
}

// DivisorsChannels - using channels to share goroutine results
func DivisorsChannels(n int) int {
	wg := &sync.WaitGroup{}
	c := make(chan int, 4)
	d := 1

	for i := 1; i <= n/2; i++ {
		wg.Add(1)
		go check1(i, n, c, wg)
	}

	go monitorWorker(wg, c)

	for o := range c {
		d = d + o
	}

	return d
}

// ## 2 ##
func check2(i int, n int, d *int, wg *sync.WaitGroup) {
	if n%i == 0 {
		*d = *d + 1
	}
	wg.Done()
}

// DivisorsPointers - using pointers to share goroutine results
func DivisorsPointers(n int) int {
	d := 1
	wg := &sync.WaitGroup{}

	for i := 1; i <= n/2; i++ {
		wg.Add(1)
		go check2(i, n, &d, wg)
	}

	wg.Wait()

	return d
}

// DivisorsSimple - simple loop
func DivisorsSimple(n int) int {
	d := 1

	for i := 1; i <= n/2; i++ {
		if n%i == 0 {
			d++
		}
	}

	return d
}
