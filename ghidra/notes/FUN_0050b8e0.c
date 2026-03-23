
undefined4 __thiscall FUN_0050b8e0(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  int local_34;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641bd8;
  local_c = ExceptionList;
  uVar6 = 0;
  uVar7 = 0;
  local_34 = 0;
  ExceptionList = &local_c;
  iVar2 = FUN_00509710(this,1);
  iVar3 = FUN_00509710(this,2);
  uVar5 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if ((uVar5 == 0) || (bVar1 = true, 3 < uVar5)) {
    bVar1 = false;
  }
  if (bVar1) {
    if ((iVar2 == 0) || (iVar3 == 0)) {
      local_34 = 0;
      uVar6 = 0;
    }
    else {
      uVar6 = 1;
      local_34 = 1;
    }
    if ((uVar6 == 0) &&
       (((*(byte *)((int)this + 0x88) & 1) != 0 ||
        (((byte)*(uint *)((int)this + 0x24) & 0xc0) != 0xc0)))) {
      FUN_004ffe70(local_2c,this,1);
      local_4 = 0;
      FUN_00513120((int)local_2c);
      for (; (local_10 != 0 && (uVar7 == 0)); uVar7 = (uint)((uVar7 >> 6 & 3) != (uVar5 >> 6 & 3)))
      {
        iVar2 = FUN_0052bed0((int)local_2c);
        uVar7 = *(uint *)(iVar2 + 0x24);
        uVar5 = *(uint *)((int)this + 0x24);
        FUN_005130d0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_004fff70(local_2c);
    }
    uVar5 = *(uint *)((int)this + 0x88);
    if ((uVar5 >> 5 & 1) == uVar7) {
      uVar6 = uVar5 >> 0xe & 1;
    }
    else {
      uVar6 = (uint)((uVar5 >> 0xc & 1) != uVar6);
    }
  }
  iVar2 = FUN_0050a5f0(this,uVar7,param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0050a900(this,local_34,param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0050a9e0(this,uVar6,param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  ExceptionList = local_c;
  return uVar4;
}

