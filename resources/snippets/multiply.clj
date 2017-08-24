(ns clj.exch
  (:require [permutation.kernel :refer [multiply]]))

(multiply {0 1, 1 0, 2 2} {0 0, 1 2, 2 1})

