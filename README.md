## A demo project for replace BlueKing install agent scripts.

### 功能

1. 并发
2. 多操作系统适配
3. 尽可能减少之前的依赖工具
  - 通过``libc``调用``UNIX``接口 完成相关功能
  - 引入轻盈的``Creat``
  - 尽可能更多的使用``Std``库完成更多的功能
4. 完善单元测试，语言自带的``test``覆盖更多的场景


### 警戒

1. 关注编译文件大小，不应该超过``8MB``

### 测试重点
1. 各个操作系统兼容性
  - Linux
  - Windows
  - Solaris
  - Aix
    - ai6
	- aix7
  - Macos


#### 平台类型支持
1. AIX: [支持计划评估中](https://community.ibm.com/community/user/power/communities/community-home/digestviewer/viewthread?GroupId=6211&MessageKey=b60b78ac-d482-4565-9db9-75690cce06f8&CommunityKey=10c1d831-47ee-4d92-a138-b03f7896f7c9&ReturnUrl=%2Fcommunity%2Fuser%2Fpower%2Fcommunities%2Fcommunity-home%2Fdigestviewer%3FListKey%3Daf5415f8-d8d8-4ac4-b3cb-08a8da184054)