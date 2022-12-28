(module
  (import "events" "piecemoved"
    (func $notify_piecemoved (param $fromX i32) (param $fromY i32)
      (param $toX i32) (param $toY i32) (param $piece i32))
  )
  (import "events" "piececrowned"
    (func $notify_piececrowned (param $x i32) (param $y i32))
  )
  (memory $mem 1)
  (global $currentTurn (mut i32) (i32.const 0))
  (global $BLACK i32 (i32.const 1))
  (global $WHITE i32 (i32.const 2))
  (global $CROWN i32 (i32.const 4))
  (export "initBoard" (func $initBoard))
  (export "move" (func $move))
  (export "getTurnOwner" (func $getTurnOwner))
  (export "getPiece" (func $getPiece))

  (func $initBoard
    (call $setPiece (i32.const 1) (i32.const 0) (global.get $WHITE))
    (call $setPiece (i32.const 3) (i32.const 0) (global.get $WHITE))
    (call $setPiece (i32.const 5) (i32.const 0) (global.get $WHITE))
    (call $setPiece (i32.const 7) (i32.const 0) (global.get $WHITE))
    (call $setPiece (i32.const 0) (i32.const 1) (global.get $WHITE))
    (call $setPiece (i32.const 2) (i32.const 1) (global.get $WHITE))
    (call $setPiece (i32.const 4) (i32.const 1) (global.get $WHITE))
    (call $setPiece (i32.const 6) (i32.const 1) (global.get $WHITE))
    (call $setPiece (i32.const 1) (i32.const 2) (global.get $WHITE))
    (call $setPiece (i32.const 3) (i32.const 2) (global.get $WHITE))
    (call $setPiece (i32.const 5) (i32.const 2) (global.get $WHITE))
    (call $setPiece (i32.const 7) (i32.const 2) (global.get $WHITE))
    (call $setPiece (i32.const 0) (i32.const 5) (global.get $BLACK))
    (call $setPiece (i32.const 2) (i32.const 5) (global.get $BLACK))
    (call $setPiece (i32.const 4) (i32.const 5) (global.get $BLACK))
    (call $setPiece (i32.const 6) (i32.const 5) (global.get $BLACK))
    (call $setPiece (i32.const 1) (i32.const 6) (global.get $BLACK))
    (call $setPiece (i32.const 3) (i32.const 6) (global.get $BLACK))
    (call $setPiece (i32.const 5) (i32.const 6) (global.get $BLACK))
    (call $setPiece (i32.const 7) (i32.const 6) (global.get $BLACK))
    (call $setPiece (i32.const 0) (i32.const 7) (global.get $BLACK))
    (call $setPiece (i32.const 2) (i32.const 7) (global.get $BLACK))
    (call $setPiece (i32.const 4) (i32.const 7) (global.get $BLACK))
    (call $setPiece (i32.const 6) (i32.const 7) (global.get $BLACK))
    ;; black first.
    (call $setTurnOwner (global.get $BLACK))
  )

  (func $move (param $fromX i32) (param $fromY i32)
              (param $toX i32) (param $toY i32) (result i32)
    (if (result i32)
      (block (result i32)
        (call $isValidMove (local.get $fromX) (local.get $fromY)
                           (local.get $toX) (local.get $toY))
      )
      (then
        (call $doMove (local.get $fromX) (local.get $fromY)
                      (local.get $toX) (local.get $toY))
      )
      (else
        (i32.const 0)
      )
    )
  )

  (func $doMove (param $fromX i32) (param $fromY i32)
                (param $toX i32) (param $toY i32) (result i32)
    (local $piece i32)
    (local.set $piece (call $getPiece (local.get $fromX) (local.get $fromY)))
    (call $toggleTurnOwner)
    (call $setPiece (local.get $toX) (local.get $toY) (local.get $piece))
    (call $setPiece (local.get $fromX) (local.get $fromY) (i32.const 0))
    (if (call $shouldCrown (local.get $toY) (local.get $piece))
      (then (call $crownPiece (local.get $toX) (local.get $toY)))
    )
    (call $notify_piecemoved (local.get $fromX) (local.get $fromY)
                             (local.get $toX) (local.get $toY) (local.get $piece))
    (i32.const 1)
  )

  (func $isValidMove (param $fromX i32) (param $fromY i32)
                     (param $toX i32) (param $toY i32) (result i32)
    (local $player i32)
    (local $target i32)
    (local.set $player (call $getPiece (local.get $fromX) (local.get $fromY)))
    (local.set $target (call $getPiece (local.get $toX) (local.get $toY)))
    (if (result i32)
      (block (result i32)
        (i32.and
          (call $validJumpDistance (local.get $fromY) (local.get $toY))
          (i32.and
            (call $isPlayersTurn (local.get $player))
            (i32.eq (local.get $target) (i32.const 0))
          )
        )
      )
      (then
        (i32.const 1)
      )
      (else
        (i32.const 0)
      )
    )
  )

  (func $validJumpDistance (param $from i32) (param $to i32) (result i32)
    (local $d i32)
    (local.set $d
      (if (result i32)
        (i32.gt_s (local.get $to) (local.get $from))
        (then
          (call $distance (local.get $to) (local.get $from))
        )
        (else
          (call $distance (local.get $from) (local.get $to))
        )
      )
    )
    (i32.le_u
      (local.get $d)
      (i32.const 2)
    )
  )

  (func $distance (param $x i32) (param $y i32) (result i32)
    (i32.sub (local.get $x) (local.get $y))
  )

  (func $crownPiece (param $x i32) (param $y i32)
    (local $piece i32)
    (local.set $piece (call $getPiece (local.get $x) (local.get $y)))
    (call $setPiece (local.get $x) (local.get $y)
      (call $withCrown (local.get $piece))
    )
    (call $notify_piececrowned (local.get $x) (local.get $y))
  )

  (func $shouldCrown (param $pieceY i32) (param $piece i32) (result i32)
    (i32.or
      (i32.and
        (i32.eq
          (local.get $pieceY)
          (i32.const 0)
        )
        (call $isBlack (local.get $piece))
      )
      (i32.and
        (i32.eq
          (local.get $pieceY)
          (i32.const 7)
        )
        (call $isWhite (local.get $piece))
      )
    )
  )

  (func $isPlayersTurn (param $player i32) (result i32)
    (i32.gt_s
      (i32.and (local.get $player) (call $getTurnOwner))
      (i32.const 0)
    )
  )

  (func $toggleTurnOwner
    (if (i32.eq (call $getTurnOwner) (global.get $BLACK))
      (then (call $setTurnOwner (global.get $WHITE)))
      (else (call $setTurnOwner (global.get $BLACK)))
    )
  )

  (func $setTurnOwner (param $turn i32)
    (global.set $currentTurn (local.get $turn))
  )

  (func $getTurnOwner (result i32)
    (global.get $currentTurn)
  )

  (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
    (i32.store
      (call $offsetForPosition (local.get $x) (local.get $y))
      (local.get $piece)
    )
  )

  (func $getPiece (param $x i32) (param $y i32) (result i32)
    (if (result i32)
      (block (result i32)
        (i32.and
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (local.get $x)
          )
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (local.get $y)
          )
        )
      )
      (then
        (i32.load
          (call $offsetForPosition
            (local.get $x)
            (local.get $y)
          )
        )
      )
      (else
        (unreachable)
      )
    )
  )

  (func $inRange (param $low i32) (param $high i32) (param $value i32) (result i32)
    (i32.and
      (i32.ge_s (local.get $value) (local.get $low))
      (i32.le_s (local.get $value) (local.get $high))
    )
  )

  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
    (i32.add
      (i32.mul
        (i32.const 8)
        (local.get $y)
      )
      (local.get $x)
    )
  )

  ;; offset = ( x + y * 8 ) * 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
    (i32.mul
      (call $indexForPosition (local.get $x) (local.get $y))
      (i32.const 4)
    )
  )

  (func $isBlack (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $BLACK))
      (global.get $BLACK)
    )
  )

  (func $isWhite (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $WHITE))
      (global.get $WHITE)
    )
  )

  (func $isCrowned (param $piece i32) (result i32)
    (i32.eq
      (i32.and (local.get $piece) (global.get $CROWN))
      (global.get $CROWN)
    )
  )

  (func $withCrown (param $piece i32) (result i32)
    (i32.or (local.get $piece) (global.get $CROWN))
  )

  (func $withoutCrown (param $piece i32) (result i32)
    (i32.xor (local.get $piece) (global.get $CROWN))
  )
)
