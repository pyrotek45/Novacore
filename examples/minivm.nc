10 ${ 10 + 20 }

[ a b ]: $[

    @ main
        call test
        call testtwo
        exit
    @ testtwo
        out b
        ret
    @ test
        out a
        ret

]