use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

use crate::{
    models::{GameEvent, Sprite},
    threadpool::ThreadPool,
};

pub trait GameListener {
    fn get_events(&self) -> Vec<GameEvent>;
}

pub trait GameApi: Send + Sync {
    fn get_sprite(&self, sender: mpsc::Sender<Sprite>);
}

pub trait GameRenderer: Send + Sync {
    fn render(&self, sprite: Sprite);
}

pub struct Game {
    pub listener: Rc<dyn GameListener>,
    pub api: Arc<dyn GameApi>,
    pub pool: ThreadPool,
    pub renderer: Arc<dyn GameRenderer>,
    pub sender: mpsc::Sender<Sprite>,
    pub receiver: Arc<Mutex<mpsc::Receiver<Sprite>>>,
}

impl Game {
    pub fn new(
        listener: Rc<dyn GameListener>,
        api: Arc<dyn GameApi>,
        renderer: Arc<dyn GameRenderer>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let pool = ThreadPool::new(20);
        Self {
            listener,
            api,
            pool,
            renderer,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn tick(&self) {
        for _ in self.listener.get_events() {
            let api = self.api.clone();
            let sender = self.sender.clone();
            self.pool.execute(move || {
                api.get_sprite(sender);
            });
        }
    }

    pub fn render(&self) {
        for sprite in self.receiver.lock().unwrap().try_iter() {
            self.renderer.render(sprite);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct MockGameListener;

    impl GameListener for MockGameListener {
        fn get_events(&self) -> Vec<GameEvent> {
            vec![
                GameEvent::Up,
                GameEvent::Left,
                GameEvent::Down,
                GameEvent::Space,
            ]
        }
    }

    struct MockApi;

    impl GameApi for MockApi {
        fn get_sprite(&self, sender: mpsc::Sender<Sprite>) {
            let sprite = Sprite::new(0.0, 0.0, 10.0, 10.0, &[255, 255, 255]);
            sender.send(sprite).unwrap();
        }
    }

    struct MockRenderer {
        sprite: Mutex<Option<Sprite>>,
    }

    impl MockRenderer {
        fn new() -> Self {
            Self {
                sprite: Mutex::new(None),
            }
        }
    }

    impl GameRenderer for MockRenderer {
        fn render(&self, sprite: Sprite) {
            self.sprite.lock().unwrap().replace(sprite);
        }
    }

    #[test]
    fn test_game_tick() {
        let listener = Rc::new(MockGameListener);
        let api = Arc::new(MockApi);
        let renderer = Arc::new(MockRenderer::new());
        let game = Game::new(listener, api, renderer.clone());

        game.tick();
        std::thread::sleep(std::time::Duration::from_millis(10));
        game.render();

        assert!(renderer.sprite.lock().unwrap().is_some());
    }
}
