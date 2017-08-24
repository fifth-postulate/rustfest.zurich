(ns clj.exch
  (:require [permutation.kernel :refer [multiply inverse]]))

(def a-level {:base 0
              :orbit [0 1 2 3 4 5]
              :transversal
              [
               {0 0, 1 1, 2 2, 3 3, 4 4, 5 5}
               {0 1, 1 2, 2 3, 3 4, 4 5, 5 0}
               {0 2, 1 1, 2 0, 3 3, 4 4, 5 5}
               {0 3, 1 2, 2 1, 3 4, 4 5, 5 0}
               {0 4, 1 3, 2 2, 3 5, 4 0, 5 1}
               {0 5, 1 4, 2 3, 3 0, 4 1, 5 2}]})

(def state {0 3, 1 0, 2 5, 3 4, 4 1, 5 2})

(a-level :base)
