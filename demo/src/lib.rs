pub mod back_of_house{
    use crate::back_of_house;
    use std::collections::HashMap;

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

    pub fn hashmap_test(){
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"),10);
        scores.insert(String::from("Yellow"),50);
        let team_name = String::from("blue");
        let score=scores.get(&team_name);
        
        match score {
            Some(s)=>println!("{}",s),
            None=>println!("team not found"),
        };
        
        for(k,v) in &scores{
            println!("{}:{}",k,v);
        }

        let text ="hello world wonderful world";
        let mut map=HashMap::new();
        for world in text.split_whitespace(){
            let count=map.entry(world).or_insert(0);
            *count+=1;
        }
        println!("{:#?}",map);


    }
}