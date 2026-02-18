use std::cell::RefCell;

struct Address {
    email_id: String,
    mobile_no: String,
}

struct ProfileData {
    full_name: String,
    years: u32,
    address: Address,
}

struct UserProfile {
    profile: RefCell<ProfileData>,
}

impl UserProfile {
    fn create(
        full_name: String,
        years: u32,
        email_id: String,
        mobile_no: String,
    ) -> Self {
        UserProfile {
            profile: RefCell::new(ProfileData {
                full_name,
                years,
                address: Address {
                    email_id,
                    mobile_no,
                },
            }),
        }
    }

    fn update_name(&self, name: String) {
        self.profile.borrow_mut().full_name = name;
        self.display();
    }

    fn update_age(&self, years: u32) {
        self.profile.borrow_mut().years = years;
        self.display();
    }

    fn update_email(&self, email: String) {
        self.profile.borrow_mut().address.email_id = email;
        self.display();
    }

    fn update_mobile(&self, mobile: String) {
        self.profile.borrow_mut().address.mobile_no = mobile;
        self.display();
    }

    fn replace_all(
        &self,
        name: String,
        years: u32,
        email: String,
        mobile: String,
    ) {
        *self.profile.borrow_mut() = ProfileData {
            full_name: name,
            years,
            address: Address {
                email_id: email,
                mobile_no: mobile,
            },
        };
        self.display();
    }

    fn display(&self) {
        let data = self.profile.borrow();

        println!("--- Snapshot A ---");
        println!("Name   : {}", data.full_name);
        println!("Age    : {}", data.years);
        println!("Email  : {}", data.address.email_id);
        println!("Mobile : {}", data.address.mobile_no);
        println!("------------------\n");

        println!("--- Snapshot B ---");
        println!("Name   : {}", data.full_name);
        println!("Age    : {}", data.years);
        println!("Email  : {}", data.address.email_id);
        println!("Mobile : {}", data.address.mobile_no);
        println!("------------------\n");
    }
}

fn main() {
    let user1 = UserProfile::create(
        "Meet Rokad".to_string(),
        22,
        "meet@example.com".to_string(),
        "9000000000".to_string(),
    );

    println!("Initial Profile:");
    user1.display();

    let user_ref = &user1;

    user_ref.update_name("Meet Rokad".to_string());
    user_ref.update_age(23);
    user_ref.update_email("meet.updated@example.com".to_string());
    user_ref.update_mobile("9888888888".to_string());

    println!("Final Profile:");
    user_ref.display();

    println!("Replacing entire profile:");
    user_ref.replace_all(
        "System User".to_string(),
        30,
        "system@example.com".to_string(),
        "1111111111".to_string(),
    );
}
