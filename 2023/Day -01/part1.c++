#include<fstream>
#include<string>
#include<iostream>
using namespace std;

int main(){
    ifstream file("input.txt");
    string str;
    int sum = 0;

    if(file.is_open()){
        while(getline(file, str)){
            int n = str.size(), digit = 0;

            for(int i = 0; i < n; ++i){
                if(str[i] >= '0' && str[i] <= '9'){
                    digit = (str[i] - '0') * 10;
                    break;
                }
            }
            for(int i = n-1; i >= 0; --i){
                if(str[i] >= '0' && str[i] <= '9'){
                    digit += str[i]-'0';
                    break;
                }
            }

            sum += digit;
        }
    }

    file.close();

    cout << sum << endl;

    return 0;
}