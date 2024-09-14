use crate::app::terminal::event::Event;
use tokio::sync::broadcast;

pub struct Ui {
  rx: broadcast::Receiver<Event>
}

impl Ui {
  pub fn new(rx: broadcast::Receiver<Event>) -> Self {
    Self {
      rx
    }
  }

  pub async fn render(&mut self) {
    let mut rotations = 0;
    while let Ok(e) = self.rx.try_recv() {
      println!("event occuried {e:?}");
    }
  }
}
