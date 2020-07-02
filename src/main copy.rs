#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use std::env;
use std::net::{UdpSocket, TcpListener, TcpStream};
use std::time::Duration;
use std::thread;
use std::io::{Read, Write};
use std::marker::Copy;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};


// mod tools;
pub mod db_tools;
pub mod xml_tools;

use xml_tools::*;
// use db_tools::*;
// use tools::*;

struct Global_vars{
    broadcast_addr:  String,
    shutdownEmuPackage: String,
    infoPackage: String
}

fn initSocket(addr_sock: String) -> std::net::UdpSocket {
    match UdpSocket::bind(addr_sock) {
        Ok(sock) => return sock,
        Err(e) => panic!("ERROR: {:?}",e) 
    };
} 

fn initDevice(maplinked: &Maplinked) -> Vec<StateItems> {
    
    let size_vec_items = maplinked.items.len();
    let mut list_state_items = Vec::new();

    // бежим по длинне списка маплинкед, заполняем затычками струтуру
    // где будем хранить значения параметров
    for ind in 0..size_vec_items {
        // новый элемент структуры
        let mut stateItem = StateItems { 
            varname: &maplinked.items[ind].varname, // берем имя параметра из maplinked
            value: StateVal::Unit // кладем временно нулевое значение
         };
        list_state_items.push( stateItem )
    }
    return list_state_items
}

// fn changeParam<T: ToString>(varname: String, value: T) {

// }

fn cli_interface(list_state: &Vec<StateItems>, conn: &rusqlite::Connection) -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let size = args.len();
    
    if size > 1 {
        let command: &str = &*args[1];
        match command {
            "update" => {
                if size == 4 {
                    let varname_inp = &args[2];
                    let value_inp = &args[3];
                    for state in list_state {
                        if state.varname == varname_inp {
                            crate::db_tools::update_value(&state.varname , value_inp, &conn);
                            return Ok("1".to_string());
                        }
                    }
                }
                else{
                    return Err("Слишком мало аргументов
                            пример: update IN_CP_LUM 20".to_string());
                }
            },
            "uuid" => {
                if size == 3 {
                    let uuid = &args[2];
                    return Ok(uuid.to_string());
                }
            }
            _ => return Err("Комманда не найдена".to_string())
        }
    }
    Ok("-1".to_string())
}

fn startInfoListener(receiver: Receiver<bool>, sender: Sender<bool>, infoPackage: String) -> std::io::Result<()> {
    


    println!("dddd");
    // let mut socket = match UdpSocket::bind("10.7.2.2:5555") {
    //     Ok(sock) => sock,
    //     Err(e) => panic!(e) 
    // };
    let socket = initSocket("10.7.2.2:5555".to_string());
    
    let broadcast_addr = "10.7.255.255:19000";
    // включаем поддержку отправки на броадкаст
    socket.set_broadcast(true).expect("set_broadcast call failed");
    
    
    loop{
        // let msg_test = infoPackage;
        socket.send_to(infoPackage.as_bytes(), broadcast_addr)?;
        
        thread::sleep(Duration::from_millis(4000));
        match receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                match sender.send(true){
                    Ok(_) => {},
                    Err(e) => panic!("ERROR: {:?}",e)     
                }
                return Ok(());
            },
            Err(TryRecvError::Empty) => {}
        }
    }
}

fn create_root_fault(code: &str, detail: &str) -> String{
    format!( "
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
        <root>\
        <fault>\
        <code>{}</code>\
        <detail>{}</detail>\
        </fault>\
        </root>\r\n", code, detail)
}

fn parserPackage(pack: String, getAnswer: &String) -> String{



    if pack.contains("<get>"){
        match crate::xml_tools::xmlPackToStruct::<Get>(pack.to_string()) {
            Ok(_) => {
                return getAnswer.to_string();
            }
            Err(e) => {
                // return "";
                panic!("ERROR PARSING: {:?}",e)
            }
        }
    }
    else{
        return create_root_fault("1", "Не получается определить тип пакета");
    }
}

fn handle_tcp_client(mut client: TcpStream, getAnswer: String) {
    let mut buffer = vec![0u8; 1000];
    while match client.read(&mut buffer) {
        Ok(size) => {
            // echo everything!
            // println!("data: {:?}", );
            let stringPack = (std::str::from_utf8(&buffer[0..size-1]).unwrap()).to_string();
            let packForSend: String = parserPackage(stringPack, &getAnswer);
            client.write(&packForSend[..].as_bytes()).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", client.peer_addr().unwrap());
            false
        }
    } {}
}

fn listenerPackage(receiver: Receiver<bool>, listAnswer: SetAnswers) -> std::io::Result<()> {
    
    let listener = TcpListener::bind("10.7.2.2:19002")?;
    // let mut buffer: Vec<u8> = Vec::with_capacity(1000);
    let get_ = listAnswer.get.clone();
    let mut buffer = vec![0u8; 1000];
    loop{

        // let (numb, src_addr) = socket.recv_from(&mut buffer).unwrap();
        for client in listener.incoming(){
            match client {
                Ok(client) => {
                    thread::spawn( || {handle_tcp_client(client, get_)} );
                    return Ok(());
                },
                Err(e) => println!("Error accept connect with client \n Error: {:?}", e)
            }
        }
        // let result_data = std::str::from_utf8(&buffer[0..numb-1]).unwrap();
        // println!("address: {:?} \n data: \n {:?}\nbytes: {:?}", src_addr, result_data, numb);
        
        // чекаем сообщение о завершении из потока "ctrl + c"
        match receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating listener package");
                return Ok(());
            },
            Err(TryRecvError::Empty) => {}
        }
    }
}   

struct SetAnswers {
    get: String
}

fn main() {

    let filename = "./xml_source/maplinked.xml";
    let maplinked: Maplinked = xmlToStruct(filename.to_string());
    let mut list_state_items = initDevice(&maplinked);

    // // println!("{:#?}", list_state_items[0].varname);
    let conn = crate::db_tools::create_db().unwrap();

    let cli_res: String = match cli_interface(&list_state_items, &conn){
        Ok(o) => o,
        Err(e) => panic!(e)
    };
    
    let uuid: &String = &cli_res;

    let broadcast_addr = String::from("10.7.255.255:19000");

    let (sender_for_info, receiver_for_info) = channel();
    let (sender_for_listener, receiver_for_listener) = channel();
    

    let infoPackage = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                        <info>\
                            <URL>{}</URL>\
                            <UUID>{}</UUID>\
                            <TTL>{}</TTL>\
                        </info>", "10.7.2.2", uuid, "30"); // захардкожено!!

    let closePackage = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                        <info>\
                            <UUID>{}</UUID>\
                            <CLOSE/>\
                        </info>", uuid);
    let getResult = format!("
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <root>\
            <device>\
            <deviceType>SMEC</deviceType>\
            <friendlyName>КУЭВ</friendlyName>\
            <company>ООО КОНТИНЕНТ</company>\
            <country>Россия</country>\
            <version>1.0</version>\
            <serialNumber>1234567890</serialNumber>\
            <UUID>{}</UUID>\
            <serviceList>\
            <service>\
            <serviceType>MAP</serviceType>\
            <serviceID>1</serviceID>\
            </service>\
            <service>\
            <serviceType>SIGNALS</serviceType>\
            <serviceID>2</serviceID>\
            </service>\
            <service>\
            <serviceType>CONFIG</serviceType>\
            <serviceID>3</serviceID>\
            </service>\
            <service>\
            <serviceType>WEB</serviceType>\
            <serviceID>WEB</serviceID>\
            </service>\
            </serviceList>\
            </device>\
            </root>\r\n", uuid);

    let setAnswers = SetAnswers{ 
        get: getResult.to_string()
    };

    let infoThread = thread::spawn( || { startInfoListener(receiver_for_info, sender_for_listener, infoPackage) } );
    let otherPackageThread = thread::spawn( || {listenerPackage(receiver_for_listener, setAnswers) });
    
    // // функция с лямбдой -- хендлер для отлова ctrl + c 
    // // отсылает пакет завершения и кидает сообщение потоку info с коммандой завершиться
    ctrlc::set_handler( move || {

        println!("received Ctrl+C!");
        let socket_ = initSocket("10.7.2.2:5556".to_string());
        socket_.set_broadcast(true).expect("set_broadcast call failed");
        socket_.send_to(&closePackage.as_bytes(), &broadcast_addr).unwrap();
        println!("exit!");
        match sender_for_info.send(true){
            // Ok(_) => std::process::exit(0),
            Ok(_) => {},
            Err(e) => panic!("{:?}",e)
        }

    }).unwrap();

    

    infoThread.join();
    otherPackageThread.join();
    

    /* инициализация таблицы, внесение всех параметров и заглушки в value */
    // crate::db_tools::init_data(&maplinked.items, &list_state_items, &conn);
    /* обновление значения у конкретного параметра */
    // crate::db_tools::update_value(&list_state_items[0].varname , 2.1, &conn)
}
