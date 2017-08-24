(ns clj.exch
  (:require [permutation.naive :refer [elements-generated-by]]))

(def t { 0 1, 1 2, 2 3, 3 4, 4 5, 5 0 })
(def s { 0 2, 1 1, 2 0, 3 3, 4 4, 5 5 })

(elements-generated-by {0 1, 1 2, 2 0} {0 1, 1 0, 2 2})

