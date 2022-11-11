pub mod back_of_house{
    use crate::back_of_house;

    pub struct Breakfast {
        pub toast:String,
        pub season_fruit:String,
    }
    impl Breakfast {
        pub fn summer(toast:&str) ->Breakfast{
            Breakfast { 
                toast: String::from(toast), 
                season_fruit: String::from("peaches"),
             }
        }
    }

    pub fn eat_at_root(){
        let mut meal=back_of_house::Breakfast::summer("Rye");
        meal.toast=String::from("wheat");
        println!("I'd like {} toast please,",meal.toast);
        println!("I'd like {} fruit please,",meal.season_fruit);

    }
}