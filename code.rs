fn is_leap(year:u32){
  //if year % 4 == 0
  if (year%4)==0{
    // if year % 100 == 0
    if(year%100)==0{
      //if year % 400 == 0 
    if(year%400)==0{
      //then it's a leap year
      print!("{} is a leap year", year);
    }
    //else not a leap year

    else{
      print!("{} is not a leap year", year);
    }
  }else{
      print!("{} is not a leap year", year);
    }
  }else{
      print!("{} is not a leap year", year);
    }
}




fn main() {

  //calling the function to check leap year
  is_leap(1900)
}
