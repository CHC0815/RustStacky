: fizz? 3 mod 0 = dup if "Fizz" puts then ;
: buzz? 5 mod 0 = dup if "Buzz" puts then ;
: fizz-buzz dup fizz? swap buzz? or ;
: loop-fb 25 1 do i dup . fizz-buzz loop ;


: helper? if . then ;
: loop-test 25 1 do i dup fizz? swap buzz? i or helper? loop ;

loop-test