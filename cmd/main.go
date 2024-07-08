package main

import (
	"context"
	"fmt"
	"time"

	"github.com/bells307/icepeak/internal/db"
)

func main() {
	db := db.DefaultDataStorage(context.Background())

	key := "key1"
	value := 1

	expires := time.Millisecond * 100
	db.Set(key, value, &expires)

	// value = db.Get(key)
	//
	// fmt.Println(value)

	// time.Sleep(time.Second * 3)

	noValue := db.Get(key)

	fmt.Println(noValue)
}
