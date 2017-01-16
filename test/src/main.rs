#[macro_use]
extern crate objc_methname;

macro_rules! intern {
    ($id:ident) => {
        {
            #[allow(dead_code, bad_style)]
            #[derive(__objc_methname)]
            struct $id;
            $id::get()
        }
    };
    ($($id:ident :)+) => {
        {
            #[allow(dead_code, bad_style)]
            #[derive(__objc_methname)]
            struct Dummy {
                $($id : u32,)+
            }
            Dummy::get()
        }
    };
}

fn main() {
    assert_eq!(intern!(Rustling), intern!(Rustling));
    assert_eq!(intern!(Apples:pears:), intern!(Apples:pears:));
}
