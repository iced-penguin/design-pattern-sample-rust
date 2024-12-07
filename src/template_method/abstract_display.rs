pub trait AbstractDisplay {
    fn open(&self);
    fn print(&self);
    fn close(&self);
    fn display(&self) {
        self.open();
        for _ in 0..5 {
            self.print();
        }
        self.close();
    }
}
