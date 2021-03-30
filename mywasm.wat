(module
  (func $callback (import "" "callback"))
  (func (export "startup") (call $callback))
)