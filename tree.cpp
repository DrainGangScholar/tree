#include <algorithm>
#include<iostream>
#include<filesystem>
#include<vector>
namespace fs=std::filesystem;
using std::cout,std::endl;
using std::string;
using std::vector;
using std::sort;

string inner[2]={"├── ", "│   "};
string final[2]={ "└── ", "    "};


size_t files=0;
size_t folders=0;

void tree(string directory,string prefix);

int  main(int argc, char*argv[]) {
    //current dir
    //tree()
    // string root= argc==1? ".":argv[1];
    tree(fs::current_path().string(),"");
    return 0;
}

auto compare=[](const fs::directory_entry &left,const fs::directory_entry &right)->bool
{
    return left.path().filename()<right.path().filename();
};

void tree(string directory,string prefix){
    vector<fs::directory_entry> entries;

    for(const auto &entry: fs::directory_iterator(directory)) {
        if(entry.path().filename().string()[0]!='.'){
            entries.emplace_back(entry);
        }
    }

    sort(entries.begin(), entries.end(),compare);

    int index=0;
    for(const auto &entry:entries){
        string pointers[2];
        if(index==entries.size()-1){
            pointers[0]=final[0];
            pointers[1]=final[1];
        }else{
            pointers[0]=inner[0];
            pointers[1]=inner[1];
        }
        cout<<prefix<<pointers[0]<<entry.path().filename().string()<<endl;
        index++;
        if(entry.is_regular_file()){
            files++;
        }else {
            folders++;
            tree(entry.path().string(),pointers[1]);
        }
    }
}
