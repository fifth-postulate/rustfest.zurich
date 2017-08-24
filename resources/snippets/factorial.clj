(defn factorial [n]
  (reduce * 1 (map inc (range n))))

(map factorial (range 10))
