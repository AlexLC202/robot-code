#include <iostream>
#include <string>
#include <cmath>
using namespace std;
//const 210.9 cm bottom of vision tape to the ground

double findHorizontalAngle(double horizontalDistance, double distance);
double findHorizontalDistance(double distance); 

//The main function is not the logic but a test!
//-------------------------------------This main is for testing--------------
int main() {
	double distance; //hypothetical value that is recieved from the limelights
	cout << "Input a distance from the limelight to the vision tape" << endl;
  cin >> distance;
  double horizontalDistance = findHorizontalDistance(distance);
	cout << "This is the horizontal distance (to the inner circle): " << horizontalDistance << endl;
	double horizontalAngle = findHorizontalAngle(horizontalDistance, distance);
	cout << "This is the horizontal angle (radians): " << horizontalAngle << endl;
  cout << "The horizontal angle in degrees: " << horizontalAngle * 180 / 3.141 << endl;

}
//---------------------------Can delete Only need bottom two functions----------

double findHorizontalDistance(double distance) {
	double groundToTape = 210.9;
	groundToTape = 210.9 * 210.9;
	distance = distance * distance;
	double horizontalDistance = sqrt(distance - groundToTape) + 73.02; //cm
	return horizontalDistance;
}

//This is the angle from the limelight to the 
double findHorizontalAngle(double horizontalDistance, double distance) {
  double theta = (horizontalDistance-73.02) / distance;
	theta = acos(theta);
	return theta;
}
