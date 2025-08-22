;; source code adapted from wasmtime
(module
  (func $gcd (param i32) (param i32) (result i32) (local i32) 
    block $y
        block $x
        local.get 0
        br_if 0 
        local.get 1
        local.set 2
        br 1
      end
      loop $l1
        local.get 1
        local.get 0
        local.tee 2
        i32.rem_u
        local.set 0
        local.get 2
        local.set 1
        local.get 0
        br_if 0 
      end
    end
    local.get 2
  )
  (export "run" (func 0))
)