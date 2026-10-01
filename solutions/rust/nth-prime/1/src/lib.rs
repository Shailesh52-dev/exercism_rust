pub fn nth(n: u32) -> u32 {
    let mut index=0;
    let mut candidate=2;
    loop{
        let mut found_divisor=false;
         let mut divisor=2;
        while divisor*divisor<=candidate{
            if candidate%divisor==0{
                found_divisor=true;
                break;
            }
            divisor+=1;
        }
        if found_divisor==true{
        candidate+=1;
    }else if index==n{
    return candidate;
}else{
    index+=1;
    candidate+=1;
}
    }  
    }