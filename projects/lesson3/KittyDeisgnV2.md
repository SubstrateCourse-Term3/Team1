### kitties_count溢出修复
	1、加入版本号类型为version:Vec<u8>,kitties_count改为u128类型
	2、刚开始赋值version.push(0);
	3、由版本号和序号共同map一只猫，map (Vec<u8>, u128)=>Kitty
	4、当前版本最大kitties_count溢出之后，version[0]++，kitties_count重新置0
	5、当version[0]溢出之后再执行version.push(0)插入一个新字符
	6、当前版本最大kitties_count溢出之后，version[1]++
	7、以此类推，缺点就是耗费很多存储空间

#### 继承基因伪代码
##### 继承部分
	最大继承64bit，只继承两只猫相同的部分
	将父母猫的DNA按位做异或，统计相同的位的个数记为N，进行继承
	如果N大于64则，N=64，全部继承到新生猫的DNA[0..N]

##### 基因突变部分
	剩下M=128-64位
	payload = sender_addr + transaction_index + kitties_count + timestamp + current_height
	first_M_bytes(hash(payload))继承到新生猫的DNA[N..128]

