use dtee::Char;

#[test]
fn _0001() {
  let chr = Char::from('a');
  assert!(!chr.is_join());
  assert!(!chr.is_full_join());
}

#[test]
fn _0002() {
  let chr = Char::from('a');
  chr.set_full_join();
  chr.set_join();
  assert!(chr.is_join());
  assert!(!chr.is_full_join());
}

#[test]
fn _0003() {
  let chr = Char::from('a');
  chr.set_join();
  chr.set_full_join();
  assert!(chr.is_full_join());
  assert!(!chr.is_join());
}

#[test]
fn _0004() {
  let chr = Char::from('a');
  chr.set_join();
  chr.clear_join();
  assert!(!chr.is_join());
}

#[test]
fn _0005() {
  let chr = Char::from('a');
  chr.set_full_join();
  chr.clear_full_join();
  assert!(!chr.is_full_join());
}
