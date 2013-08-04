struct HouseData{
 size: float,
 cost: float
}
impl HouseData {
 fn new(size: float,
 cost: float) 
 -> HouseData {

 HouseData{size: size, cost: cost}
 }
}
fn calculate_derivative(theta0: float,
 theta1: float, 
 data_set: &[HouseData]) 
 -> (float,float) {

 let m = 1.0f/(data_set.len() as float);
 let derivative1 = data_set
 .iter()
 .fold(0.0f,
 |x,b: &HouseData|x + ((theta0 + theta1 * b.size) - b.cost));

 let derivative2 = data_set
 .iter()
 .fold(0.0f,
 |x,b: &HouseData|x + ((theta0 + theta1 * b.size) - b.cost)*b.size);

 (m*derivative1,m*derivative2)
}
fn gradient_descend(step_size: float,
 eps: float,
 mut theta0: float, 
 mut theta1: float, 
 data_set: &[HouseData]) 
 -> (float,float) {
 loop{
 let (x,y) = calculate_derivative(theta0,theta1,data_set);
 let temp_theta0 = theta0 - step_size *x;
 let temp_theta1 = theta1 - step_size *y;
 
 if((theta0.abs()-temp_theta0.abs()) <= eps 
 && (theta1.abs()-temp_theta1.abs()) <= eps){
 break;
 }
 theta0 = temp_theta0;
 theta1 = temp_theta1;
 }
 (theta0,theta1)
}
fn linear_regression(step_size:float,
 eps: float ,
 theta0: float, 
 theta1: float,size: float,
 data_set: &[HouseData]) 
 -> float {
 let (b,m) = gradient_descend(step_size,eps,theta0,theta1,data_set);
 m * size + b
}
fn main() {

 
 let data_set = ~[HouseData::new(10.0f,20.0f),
                 HouseData::new(20.0f,40.0f),
                 HouseData::new(30.0f,60.0f),
                 HouseData::new(40.0f,80.0f),
                 HouseData::new(50.0f,100.0f),
                 HouseData::new(60.0f,120.0f)];

 printfln!(linear_regression(0.00001f,//stepsize
 0.0000001f,//epsilon
 0.0f,// start theta0
 10.0f,// start theta1
 70.0f,// size
 data_set// dataset
 ));
 
 
}
