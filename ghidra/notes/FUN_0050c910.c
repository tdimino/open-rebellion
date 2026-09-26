
int __thiscall FUN_0050c910(void *this,void *param_1)

{
  uint uVar1;
  void *pvVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641db8;
  local_c = ExceptionList;
  local_30 = 1;
  if ((*(byte *)((int)this + 0x88) & 4) != 0) {
    uVar1 = *(uint *)((int)this + 0x24) >> 6 & 3;
    if (uVar1 == 1) {
      iVar5 = 1;
    }
    else if (uVar1 == 2) {
      iVar5 = 2;
    }
    else {
      iVar5 = 0;
    }
    if (iVar5 != 0) {
      ExceptionList = &local_c;
      pvVar2 = FUN_00504c40(local_2c,this,1);
      local_4 = 0;
      iVar3 = FUN_00513180((int)pvVar2);
      local_4 = 0xffffffff;
      FUN_00504d40(local_2c);
      uVar1 = *(uint *)((int)this + 0x24);
      iVar7 = 0;
      uVar4 = *(uint *)((int)this + 0x88) >> 0xb & 1;
      iVar6 = 1;
      iVar5 = FUN_00507270(this,iVar5);
      iVar5 = FUN_00559fb0(uVar1 >> 6 & 3,iVar5,uVar4,iVar6,iVar7,iVar3);
      if (-1 < iVar5) {
        local_30 = FUN_0050a4a0(this,0,param_1);
      }
    }
  }
  ExceptionList = local_c;
  return local_30;
}

