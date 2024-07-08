package db

import (
	"sync"
	"time"
)

// Внутреннее представление шарда - внутри содержит hashmap и rw-mutex для синхронизации.
type shard struct {
	inner map[string]shardedValue
	rw    sync.RWMutex
}

type shardedValue struct {
	val            Value
	expirationTime *time.Time
}

func newShard() shard {
	return shard{
		inner: make(map[string]shardedValue),
		rw:    sync.RWMutex{},
	}
}

// Получить значение по ключу
func (s *shard) get(key string) Value {
	s.rw.RLock()
	val := s.inner[key]
	s.rw.RUnlock()

	// Проверяем, не вышло ли время у ключа
	if (val.expirationTime != nil) && (val.expirationTime.Before(time.Now())) {
		s.remove(key)
		return nil
	}

	return val.val
}

// Установить значение по ключу
func (s *shard) set(key string, val Value, expires *time.Duration) {
	var expirationTime *time.Time = nil

	if expires != nil {
		t := time.Now().Add(*expires)
		expirationTime = &t
	}

	s.rw.Lock()
	defer s.rw.Unlock()
	s.inner[key] = shardedValue{val, expirationTime}
}

// Удалить значение по ключу
func (s *shard) remove(key string) {
	s.rw.Lock()
	defer s.rw.Unlock()
	delete(s.inner, key)
}
