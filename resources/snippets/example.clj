(defn sum
  ([xs] (sum 0 xs))
  ([acc xs]
   (if (empty? xs)
     acc
     (sum (+ acc (first xs)) (rest xs)))))

(sum [1 2 3 4])

