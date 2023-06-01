fn main(){
 let aldohexose = Monosaccharide{
     carbons:6,
     classification:MonosaccharideClass::ALDEHYDE
 };
 let fructose = Monosaccharide{
     carbons:6,
     classification:MonosaccharideClass::KETONE
 };
}


struct Monosaccharide{
    carbons: u32,
    classification: MonosaccharideClass,
}

enum MonosaccharideClass{
    KETONE,
    ALDEHYDE
}
 