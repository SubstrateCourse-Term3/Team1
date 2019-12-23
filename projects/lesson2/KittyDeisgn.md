## 设计

### 1 数据结构
	struct KittyCat {
		No. : u64, //猫全局序号
		DNA: [u8;16], //128bit DNA
	}

### 2 存储定义
	TotalCountOfKitties : u64  //全局序号
	KittyMap : map u64=>KittyCat //猫序号到猫实例的map
	AccountKitties : map AccountID=>u64 //所有者ID名下有几只猫
	AccountKittyMap : map (AccountID, u64)=>u64 //所有者id，以及名下的猫序号对应KittyMap中猫的序号
	KittyAccountMap : map u64=>（AccountID, u64） //猫序号对应的账户，以及在账户中的序号
	sellKittyMap : map u64 => (bool, Balance) //是否正在卖,售价多少

### 3 可调用函数
	//No.为全局编号，由生成猫时候的TotalCountOfKitties决定
	NewKitty()//生成新猫
	BreedKitty(KittyCat, KittyCat) //生育新猫
	GetKitty(int No.)//获取对应序号的猫的数据
	TransferKitty(AccountID newOwner, int No.)//将自己的编号为No.的猫赠送给newOwner
	SellKitty(int No., Balance) //出售自己的编号为id的猫
	CancelSellKitty(int No.) //取消出售自己的编号为id的猫
	BuyKitty(int No.) //购买编号为id的猫

### 4 算法伪代码
#### 4.1生成新猫
	NewKitty() {
		id 为当前调用者AccountId
		获取当前random_seed，调用者id，TotalCountOfKitties生成随机数，获取128bit赋值给kittyCat.DNA
		kittyCat.No = TotalCountOfKitties
		KittyMap.insert(TotalCountOfKitties, kittyCat) //记录当前序号的猫
		let count = AccountKitties.get(id) 获取当前账户有几只猫
		AccountKitties.insert(id, count+1) 当前用户名下猫数量+1
		AccountKittyMap.insert((id, count), TotalCountOfKitties) //记录当前用户名下的第count只猫对应的序号是多少
		KittyAccountMap.insert(TotalCountOfKitties,（id,count）) //记录该序号的猫对应的owner，以及在其名下的序号
		TotalCountOfKitties++  //全局序号+1
	}

生育猫可以用两只猫的DNA+random_seed重新DNA

#### 4.2遍历猫
	获取TotalCountOfKitties
	while n < TotalCountOfKitties{
		KittyMap.get(n)
	}

#### 4.3遍历某用户的所有猫
	let total = AccountKitties.get(id) 获取当前账户有几只猫
	let n = 0
	//遍历当前所有者的猫的序号
	while n < total{
		int serialNo = AccountKittyMap.get((id, n)) //获取当前用户第n只猫的序号
		KittyMap.get(serialNo) 获取猫信息
	}

#### 4.4 赠送
	transferKitty(AccountID newOwner, int No.){
		//No.为全局编号，由生成猫时候的TotalCountOfKitties决定
		senderID //调用此函数的账户id
		(id,oldNo) = KittyAccountMap.get(No.)//获取当前No.对应的owner的AccountID以及序号oldNo
		判断当前调用者为猫的所有者id==senderID，如果不是则返回
		调用changeOwner(senderID,newOwner, No.)
	}

	changeOwner(AccountID oldOwner, AccountID newOwner, int No)
	{
		let countOld = AccountKitties.get(oldOwner) 获取当前账户有几只猫
		let countNew = AccountKitties.get(newOwner) 获取新owner有几只猫
		AccountKitties.get(oldOwner)-- //当前调用者猫数量减1
		AccountKitties.get(newOwner)++ //新的所有者猫数量+1
		KittyAccountMap.insert(No., （newOwner, countNew）) //更改新owner以及新owner下的序号
		sellKittyMap如果当前猫正在出售则标记当前猫不再继续出售
		如果赠送的不是最大序号那只猫，将最大序号那只猫移动到oldNo位置，保持连续性
		先获取序号最大猫在当前owner中的序号
		NoMax = AccountKittyMap.get(id, countOld-1)
		KittyAccountMap.insert(NoMax, (id, oldNo))//将最大序号那只猫移动到刚送人那只猫那个序号位置，这样遍历某用户的所有猫的时候还是连续的序号遍历即可
	}

#### 4.4 交易
	//No.为全局编号，由生成猫时候的TotalCountOfKitties决定
	//senderID 为调用此函数的账户id
	SellKitty(int No，Balance balance)
	{
		(id, _) = KittyAccountMap.get(No)
		id == senderID如果不等则返回
		sellKittyMap标记当前猫正在出售为true, 且标记售价
	}
	
	CancelSellKitty(int No)
	{
		sellKittyMap标记当前猫正在出售为false
	}

	BuyKitty(int No.){
		sellKittyMap判断No号猫是否在出售，并获取售价
		判断当前账户余额是否足够大于等于售价
		(oldOwner,oldNo) = KittyAccountMap.get(No.)
		oldOwner.Blannce+=售价
		senderID.Balance-=售价
		changeOwner(oldOwner, senderID, No)//转移所有权
	}
