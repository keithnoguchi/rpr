#![warn(missing_debug_implementations, rust_2018_idioms)]
pub mod checkers;

extern "C" {
    fn notify_piecemoved(fx: i32, fy: i32, tx: i32, ty: i32);
    fn notify_piececrowned(tx: i32, ty: i32);
}

lazy_static::lazy_static! {
    pub(crate) static ref GAME_ENGINE: mut_static::MutStatic<checkers::Engine> = {
        mut_static::MutStatic::from(checkers::Engine::new())
    };
}

#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    engine.current_turn.into()
}

#[no_mangle]
pub extern "C" fn get_move_count() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    engine.move_count as i32
}

#[no_mangle]
pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    match engine.piece(checkers::Coordinate::new(x as u8, y as u8)) {
        Some(p) => p.into(),
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();

    let mv = checkers::Move::new((fx as u8, fy as u8), (tx as u8, ty as u8));
    match engine.move_piece(mv) {
        Some(crowned) => {
            unsafe {
                notify_piecemoved(fx, fy, tx, ty);
            }
            if crowned {
                unsafe {
                    notify_piececrowned(tx, ty);
                }
            }
            1
        }
        None => 0,
    }
}
