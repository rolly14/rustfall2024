#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major 
{
    ComputerScience,
    ElectricalEnginering,
}
#[derive(Debug)]
struct Student
{
    name: String,
    grade: GradeLevel,
    major: Major,
}
impl Student
{
    fn new(name:String, grade:GradeLevel, major:Major) -> Self
    {
        Student{
        name:name,
        grade:grade,
        major:major,
        }
    }
    fn introduce_yourself(&self)
        {
            let grade = GradeLevel::Bachelor;
            let major = Major::ComputerScience;

            let grade_msg = match grade 
            {
                GradeLevel::Bachelor => "I am Bachelor",
                GradeLevel::Master => "I am a Master",
                GradeLevel::PhD => "I am PhD",
            };

            let major_msg = match major
            {
                Major::ComputerScience => "I am in Computer Science",
                Major::ElectricalEnginering => " I am in Electrical Enginering"
            };
            
            println!("My name is {} {} {}", self.name, grade_msg, major_msg);
        }

}
fn main() {
let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();


}