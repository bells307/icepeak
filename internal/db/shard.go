package db

import (
	"context"
	"math/rand/v2"
	"sync"
	"time"
)

const (
	RANDOM_KEY_EXPIRE_CHECK_COUNT = 20
	EXPIRE_KEYS_THRESHOLD         = 0.2
	KEY_EXPIRE_TASK_PERIOD        = time.Second
)

// Внутреннее представление шарда - внутри содержит hashmap и rw-mutex для синхронизации.
type shard struct {
	// Хэшмап с данными
	inner map[string]shardedValue
	// Массив ключей, которые есть в хэшмапе (необходимо для рандомного взятия ключей и активной
	// очистки)
	keys []string
	rw   sync.RWMutex
}

type shardedValue struct {
	val            Value
	expirationTime *time.Time
	idx            int
}

func newShard(ctx context.Context) *shard {
	s := shard{
		inner: make(map[string]shardedValue),
		rw:    sync.RWMutex{},
	}

	go s.keyExpireTask(ctx)

	return &s
}

// Получить значение по ключу
func (s *shard) get(key string) Value {
	s.rw.RLock()
	val := s.inner[key]
	s.rw.RUnlock()

	// Проверяем, не вышло ли время у ключа
	if val.expired() {
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

	shardedVal, exists := s.inner[key]

	if !exists {
		s.keys = append(s.keys, key)
		// Если элемент уже добавлялся ранее, то в эту ветку он не попадет и сохранит прошлое значение индекса
		// Иначе присваиваем ему индекс только что созданного элемента
		shardedVal.idx = len(s.keys) - 1
	}

	s.inner[key] = shardedValue{val, expirationTime, shardedVal.idx}
}

// Удалить значение по ключу
func (s *shard) remove(key string) {
	s.rw.Lock()
	defer s.rw.Unlock()

	if val, exists := s.inner[key]; exists {
		s.keys = append(s.keys[:val.idx], s.keys[val.idx+1:]...)
		delete(s.inner, key)
	}
}

func (s *shard) keyExpireTask(ctx context.Context) {
	t := time.NewTimer(KEY_EXPIRE_TASK_PERIOD)

	for {
		select {
		case <-ctx.Done():
			t.Stop()
			return
		case <-t.C:
			for {
				expired := 0
				checkCount := RANDOM_KEY_EXPIRE_CHECK_COUNT

				if len(s.keys) < checkCount {
					checkCount = len(s.keys)
				}

				for range checkCount {
					if s.checkRandomKeyExpire() {
						expired += 1
					}
				}

				// Если процент истекших ключей больше порогового значения, то будем повторять очистку снова
				// Иначе, завершаем цикл
				if (float32(expired) / float32(RANDOM_KEY_EXPIRE_CHECK_COUNT)) < EXPIRE_KEYS_THRESHOLD {
					break
				}
			}
		}
	}
}

// Взять рандомный ключ из хешмапа (чтобы проверить его время истечения и удалить, если необходимо)
// Возвращает `true`, если время действия ключа истекло
func (s *shard) checkRandomKeyExpire() bool {
	keysLen := len(s.keys)

	if keysLen == 0 {
		return false
	}

	randIdx := rand.IntN(keysLen)
	randKey := s.keys[randIdx]

	s.rw.RLock()
	val := s.inner[randKey]
	s.rw.RUnlock()

	if val.expired() {
		s.remove(randKey)
		return true
	} else {
		return false
	}
}

func (v *shardedValue) expired() bool {
	if (v.expirationTime != nil) && (v.expirationTime.Before(time.Now())) {
		return true
	} else {
		return false
	}
}
