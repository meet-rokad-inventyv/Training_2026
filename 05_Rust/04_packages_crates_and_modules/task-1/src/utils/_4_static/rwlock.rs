use std::sync::RwLock;

static TOTAL: RwLock<usize> = RwLock::new(0);
static GET_REQUESTS: RwLock<usize> = RwLock::new(0);
static POST_REQUESTS: RwLock<usize> = RwLock::new(0);
static DELETE_REQUESTS: RwLock<usize> = RwLock::new(0);

enum Request
    {
        Get{endpoint:String},
        Post{endpoint:String,payload_size:u32},
        Delete(u32),
    }

pub fn rwlock() 
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


    println!("Total number of request processed : {} ",TOTAL.read().unwrap());
    println!("Total GET request processed : {} ",GET_REQUESTS.read().unwrap());
    println!("Total POST request processed : {} ",POST_REQUESTS.read().unwrap());
    println!("Total DELETE request processed : {} ",DELETE_REQUESTS.read().unwrap());
}

fn handle_request(req:Request)->String
{
    {
        *TOTAL.write().unwrap()+=1;
    }

    match req{
        Request::Get{endpoint} => 
		{
			{
				*GET_REQUESTS.write().unwrap() += 1;
			}

            format!(
                "GET request received for endpoint '{}'.",
                endpoint
            )
        }

        Request::Post{endpoint,payload_size} => 
        {
			{
				*POST_REQUESTS.write().unwrap() += 1;
			}
             format!(
               "POST request to '{}' with payload size {} bytes.",
                endpoint, payload_size
            )
        }

        Request::Delete(id) => 
        {
			{
				*DELETE_REQUESTS.write().unwrap() += 1;
			}
             format!(
                 "DELETE request received for resource ID {}.",
                id
            )
        }
    }

}