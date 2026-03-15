
bool __thiscall FUN_00528960(void *this,void *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  bool bVar3;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00644690;
  local_c = ExceptionList;
  bVar3 = true;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_2 = 0;
  iVar1 = FUN_00528100(this,0);
  if (iVar1 == 0) {
    iVar1 = FUN_00528100(this,1);
    if (iVar1 == 0) {
      local_34 = 0xa0;
      local_30 = 3;
      local_4 = 1;
      FUN_00520580(param_1,&local_34);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    else {
      local_34 = 0xa0;
      local_30 = 4;
      local_4 = 2;
      FUN_00520580(param_1,&local_34);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  else {
    iVar1 = FUN_0052bed0((int)this);
    bVar3 = iVar1 != 0;
    if (iVar1 != 0) {
      FUN_0052cda0(local_2c,iVar1);
      local_4 = 3;
      FUN_00513120((int)local_2c);
      iVar1 = FUN_0052bed0((int)local_2c);
      while (iVar1 != 0) {
        piVar2 = (int *)FUN_0052bed0((int)local_2c);
        iVar1 = FUN_0052bde0(this,piVar2);
        if ((iVar1 != 0) &&
           (iVar1 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar1 + 0x60) & 1) == 0)) {
          iVar1 = FUN_0052bed0((int)local_2c);
          iVar1 = FUN_00520b70(iVar1);
          iVar1 = FUN_0053e1b0(1,iVar1);
          *param_2 = *param_2 + iVar1;
        }
        FUN_005130d0((int)local_2c);
        iVar1 = FUN_0052bed0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_0052cec0(local_2c);
    }
  }
  ExceptionList = local_c;
  return bVar3;
}

