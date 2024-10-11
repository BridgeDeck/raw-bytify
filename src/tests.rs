use crate::{
    from::*, 
    to::*
};


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
        {
            let tester_ref = from_bytes::<TestingStruct>(&mut tester_bytes);
            tester_ref.increment();
            tester_ref.increment();
            tester_ref.increment();
        }    
        let tester:TestingStruct = from_bytes::<TestingStruct>(&mut tester_bytes).clone();
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

        let tester = from_bytes_static::<TestingStruct>(&mut tester_bytes).clone();

        assert!(tester.counter()==5);
    }
    Ok(())
}

// #[test]
// fn modify_bytes_directly()->Result<(),()>
// {
//     const MODIFY_TO:i32 = 257;
//     let tester = TestingStruct::default();
//     unsafe{

//         let mut tester_bytes = to_bytes(tester);
//         tester_bytes.remove(2);
//         tester_bytes.insert(2, 1);
//         tester_bytes.remove(3);
//         tester_bytes.insert(3, 1);
//         let tester = from_bytes::<TestingStruct>(&mut tester_bytes);
//         assert!(tester.counter()==257);
//     }


//     Ok(())
// }