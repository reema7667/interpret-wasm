;; Adapted from LFD134x
(module
    (memory $mem 1)
    ;; X plays first
    (global $currentTurn (mut i32) (i32.const 1))
    (global $X i32 (i32.const 1))
    (global $Y i32 (i32.const 2))
    (global $EMPTY i32 (i32.const 0))

    ;; Linearize a 3x3 tic-tac-toe board
    (func $indexForPosition (param $row i32) (param $col i32) (result i32)
        local.get $col ;; wrong?
        local.get $row
        i32.const 3
        i32.mul
        i32.add
    )

    ;; Offset = ( index ) * 4
    (func $offsetForPosition (param $row i32) (param $col i32) (result i32)
        i32.const 4
        local.get $row
        local.get $col
        call $indexForPosition
        i32.mul
    )


    ;; Sets a piece in the board. No error checking is done here
    (func $setPiece (param $row i32) (param $col i32) (param $piece i32)
        local.get $col
        local.get $row
        call $offsetForPosition
        local.get $piece
        i32.store
    )

    ;; Places the current player's piece in the given location
    ;; advances to next player
    (func $takeTurn (param $row i32) (param $col i32)
        global.get $currentTurn
        local.get $col
        local.get $row
        call $setPiece
        call $advanceTurn
    )

    ;; Retrieves the value of the piece at a given position on
    ;; the board. No error checking done here.
    (func $getPiece (param $row i32) (param $col i32) (result i32)
        local.get $col
        local.get $row
        call $offsetForPosition
        i32.load
    )

    ;; Called to switch the current turn
    (func $advanceTurn
        block $if 
            block $else
                global.get $currentTurn
                global.get $X
                i32.eq
                br_if $else
                global.get $X
                global.set $currentTurn
                br $if
            end
            global.get $Y
            global.set $currentTurn
        end
    )

    (func $getCurrent (result i32)
        global.get $currentTurn
    )

    ;; Initializes the game board
    (func $initGame
        (local $r i32)
        (local $c i32)
        i32.const 0
        local.set $r 
        block 
            loop $iterate_row
                i32.const 0
                local.set $c
                block 
                    loop $iterate_col
                        global.get $EMPTY
                        local.get $c
                        local.get $r
                        call $setPiece

                        local.get $c
                        call $inc 
                        local.set $c 

                        local.get $c
                        i32.const 3
                        i32.eq  
                        br_if 1 

                        br 0
                    end
                end
                local.get $r
                call $inc
                local.set $r 

                local.get $r
                i32.const 3
                i32.eq
                br_if 1 
                br 0
            end
        end       
    )

    ;; Shortcut for adding 1
    (func $inc (param $a i32) (result i32)
        local.get $a
        i32.const 1
        i32.add
    )

    (export "initGame" (func $initGame))
    (export "getPiece" (func $getPiece))
    (export "currentTurn" (func $getCurrent))
    (export "takeTurn" (func $takeTurn))
    (export "memory" (memory $mem))
    (export "x" (global $X))
    (export "y" (global $Y))
)