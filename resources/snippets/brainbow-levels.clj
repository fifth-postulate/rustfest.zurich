(ns clj.exch
  (:require [permutation.bsgs :refer [level]]))

(def t { 0 1, 1 2, 2 3, 3 4, 4 5, 5 0 })
(def s { 0 2, 1 1, 2 0, 3 3, 4 4, 5 5 })

(def u {0 0, 1 3, 2 2, 3 1, 4 4, 5 5})
(def v {0 0, 1 1, 2 4, 3 3, 4 2, 5 5})
(def w {0 0, 1 1, 2 2, 3 5, 4 4, 5 3})

(level [t s])
