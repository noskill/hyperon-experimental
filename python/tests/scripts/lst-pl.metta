(= (lst1) (Cons a1 (Cons a2 Nil) ) )
(= (lst2) (Cons b1 Nil)  )
(= (lst3) (Cons a1 Nil) )

;append( [], X, X).                                 
;append( [X | Y], Z, [X | W]) :- append( Y, Z, W). 

(= (append Nil $x1 $x1) True)

(= (append (Cons $X $Y) $Z (Cons $X $W))
   (append $Y $Z $W)
)

(= (if True $then) $then)

;Ok
!(println! (if (append (lst3) (lst2) $L) $L))

; doesn't work
!(println! (if (append (lst1) (lst2) $L) $L))
