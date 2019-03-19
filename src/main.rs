mod eating_philosophers;
mod readers_writers;
mod sleeping_dresser;
use readers_writers::readers_writers_test;
use readers_writers::Priority;
use eating_philosophers::eating_philosophers_test;
use sleeping_dresser::sleeping_dresser_test;
fn main() {
    //ReadersWriters
    //readers_writers_test(5, 5, Priority::Equal);
    //Eating Philosophers
    //eating_philosophers_test();
    //Sleeping Dresser
    sleeping_dresser_test(4, 20);
}
