main:
  LD V2 0x0F
  LD F v0
  JP draw
undraw:
  DRW V8 V8 5
  ADD V0 1
  AND V0 V2
  LD F v0
draw:
  DRW V8 V8 5
  LD V1 60
  LD DT V1
dt_check:
  LD V1 DT
  SNE V1 0
  JP undraw
  JP dt_check