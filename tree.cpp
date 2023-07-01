#include <algorithm>
#include<iostream>
#include<filesystem>
#include<vector>
namespace fs=std::filesystem;
using std::cout,std::endl;
using std::string;
using std::vector;
using std::sort;
int  main(int argc, char*argv[]) {
    //current dir
    //tree()
    string root= argc==1? ".":argv[1];

    return 0;
}

auto compare=[](const fs::directory_entry &left,const fs::directory_entry &right)->bool
{
    return left.path().filename()<right.path().filename();
};

void tree(string directory){
    vector<fs::directory_entry> entries;

    for(const auto &entry: fs::directory_iterator(directory)) {
        if(entry.path().filename().string()[0]!='.')
            entries.emplace_back(entry);
    }
    
    sort(entries.begin(), entries.end(),compare);

    for(const auto &entry:entries){
        cout<<entry.path().filename().string()<<endl;
    }
}
