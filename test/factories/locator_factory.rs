use rtps::common_types::*;
use factories::Create;

impl Create for Locator {
    fn create() -> Self {
        Locator::KIND_UDPv4(8000, [0,0,0,0, 0,0,0,0, 0,0,0,0, 127,0,0,1])
    }
}

impl<W> Create for Vec<W> where W: Create {
    fn create() -> Self {
        let mut list : Vec<W> = vec![];
        for _ in 0..5 {
            list.push(W::create())
        }
        list
    }
}