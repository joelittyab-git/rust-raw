

----------------------------------------------------------Server-Structure------------------------------------------------
```.......................................................Client::Receiver...............................................
/// +-----------------------+   
/// |      Client  (rx)     |   
/// +-----------+-----------+   
///             |              
///             V             
/// +-----------------------+       +---------+------------+           
/// |        Server         | ----> |    Thread Creation   |  ----->   await_for_data()
/// +-----------+-----------+       +---------+------------+           
///
///.....................................................Client::Sender..................................................
/// +-----------------------+   
/// |      Client  (sn)     |   
/// +-----------+-----------+   
///             |              
///             V             
/// +-----------------------+       +---------+------------+           +---------------Pool-----------------+
/// |        Server         | ----> |    Thread Creation   |  ----->   | get_sender_for(username)->Sender   |
/// +-----------+-----------+       +---------+------------+           +------------------------------------+
///             |                                                                         |    
///             |                                                                         |
///             |                                                                         V
///             |                                                              +-----------------------+   
///             |                                                              |      Sender.send()    |   
///             |                                                              +-----------+-----------+
///         _________
///         |       |
///         |Receive|
///         |_______| 
/// ```

--------------------------------------------------------------------------------------------------------------------------



----------------------------------------------------------Protocol-Structure------------------------------------------------
I. SEND/RECEIVE
     - The client initializes a handshake by specifying the client type to the server

     /*Format-----------------------
     <type(SEND;<to-username>/RECEIVE;<self-usrname>)>
      ------------------------------*/


II. Data transfer
     - After the server identifies the client type, a thread is initialized to handle the tcpstream
     - Data is sent to the server by the client using the BaseProtocol
     /*Format-----------------------
     <alias>-<to>(/n)
     <body>
      ------------------------------*/

III. Responses
      - After the client sends to_alias the server sends a response 
      - Types:
            1. Success
            2. InvalidIdentifier
            3. ServerError

      /*Format-----------
      <Status>;<Message>
      ------------------*/
---------------------------------------------------------------------------------------------------------------------------


feat – a new feature is introduced with the changes
fix – a bug fix has occurred
chore – changes that do not relate to a fix or feature and don't modify src or test files (for example updating dependencies)
refactor – refactored code that neither fixes a bug nor adds a feature
docs – updates to documentation such as a the README or other markdown files
style – changes that do not affect the meaning of the code, likely related to code formatting such as white-space, missing semi-colons, and so on.
test – including new or correcting previous tests
perf – performance improvements
eradicate - removed a feature