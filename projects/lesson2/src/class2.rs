//链上存储加密猫数据
//遍历猫
//DNA 128bit


//每个用户可以拥有零到多只猫
//猫只有一个主人 
//遍历用户拥有的所有猫




数据结构
struct Kitty {
    unit32 ID 
    string name
    uint128 DNA; 
    uint64 birthTime; // 出生时间
  
    uint32 moytherId; // 母亲的ID
    uint32 fatherId; // 父亲的ID

    uint16 generation; // 第几代
}

  
    
存储定义
/*** STORAGE ***/

    /// 保存所有的猫信息
    Kitty[] kitties;

    /// 所有猫的ID到owner的地址的映射
    mapping (uint128 => address) public kittyIndexToOwner;

   



链上函数
　　Search Catbyowner(Owner ID) return List
　　Sold cat (price, target)
　　Search catinfo(cat ID)
    gotDNA(cat iD)

   CreateKitty(
        
        uint32 _motherID， 
        uint32 _fatherId,  
        address _owner

        uint32 _generation, 
        uint128 _DNA, 
        unit32  ID
         
    )
       
     
    
算法代码

DNA   128bit

32个特征



每个特征四个字符，第一位为显性基因，决定样式。前端按照样式表去画出该猫特征

ABCD 
4*3*2*1=24种排列

初代猫的ＤＮＡ
　系统按照时间生成的随机数除以24得到的余数决定ABCD
  运行32次，生成所有特征　　
      



繁殖猫的ＤＮＡ
     概率
ABCD 50 25 15 10
BACD 50 25 15 10
繁殖后代的DNA 按照概率排序，
    A概率 75
    B概率 75
    C概率 30
    D概率 20
    AB概率相同时，系统生成两次随机数，分别对应A B
    取大数为先

















链上存储
tokens


外部函数





