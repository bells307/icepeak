package db

import (
	"runtime"
	"time"
)

// Хранилище данных, реализованное в виде шардированных хэшмапов.
//
// Каждый ключ имеет уникальный хэш, соответствующий только определенному шарду. Поэтому,
// мы всегда можем быстро вычислить, на каком шарде должно находиться значение,
// соответствующее ключу
type DataStorage struct {
	shards []shard
}

type Value interface{}

func NewDataStorage(shard_count uint) DataStorage {
	shards := make([]shard, shard_count)
	for i := range shard_count {
		shards[i] = newShard()
	}

	return DataStorage{shards}
}

func DefaultDataStorage() DataStorage {
	return NewDataStorage(uint(runtime.NumCPU()))
}

// Получить значение по ключу
func (s *DataStorage) Get(key string) Value {
	return s.getShard(key).get(key)
}

// Установить значение по ключу
func (s *DataStorage) Set(key string, value Value, expires *time.Duration) {
	s.getShard(key).set(key, value, expires)
}

// Удалить значение по ключу
func (s *DataStorage) Remove(key string) {
	s.getShard(key).remove(key)
}

// Получение шарда по ключу
func (s *DataStorage) getShard(key string) *shard {
	hash := djb2(key)
	idx := hash % uint32(len(s.shards))
	return &s.shards[idx]
}

// Вычисление хэша строки
func djb2(input string) uint32 {
	var x, r uint32 = 33, 5381

	for _, c := range input {
		r = r*x + uint32(c)
		r >>= 0
	}

	return r
}
