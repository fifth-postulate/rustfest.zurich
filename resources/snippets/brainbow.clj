(ns clj.exch
  (:require [permutation.bsgs :refer [bsgs-for sift solvable?]]))

(def t { 0 1, 1 2, 2 3, 3 4, 4 5, 5 0 })
(def s { 0 2, 1 1, 2 0, 3 3, 4 4, 5 5 })

(def brainbow (bsgs-for [t s]))

(def state {0 3, 1 0, 2 5, 3 4, 4 1, 5 2})

(sift brainbow state)
