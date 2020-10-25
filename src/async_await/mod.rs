pub async fn print_hello_world() {
    for i in 0..50 {
        println!("Printing Hello World: {}", i);
    }
}

pub async fn make_api_call() {
    for i in 0..50 {
        println!("Making Api Call: {}", i);
    }
    let res = reqwest::get("https://math.iith.ac.in/data/seminars.json").await;
    let res = match res {
        Ok(res) => {    
            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());
            match res.text().await {
                Ok(text) => println!("Body:\n{}", text),
                Err(err) => panic!("Problem with the response: {:?}", err)
            }
        },
        Err(err) => panic!("Problem with the request: {:?}", err)
    };
}