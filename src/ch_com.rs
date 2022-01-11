use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

struct async_channel_endpoint<T, A>
{
    sender : std::sync::mpsc::Sender<T>,
    reciever : std::sync::mpsc::Receiver<A>,

    

}
impl<T,A> async_channel_endpoint<T,A>
{
    pub fn new(tx : Sender<T>, rx : Receiver<A>) -> async_channel_endpoint<T,A>
    {
        async_channel_endpoint
        {
            sender : tx,
            reciever : rx
        }
    }
    pub fn Send(self ,tgt : T)
    {
        self.sender.send(tgt).unwrap()
    }
    pub fn Try_Recv(self) -> A
    {
        self.reciever.try_recv().unwrap()
    }
}
pub fn Create_Async_Channel<T,A>(one : T, two : A) -> (async_channel_endpoint<T,A> , async_channel_endpoint<A,T>)
{
    let (onetx,onerx) : (Sender<T> , Receiver<T>) = mpsc::channel();
    let (twotx, tworx) : (Sender<A> , Receiver<A>) = mpsc::channel();
    ()


}

pub enum Input_to_Physics
{


}