//Developement feature.
//To be implemented
#[cfg(feature="development")]
struct Authentication{

}
//Developement feature.
//To be implemented [docs]
#[cfg(feature="development")]
trait BaseUserAuth<A,B>{
     fn auth(username:A, password:B)->Option<User<A,B>>;
}
//Developement feature.
//To be implemented
#[cfg(feature="development")]
struct User<A,B>{
     username:A,
     password:B
}
