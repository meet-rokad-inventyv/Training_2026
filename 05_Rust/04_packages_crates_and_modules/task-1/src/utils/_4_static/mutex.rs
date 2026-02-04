use std::sync::Mutex;

static TOTAL: Mutex<usize> = Mutex::new(0);
static GET_REQUESTS: Mutex<usize> = Mutex::new(0);
static POST_REQUESTS: Mutex<usize> = Mutex::new(0);
static DELETE_REQUESTS: Mutex<usize> = Mutex::new(0);

enum Request
    {
        Get{endpoint:String},
        Post{endpoint:String,payload_size:u32},
        Delete(u32),
    }

pub fn mutex() 
{
   let requests = vec![
    Request::Get {
        endpoint:"/users".to_string(),
    },

    Request::Post {
        endpoint:"login".to_string(),
        payload_size: 512,
    },

    Request::Delete(10),
   ];

    for req in  requests {
        let message = handle_request(req);
        println!("{}",message);
    }


    println!("Total number of request processed : {} ",TOTAL.lock().unwrap());
    println!("Total GET request processed : {} ",GET_REQUESTS.lock().unwrap());
    println!("Total POST request processed : {} ",POST_REQUESTS.lock().unwrap());
    println!("Total DELETE request processed : {} ",DELETE_REQUESTS.lock().unwrap());
}

fn handle_request(req:Request)->String
{
    {
        *TOTAL.lock().unwrap()+=1;
    }

    match req{
        Request::Get{endpoint} => 
		{
			{
				*GET_REQUESTS.lock().unwrap() += 1;
			}

            format!(
                "GET request received for endpoint '{}'.",
                endpoint
            )
        }

        Request::Post{endpoint,payload_size} => 
        {
			{
				*POST_REQUESTS.lock().unwrap() += 1;
			}
             format!(
               "POST request to '{}' with payload size {} bytes.",
                endpoint, payload_size
            )
        }

        Request::Delete(id) => 
        {
			{
				*DELETE_REQUESTS.lock().unwrap() += 1;
			}
             format!(
                 "DELETE request received for resource ID {}.",
                id
            )
        }
    }

}