# 006-jsrzx-hw-lesson6

## 完成作业

### 作业点１

![image-20200623231359509](imgs/image-20200623231359509.png)

### 作业点２

![image-20200623232620556](imgs/image-20200623232620556.png)

![image-20200623232750487](imgs/image-20200623232750487.png)

### 作业点３

![image-20200624111928690](imgs/image-20200624111928690.png)

![image-20200624112121376](imgs/image-20200624112121376.png)

##　功能测试

> 启动网络：
>
> ```bash
> $ ./target/release/node-template purge-chain dev
> Are you sure to remove "/home/jason/.local/share/node-template/chains/dev/db"? [y/N]: y
> "/home/jason/.local/share/node-template/chains/dev/db" removed.
> 
> $ ./target/release/node-template  --dev
> ```

### （1）使用Alice账号创建5只小猫

![image-20200624112447124](imgs/image-20200624112447124.png)

### （2）查看小猫总数

![image-20200624112602105](imgs/image-20200624112602105.png)

### （3）分别查看5只小猫链表关系

![image-20200624112925320](imgs/image-20200624112925320.png)

### （4）Alice转移给Bob一只不存在的小猫，将会报错

![image-20200624113014514](imgs/image-20200624113014514.png)

### （5）Alice转移属于自己编号为3的小猫给Bob

![image-20200624113137852](imgs/image-20200624113137852.png)

### （6）可见Alice编号已经查询不到编号为3的小猫，且链表索引已经做了重建

![image-20200624113214802](imgs/image-20200624113214802.png)

### （7）使用kitties-ui转移编号为2的小猫给Bob

![image-20200624113750451](imgs/image-20200624113750451.png)

### （8）再此查看Alice拥有小猫的链表索引变化

![image-20200624114028141](imgs/image-20200624114028141.png)

### （9）查看Bob拥有小猫的链表索引

![image-20200624114416145](imgs/image-20200624114416145.png)

### 符合预期，通过测试。
