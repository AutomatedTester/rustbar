use std::io::{stdout, stderr, Write};

//use term_utils;

enum Stream {
    Stdout,
    Stderr,
}

impl Default for Stream {
   fn default() -> Stream {
        Stream::Stdout
   } 
}

///all progressbars will implement it
pub trait ProgressBar<T> {
    fn new() -> T;
    fn to_stderr(mut self) -> T;
}


#[derive(Default)]
pub struct PercentageProgressBar {
    value: u8, //0..100
    msg:   String,
    stream: Stream,
}

impl ProgressBar<PercentageProgressBar> for PercentageProgressBar {

    fn new() -> PercentageProgressBar {
        PercentageProgressBar { ..Default::default()}
    }

    fn to_stderr(mut self) ->  PercentageProgressBar{
        self.stream = Stream::Stderr;
        self
    }
}


impl PercentageProgressBar {

    pub fn render(&self) {
        match self.stream {
            Stream::Stdout => {
                write!(stdout(), "\r{msg}{value}%", msg=self.msg, value=self.value);
                stdout().flush().ok().expect("write to stdout failed");
            },
            Stream::Stderr => {
                write!(stderr(), "\r{msg}{value}%", msg=self.msg, value=self.value);
                stderr().flush().ok().expect("write to stdout failed");
            },
        }
    }

    pub fn set_value(&mut self, value: u8) { if value <= 100 { self.value = value } }
    pub fn get_value(&self) -> u8 { self.value }

    pub fn set_msg(&mut self, msg: &str) { self.msg = msg.to_owned() }
    pub fn get_msg(&self) -> &str { self.msg.as_ref() }


    pub fn inc(&mut self) { if self.value < 100 { self.value += 1; } }
    pub fn dec(&mut self) { if self.value > 0 { self.value -= 1; } }

}

pub struct InfiniteProgressBar {
    msg:   String,
    marker_position:  i8,
    step: i8,
    stream: Stream,
}

impl Default for InfiniteProgressBar {
    fn default() -> InfiniteProgressBar {
        InfiniteProgressBar {
            step: 1,
            msg: "".to_owned(),
            marker_position: 0,
            stream: Stream::Stdout
        }
    }
}

impl ProgressBar<InfiniteProgressBar> for InfiniteProgressBar {

    fn new() -> InfiniteProgressBar {
        InfiniteProgressBar { ..Default::default()}
    }

    fn to_stderr(mut self) -> InfiniteProgressBar {
        self.stream = Stream::Stderr;
        self
    }

}

impl InfiniteProgressBar {

    pub fn set_msg(&mut self, msg: &str) { self.msg = msg.to_owned() }
    pub fn get_msg(&self) -> &str { self.msg.as_ref() }

    pub fn render(&mut self) {


        //let (screen_w, screen_h) = term_utils::get_winsize().unwrap();

        if self.marker_position <= 0 {
            self.marker_position = 0;
            self.step = 1;
        } else if self.marker_position > 9 {
            self.marker_position = 10;
            self.step = -1;
        }
        self.marker_position = self.marker_position + self.step;
        

        let mut bar:String = "..........".to_owned(); //10 dots
        bar.insert(self.marker_position as usize, '#');


        match self.stream {
            Stream::Stdout => {
                write!(stdout(), "\r{msg}[{bar}]", msg=self.msg, bar=bar);
                stdout().flush().ok().expect("write to stdout failed");
            },
            Stream::Stderr => {
                write!(stderr(), "\r{msg}[{bar}]%", msg=self.msg, bar=bar);
                stderr().flush().ok().expect("write to stdout failed");
            },
        }

    }
   
}
