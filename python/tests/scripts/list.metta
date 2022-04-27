(= (lst1) (Cons a1 (Cons a2 Nil)))
(= (lst2) (Cons b1 (Cons b2 Nil)))


(= (Concat (Cons $head1 Nil) (Cons $head2 $tail2))
   (Cons $head1 (Cons $head2 $tail2)))

(= (Concat (Cons $head1 Nil) Nil)
   (Cons $head1 Nil))

(= (Concat (Cons $head1 (Cons $t1 $t11)) (Cons $head2 $tail2))
   (Cons $head1 (Concat (Cons $t1 $t11) (Cons $head2 $tail2))))

!(println! (Concat (lst1) (lst2)))

