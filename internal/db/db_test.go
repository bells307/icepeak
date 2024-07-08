package db

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestDatabaseGetSet(t *testing.T) {
	db := DefaultDataStorage()
	key := "key1"
	value := 1

	db.Set(key, value, nil)

	gotValue := db.Get(key)
	assert.Equal(t, value, gotValue)

	noValue := db.Get("key2")
	assert.Equal(t, nil, noValue)
}

func TestDatabaseRemove(t *testing.T) {
	db := DefaultDataStorage()
	key := "key1"
	value := 1

	db.Set(key, value, nil)

	gotValue := db.Get(key)
	assert.Equal(t, value, gotValue)

	db.Remove("key1")

	noValue := db.Get(key)
	assert.Equal(t, nil, noValue)
}

func TestDatabaseExpires(t *testing.T) {
	db := DefaultDataStorage()

	key := "key1"
	value := 1

	expires := time.Second
	db.Set(key, value, &expires)

	gotValue := db.Get(key)
	assert.Equal(t, value, gotValue)

	time.Sleep(expires)

	noValue := db.Get(key)
	assert.Equal(t, nil, noValue)
}
