use crate::{
    prelude::*,
    model::{TimeoutType, ScreenId, PopupId},
};
use crossterm::event::{Event as CrosstermEvent};

const SENDER_ERR: &'static str = "could not send terminal event";

/// Terminal events
#[derive(Debug)]
pub enum Event {
    CrosstermKey(KeyEvent),

    Quit,
    EnterScreen(ScreenId),
    EnterPopup(Option<PopupId>),

    SwitchDiv,
    ToggleDisplayMsg,
    Resize,
    TimeoutTick(TimeoutType),
    KeyInput(KeyEvent),
    SwitchInput,
    NextInput,
    PrevInput,
    SwitchAction,
    SelectAction,

    //NextListItem(ListType),
    //PrevListItem(ListType),
    //SelectListItem(ListType),

    //NextTableItem(TableType),
    //PrevTableItem(TableType),
    //SelectTableItem(TableType),

    TryLogin,

    TryGetUserLocker(String, String),
    TryGetUserBranch(String, String),
    TryGetUserDelivery(String),
    PlaceOrderReq,

    UpdatePaymentInfo,
    RejectOrderReq,
    PlaceOrder
}

#[derive(Debug)]
pub struct EventHandler {
    // Event sender channel
    pub tx: mpsc::Sender<Event>,
    // Event receiver channel
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    // Constructs a new instance of [`EventHandler`]
    pub fn new(tick_step: u16) -> Self {
        let tick_rate = Duration::from_millis(tick_step as u64);
        let (tx, rx) = mpsc::channel();

        {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    // Poll for terminal input
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(Duration::from_secs(0));

                    if event::poll(timeout).expect("poll failed") {
                        if let Ok(CrosstermEvent::Key(key)) = event::read() {
                            tx.send(Event::CrosstermKey(key)).ok();
                        }
                    }

                    // Emit a generic tick for the App to handle
                    if last_tick.elapsed() >= tick_rate {
                        //sender.send(Event::Tick).ok();
                        last_tick = Instant::now();
                    }
                }
            });
        }

        Self { tx, rx }
    }

    /// Receive the next event from the handler thread
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent
    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}

/*
TODO (Maybe):
    CrosstermEvent::Resize(_, _) => {
        //let mut app_lock = app.lock().unwrap();
        /*if !app_lock.timeout.contains_key(&TimeoutType::Resize) {
            app_lock.add_timeout(1, 250, TimeoutType::Resize);
            sender.send(Event::Resize).expect(SENDER_ERR);
        }*/
    },
*/