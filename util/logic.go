package util

// InSlice 当前给定的值是否在slice中
func InSlice[T comparable](slice []T, val T) (T, bool) {
	for _, v := range slice {
		if v == val {
			return val, true
		}
	}
	var zero T
	return zero, false
}

// MapTo 将传入的切片映射，返回另一种类型的切片
func MapTo[K, V any](slice []K, f func(k K) V) []V {
	var result []V
	for _, v := range slice {
		result = append(result, f(v))
	}
	return result
}

func First[T any](slice []T, f func(t T) bool) (T, bool) {
	for _, v := range slice {
		if f(v) {
			return v, true
		}
	}
	var zero T
	return zero, false
}
