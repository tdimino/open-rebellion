
int __thiscall FUN_0058d990(void *this,void *param_1)

{
  void *this_00;
  void *pvVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  bool bVar5;
  int local_38 [2];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650080;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0050a660(*(void **)((int)this + 0x10),1,param_1);
  bVar5 = iVar2 != 0;
  iVar2 = 0;
  if (*(int *)((int)this + 0x44) != 0) {
    FUN_0058d3f0(local_38,(void *)((int)this + 8));
    local_4 = 0;
    iVar3 = FUN_0058ca70((void *)((int)this + 8),local_38,pvVar1);
    iVar2 = local_30;
    if ((iVar3 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    local_4 = 0xffffffff;
    FUN_0058d430(local_38);
  }
  if (iVar2 == 0) {
    this_00 = (void *)((int)this + 8);
    FUN_0058d150(local_38,this_00);
    local_4 = 2;
    iVar2 = FUN_0058ca70(this_00,local_38,pvVar1);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    local_4 = 0xffffffff;
    *(int *)((int)this + 0x1c) = local_30;
    FUN_0058d190(local_38);
    FUN_0058d1c0(local_38,this_00);
    local_4 = 3;
    iVar2 = FUN_0058cba0(this_00,local_38,pvVar1);
    if ((iVar2 == 0) || (!bVar5)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
    *(int *)((int)this + 0x20) = local_30;
    local_4 = 0xffffffff;
    FUN_0058d200(local_38);
    *(uint *)((int)this + 0x24) = (uint)(*(int *)((int)this + 0x40) != 0);
  }
  else {
    iVar2 = FUN_0058ce70((void *)((int)this + 8),*(int **)((int)this + 0x10),pvVar1);
    if ((iVar2 == 0) || (!bVar5)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
    *(undefined4 *)((int)this + 0x2c) = 1;
    *(undefined4 *)((int)this + 0x28) = 1;
    FUN_0056f3d0(local_2c,*(undefined4 *)((int)this + 0x10),3);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar3 = FUN_0052bed0((int)local_2c);
      param_1 = (void *)(*(uint *)(iVar3 + 0x50) >> 5 & 1);
      if (param_1 == (void *)0x0) {
        param_1 = (void *)0x0;
        iVar3 = FUN_0055e9e0(&param_1);
        if ((iVar3 == 0) || (iVar2 == 0)) {
          iVar2 = 0;
        }
        else {
          iVar2 = 1;
        }
        if (param_1 != (void *)0x0) {
          piVar4 = (int *)FUN_0052bed0((int)local_2c);
          iVar3 = (**(code **)(*piVar4 + 0x2f0))(1,10,pvVar1);
          if ((iVar3 == 0) || (iVar2 == 0)) {
            iVar2 = 0;
          }
          else {
            iVar2 = 1;
          }
          iVar3 = FUN_0052bed0((int)local_2c);
          if ((*(uint *)(iVar3 + 0x50) >> 6 & 1) != 0) {
            piVar4 = (int *)FUN_0052bed0((int)local_2c);
            iVar3 = (**(code **)(*piVar4 + 0xac))(10,pvVar1);
            if ((iVar3 == 0) || (iVar2 == 0)) {
              iVar2 = 0;
            }
            else {
              iVar2 = 1;
            }
          }
        }
      }
      FUN_005130d0((int)local_2c);
    }
    *(undefined4 *)((int)this + 0x30) = 1;
    local_4 = 0xffffffff;
    FUN_0056f4d0(local_2c);
  }
  ExceptionList = local_c;
  return iVar2;
}

