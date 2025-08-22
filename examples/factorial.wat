(module 
(func $fac (param i32) (result i32)
  block $if (result i32)
    block $else (result i32)
  		i32.const 1 
        local.get 0
        i32.const 2
        i32.lt_s
        br_if $if
  		  drop
        local.get 0
        local.get 0
        i32.const 1
        i32.sub
        call $fac
        i32.mul
        br $if
      end
    end  
  )
    (export "run" (func 0))
)