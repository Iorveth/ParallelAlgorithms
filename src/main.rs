mod eating_philosophers;
mod readers_writers;
mod sleeping_dresser;
use eating_philosophers::eating_philosophers_run;
use readers_writers::readers_writers_run;
use readers_writers::Priority;
use sleeping_dresser::sleeping_dresser_run;
fn main() {
    //ReadersWriters
    //readers_writers_run(5, 5, Priority::Equal);
    //Eating Philosophers
    //eating_philosophers_run();
    //Sleeping Dresser
    sleeping_dresser_run(4, 20);
}
