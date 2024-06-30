use crate::{from::{from_bytes, from_bytes_owned, from_bytes_static}, to::to_bytes};


struct TestingStruct
{
    counter:i32
}
impl TestingStruct
{
    pub fn increment(&mut self)->i32
    {
        self.counter+=1;
        self.counter
    }
    pub fn counter(&self)->i32
    {
        self.counter
    }
    /*pub fn counter_ref<'a>(&'a mut self)->&'a mut i32
    {
        &mut self.counter
    }*/
}
impl Default for TestingStruct
{
    fn default() -> Self {
        TestingStruct { counter: 0 }
    }
}
impl Clone for TestingStruct
{
    fn clone(&self) -> Self {
        Self
        {
            counter:self.counter
        }
    }
}
#[test]
fn simple_struct()->Result<(),()>
{
    let tester = TestingStruct::default();
    unsafe 
    {
        let mut tester_bytes = to_bytes(tester);

        let tester_ref = from_bytes::<TestingStruct>(&mut tester_bytes);
        tester_ref.increment();
        tester_ref.increment();
        tester_ref.increment();

        let tester = from_bytes_owned::<TestingStruct>(tester_bytes);
        assert!(tester.counter()==3);
    }
    Ok(())
}

#[test]
fn extended_lifetime()->Result<(), ()>
{
    let tester = TestingStruct::default();
    fn add_5(testing_struct:&'static mut TestingStruct)
    {
        for i in 0..5
        {
            testing_struct.increment();
        }
    }
    unsafe 
    {
        
        let mut tester_bytes = to_bytes(tester);
        
        add_5(from_bytes_static(&mut tester_bytes));

        let tester = from_bytes_owned::<TestingStruct>(tester_bytes);
        assert!(tester.counter()==5);
    }
    Ok(())
}