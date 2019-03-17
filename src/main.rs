mod eating_philosophers;
mod readers_writers;
use readers_writers::ReadersWriters;
use readers_writers::Priority;
use eating_philosophers::Philosopher;


fn main() {
    //ReadersWriters
    readers_writers::readers_writers_test(5,5,Priority::Readers);
    //Eating Philosophers
    //eating_philosophers::eating_philosophers_test();
}
